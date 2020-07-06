mod co_class;
#[doc(hidden)]
pub mod offset;
#[doc(hidden)]
#[cfg(windows)]
pub mod registration;

pub use co_class::CoClass;
pub use co_class::ProductionComInterface;
