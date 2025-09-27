//! Various plasma current profiles.

use rsl_interpolation::Accelerator;

use crate::Result;

mod lar;
mod numerical;

pub use lar::Lar;
pub use numerical::Numerical;

/// Calculation of plasma current related quantities.
pub trait Current {
    /// Calculates `I(ψ, θ)`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let cur = current::Lar::new()?;
    ///
    /// let i = cur.i(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn i(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;

    /// Calculates `g(ψ, θ)`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let cur = current::Lar::new()?;
    ///
    /// let g = cur.g(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    fn g(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;

    /// Calculates `𝜕I(ψ, θ)/𝜕ψ`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let cur = current::Lar::new()?;
    ///
    /// let i_der = cur.i_der(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Current derivatives are calculated with respect to `ψ`, and not `𝜓ₚ`, which appears in the
    /// guiding center equations of motion. To get the derivatives with respect to `𝜓ₚ`, we can
    /// simply multiply with `q(ψ)`.
    fn i_der(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;

    /// Calculates `𝜕g(ψ, θ)/𝜕ψ`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// # use rsl_interpolation::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut acc = Accelerator::new();
    /// let cur = current::Lar::new()?;
    ///
    /// let g_der = cur.g_der(0.015, &mut acc)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Current derivatives are calculated with respect to `ψ`, and not `𝜓ₚ`, which appears in the
    /// guiding center equations of motion. To get the derivatives with respect to `𝜓ₚ`, we can
    /// simply multiply with `q(ψ)`.
    fn g_der(&self, psi: f64, acc: &mut Accelerator) -> Result<f64>;
}
