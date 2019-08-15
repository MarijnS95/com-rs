use com::{ComInterface, ComPtr, IClassFactoryMethods, IUnknownMethods, IUnknown,};

use winapi::shared::guiddef::IID;

pub const IID_ICAT_CLASS: IID = IID {
    Data1: 0xf5353c58,
    Data2: 0xcfd9,
    Data3: 0x4204,
    Data4: [0x8d, 0x92, 0xd2, 0x74, 0xc7, 0x57, 0x8b, 0x53],
};

pub trait ICatClass: IUnknown {}

unsafe impl ComInterface for ICatClass {
    const IID: IID = IID_ICAT_CLASS;
}

pub type ICatClassVPtr = *const ICatClassVTable;

impl <T: ICatClass + ComInterface + ?Sized> ICatClass for ComPtr<T> {}

#[repr(C)]
pub struct ICatClassVTable(
    pub IUnknownMethods,
    pub IClassFactoryMethods,
    pub ICatClassMethods,
);

#[allow(non_snake_case)]
#[repr(C)]
pub struct ICatClassMethods {}


