#![doc = include_str!("../README.md")]
//! # Example - Analytical Equilibrium
//!
//! ```
//! # use tokamak_equilibria::*;
//! # use rsl_interpolation::*;
//! # use std::f64::consts::PI;
//! #
//! # fn main() -> Result<()> {
//! let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
//! let bfield = bfield::Lar::new()?;
//! let current = current::Lar::new()?;
//! let efield = efield::NoEfield::new()?;
//!
//! let tokamak = Tokamak::build(qfactor, bfield, current, efield)?;
//!
//! // Evaluation of electromagnetic field and q-factor inside the tokamak.
//! let mut psi_acc = Accelerator::new();
//! let mut theta_acc = Accelerator::new();
//!
//! let q = tokamak.qfactor.q(0.01, &mut psi_acc)?;
//! let b = tokamak.bfield.b(0.01, PI, &mut psi_acc, &mut theta_acc)?;
//! let i = tokamak.current.i(0.01, &mut psi_acc)?;
//! let phi = tokamak.efield.phi(0.01, PI, &mut psi_acc, &mut theta_acc)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Note
//!
//! Even in analytical equilibria objects, the accelerators must be passed as parameters as well,
//! even though they are not used. Until I find a better solution...
//!
//! # Example - Numerical Equilibrium
//!
//! ```no_run
//! # use tokamak_equilibria::*;
//! # use rsl_interpolation::*;
//! # use std::f64::consts::PI;
//! # use std::path::PathBuf;
//! #
//! # fn main() -> Result<()> {
//! let path = PathBuf::from("./data.nc");
//! let qfactor = qfactor::Numerical::from_dataset(&path, "cubic")?;
//! let current = current::Numerical::from_dataset(&path, "cubic")?;
//! let bfield = bfield::Numerical::from_dataset(&path, "bicubic")?;
//! let efield = efield::NoEfield::new()?;
//!
//! let tokamak = Tokamak::build(qfactor, bfield, current, efield)?;
//!
//! // Evaluation of electromagnetic field and q-factor inside the tokamak.
//! let mut psi_acc = Accelerator::new();
//! let mut theta_acc = Accelerator::new();
//!
//! let q = tokamak.qfactor.q(0.01, &mut psi_acc)?;
//! let b = tokamak.bfield.b(0.01, PI, &mut psi_acc, &mut theta_acc)?;
//! let i = tokamak.current.i(0.01, &mut psi_acc)?;
//! let phi = tokamak.efield.phi(0.01, PI, &mut psi_acc, &mut theta_acc)?;
//! # Ok(())
//! # }
//! ```
mod error;
mod tokamak;

pub mod bfield;
pub mod current;
pub mod efield;
pub mod qfactor;

pub use error::EqError;

#[doc(inline)]
pub use tokamak::Tokamak;

#[doc(inline)]
pub use bfield::Bfield;
#[doc(inline)]
pub use current::Current;
#[doc(inline)]
pub use efield::Efield;
#[doc(inline)]
pub use qfactor::Qfactor;

pub type Result<T> = std::result::Result<T, EqError>;
