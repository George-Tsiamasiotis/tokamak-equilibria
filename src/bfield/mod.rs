//! Various magnetic field profiles.

use rsl_interpolation::Accelerator;

use crate::Result;

mod lar;
mod numerical;

pub use lar::*;
pub use numerical::Numerical;

/// Calculation of magnetic field related quantities.
pub trait Bfield {
    /// Calculates `B(ψ, θ)`,
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut psi_acc = Accelerator::new();
    /// let mut theta_acc = Accelerator::new();
    /// let bfield = bfield::Lar::new()?;
    ///
    /// let b =  bfield.b(0.015, 2.0*PI, &mut psi_acc, &mut theta_acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn b(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64>;

    /// Calculates `𝜕B /𝜕𝜃`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut psi_acc = Accelerator::new();
    /// let mut theta_acc = Accelerator::new();
    /// let bfield = bfield::Lar::new()?;
    ///
    /// let db_dtheta =  bfield.db_dtheta(0.015, 2.0*PI, &mut psi_acc, &mut theta_acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn db_dtheta(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64>;

    /// Calculates `𝜕B /𝜕ψ`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut psi_acc = Accelerator::new();
    /// let mut theta_acc = Accelerator::new();
    /// let bfield = bfield::Lar::new()?;
    ///
    /// let db_dpsi =  bfield.db_dpsi(0.015, 2.0*PI, &mut psi_acc, &mut theta_acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn db_dpsi(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64>;

    /// Calculates `𝜕²B /𝜕𝜓²`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// # use std::f64::consts::PI;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut psi_acc = Accelerator::new();
    /// let mut theta_acc = Accelerator::new();
    /// let bfield = bfield::Lar::new()?;
    ///
    /// let d2b_dpsi2 =  bfield.d2b_dpsi2(0.015, 2.0*PI, &mut psi_acc, &mut theta_acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn d2b_dpsi2(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64>;
}
