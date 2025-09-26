//! Various q-factor profiles.

use rsl_interpolation::Accelerator;

use crate::Result;

mod parabolic;
mod unity;

pub use parabolic::Parabolic;
pub use unity::Unity;

/// Calculation of q-factor related quantities.
pub trait Qfactor {
    /// Calculates the q-factor `q(ψ)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    /// let q =  qfactor.q(0.015, None)?;
    /// # Ok(())
    /// # }
    /// ```
    fn q(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64>;

    /// Calculates the poloidal flux `𝜓ₚ(𝜓)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    /// let q =  qfactor.psip(0.015, None)?;
    /// # Ok(())
    /// # }
    /// ```
    fn psip(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64>;
}
