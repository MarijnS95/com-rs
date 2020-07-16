use interface::{ianimal::IAnimal, icat::ICat, idomesticanimal::IDomesticAnimal};

use com::co_class;
use com::sys::{HRESULT, NOERROR};

co_class! {
    /// The implementation class
    /// https://en.wikipedia.org/wiki/British_Shorthair
    pub coclass BritishShortHairCat: IDomesticAnimal, ICat, IAnimal {
        num_owners: u32,
    }

    // impl IDomesticAnimal for BritishShortHairCat {
    //     unsafe fn train(&self) -> HRESULT {
    //         println!("Training...");
    //         NOERROR
    //     }
    // }

    // impl ICat for BritishShortHairCat {
    //     unsafe fn ignore_humans(&self) -> HRESULT {
    //         println!("Ignoring Humans...");
    //         NOERROR
    //     }
    // }

    // impl IAnimal for BritishShortHairCat {
    //     unsafe fn eat(&self) -> HRESULT {
    //         println!("Eating...");
    //         NOERROR
    //     }
    // }
}

// impl BritishShortHairCat {
//     pub(crate) fn new() -> Box<BritishShortHairCat> {
//         BritishShortHairCat::allocate(20)
//     }
// }
