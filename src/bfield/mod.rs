//! Various magnetic field profiles.

use crate::Result;

mod lar;

pub use lar::*;

/// Calculation of magnetic field related quantities.
pub trait Bfield {
    /// Calculates `B(ψ, θ)`,
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let bfield = bfield::Lar::new()?;
    /// let b =  bfield.b(0.015, 2.0*PI)?;
    /// # Ok(())
    /// # }
    /// ```
    fn b(&self, psi: f64, theta: f64) -> Result<f64>;

    /// Calculates `𝜕B /𝜕𝜃`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let bfield = bfield::Lar::new()?;
    /// let b =  bfield.db_dtheta(0.015, 2.0*PI)?;
    /// # Ok(())
    /// # }
    /// ```
    fn db_dtheta(&self, psi: f64, theta: f64) -> Result<f64>;

    /// Calculates `𝜕B /𝜕ψ`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let bfield = bfield::Lar::new()?;
    /// let b =  bfield.db_dpsi(0.015, 2.0*PI)?;
    /// # Ok(())
    /// # }
    /// ```
    fn db_dpsi(&self, psi: f64, theta: f64) -> Result<f64>;

    /// Calculates `𝜕²B /𝜕𝜓²`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let bfield = bfield::Lar::new()?;
    /// let b =  bfield.d2b_dpsi2(0.015, 2.0*PI)?;
    /// # Ok(())
    /// # }
    /// ```
    fn d2b_dpsi2(&self, psi: f64, theta: f64) -> Result<f64>;
}
