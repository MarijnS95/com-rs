// use interface::{ianimal::IAnimal, icat::ICat, idomesticanimal::IDomesticAnimal, Food};

use com::sys::{E_INVALIDARG, HRESULT, NOERROR};
use com::{class, interfaces, interfaces::IUnknown, Interface};
use std::cell::Cell;
use std::ffi::c_void;
use std::ptr::NonNull;

use ::com::interfaces::IClassFactory;

interfaces! {
    #[uuid("14f486bf-408d-43be-8a34-bbfa56980a37")]
    pub(crate) unsafe interface IFood: IUnknown {
        fn consume_food(&self, amount: u32) -> HRESULT;
    }

    #[uuid("d17892d4-40d9-4323-b318-fa49ece56552")]
    pub(crate) unsafe interface IAnimal: IUnknown {
        fn consume_optional_food(&self, food: Option<IFood>) -> HRESULT;
        fn consume_mandatory_food(&self, food: IFood) -> HRESULT;
        fn consume_food_ptr(&self, food: *const IFood) -> HRESULT;
    }
}

class! {
    pub(crate) class Bowl: IFood {
        level: Cell<u32>,
    }

    impl IFood for Bowl {
        fn consume_food(&self, amount: u32) -> HRESULT {
            // TODO checked_sub
            if let Some(l) = self.level.get().checked_sub(amount){
                self.level.set(l);
                NOERROR
            } else {
                println!("Not enough food in bowl");
                E_INVALIDARG
            }
        }
    }
}

// Mimick an API generally exported by a dlopened library (ie. dxcompiler):
class! {
    pub(crate) class ExternalAnimal: IAnimal {
    }

    impl IAnimal for ExternalAnimal  {
        // fn consume_optional_food(&self, food: Option<IFood>) -> HRESULT {
        fn consume_optional_food(&self, food: *mut NonNull<<IFood as Interface>::VTable>) -> HRESULT {
            let object = food as *mut _ as *mut IFood;
            println!("consume_optional_food({:p} {:p}", self, food);
            if object.is_null() {
                E_INVALIDARG
            } else {
                unsafe { object.as_ref().unwrap().consume_food(1) }
            }
        }
        // fn consume_mandatory_food(&self, food: IFood) -> HRESULT {
        fn consume_mandatory_food(&self, food: NonNull<NonNull<<IFood as Interface>::VTable>>) -> HRESULT {
            let object = food.as_ptr() as *mut _ as *mut IFood;
            unsafe { object.as_ref().unwrap().consume_food(1) }
        }
        fn consume_food_ptr(&self, food: *const IFood) -> HRESULT {
            unsafe { food.as_ref().unwrap().consume_food(1) }
        }
    }
}

impl Default for Bowl {
    fn default() -> Self {
        Self::new(Cell::new(20))
    }
}

impl Default for ExternalAnimal {
    fn default() -> Self {
        Self::new()
    }
}

impl Bowl {
    pub fn get_interface<I: Interface>(&self) -> Option<I> {
        // pub fn get_interface<'a, I: Interface>(&self) -> Option<I> {
        use com::sys::{E_NOINTERFACE, E_POINTER, FAILED};
        use com::IID;
        let mut ppv = None;
        let hr = unsafe {
            self.query_interface(
                &I::IID as *const IID,
                &mut ppv as *mut _ as *mut *mut c_void,
            )
        };
        if FAILED(hr) {
            assert!(
                hr == E_NOINTERFACE || hr == E_POINTER,
                "QueryInterface returned non-standard error"
            );
            return None;
        }
        debug_assert!(ppv.is_some());
        ppv
    }

    pub fn get_interface_ref<'a, I: Interface>(&'a self) -> Option<&'a I> {
        // pub fn get_interface<'a, I: Interface>(&self) -> Option<I> {
        use com::sys::{E_NOINTERFACE, E_POINTER, FAILED};
        use com::IID;
        let mut ppv = None;
        let hr = unsafe {
            self.query_interface(
                &I::IID as *const IID,
                &mut ppv as *mut _ as *mut *mut c_void,
            )
        };
        if FAILED(hr) {
            assert!(
                hr == E_NOINTERFACE || hr == E_POINTER,
                "QueryInterface returned non-standard error"
            );
            return None;
        }
        debug_assert!(ppv.is_some());
        ppv
    }
}

fn main() {
    let b = Bowl::default();
    // unsafe { b.add_ref() };
    println!("Bowl@{:p}, refcnt = {}", &b, b.__refcnt.get());
    let food = b.get_interface_ref::<IFood>().unwrap();
    println!("IFood@{:p}", food);
    println!("bowl refcnt = {}", b.__refcnt.get());
}
