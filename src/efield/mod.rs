//! Various electric field profiles.

use rsl_interpolation::Accelerator;

use crate::Result;

mod nofield;

pub use nofield::*;

/// Calculation of electric field related quantities.
pub trait Efield {
    // TODO: add examples.

    /// Calculates `Φ(ψ, θ)`.
    fn phi(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> Result<f64>;

    /// Calculates `E(ψ, θ)`.
    fn e(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> Result<f64>;

    /// Calculates `𝜕𝛷 /𝜕𝜓`.
    fn dphi_dpsi(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> Result<f64>;

    /// Calculates `𝜕²𝛷 /𝜕𝜓²`.
    fn dphi_dtheta(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> Result<f64>;
}
