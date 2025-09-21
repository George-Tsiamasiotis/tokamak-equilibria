//! Various plasma current profiles.

use crate::Result;

mod lar;

pub use lar::*;

/// Calculation of plasma current related quantities.
pub trait Current {
    /// Calculates `I(ψ, θ)`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let cur = current::Lar::new()?;
    /// let i = cur.i(0.015)?;
    /// # Ok(())
    /// # }
    /// ```
    fn i(&self, psi: f64) -> Result<f64>;

    /// Calculates `g(ψ, θ)`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let cur = current::Lar::new()?;
    /// let g = cur.g(0.015)?;
    /// # Ok(())
    /// # }
    /// ```
    fn g(&self, psi: f64) -> Result<f64>;

    /// Calculates `𝜕I(ψ, θ)/𝜕ψ`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let cur = current::Lar::new()?;
    /// let i_der = cur.i_der(0.015)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Current derivatives are calculated with respect to `ψ`, and not `𝜓ₚ`, which appears in the
    /// guiding center equations of motion. To get the derivatives with respect to `𝜓ₚ`, we can
    /// simply multiply with `q(ψ)`.
    fn i_der(&self, psi: f64) -> Result<f64>;

    /// Calculates `𝜕g(ψ, θ)/𝜕ψ`
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let cur = current::Lar::new()?;
    /// let g_der = cur.g_der(0.015)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Current derivatives are calculated with respect to `ψ`, and not `𝜓ₚ`, which appears in the
    /// guiding center equations of motion. To get the derivatives with respect to `𝜓ₚ`, we can
    /// simply multiply with `q(ψ)`.
    fn g_der(&self, psi: f64) -> Result<f64>;
}
