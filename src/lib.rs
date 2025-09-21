#![doc = include_str!("../README.md")]
//! # Example
//!
//! ```
//! # use tokamak_equilibria::*;
//! # use std::f64::consts::PI;
//! #
//! # fn main() -> Result<()> {
//! // Construction of an analytical equilibrium
//! let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
//! let bfield = bfield::Lar::new()?;
//! let efield = efield::NoEfield::new()?;
//! let eq = Equilibrium::from_analytical(qfactor, bfield, efield)?;
//!
//! // Evaluation of electromagnetic field and q-factor inside the tokamak.
//! let q = eq.qfactor.q(0.01)?;
//! let b = eq.bfield.b(0.01, PI)?;
//! let phi = eq.efield.phi(0.01, PI)?;
//! # Ok(())
//! # }
//! ```
mod equilibrium;
mod error;

pub mod bfield;
pub mod efield;
pub mod qfactor;

pub use error::EqError;

#[doc(inline)]
pub use equilibrium::Equilibrium;

#[doc(inline)]
pub use bfield::*;
#[doc(inline)]
pub use efield::*;
#[doc(inline)]
pub use qfactor::*;

pub type Result<T> = std::result::Result<T, EqError>;
