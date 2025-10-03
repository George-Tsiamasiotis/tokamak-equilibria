mod error;

mod bfield;
mod current;
mod qfactor;

pub use error::EqError;

#[doc(inline)]
pub use bfield::Bfield;
#[doc(inline)]
pub use current::Current;
#[doc(inline)]
pub use qfactor::Qfactor;

pub type Result<T> = std::result::Result<T, EqError>;
