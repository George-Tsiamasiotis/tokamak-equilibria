//! Various q-factor profiles.

use rsl_interpolation::Accelerator;

use crate::Result;

mod numerical;
mod parabolic;
mod unity;

pub use numerical::Numerical;
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
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    ///
    /// let q =  qfactor.q(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn q(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;

    /// Calculates the poloidal flux `𝜓ₚ(𝜓)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    ///
    /// let q =  qfactor.psip(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn psip(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;
}
