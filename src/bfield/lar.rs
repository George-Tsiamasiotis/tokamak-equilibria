use crate::{Bfield, Result};

/// Representation of Large Aspect Ratio magnetic field.
pub struct Lar;

impl Lar {
    /// Crates a new Large Aspect Ration magnetic field profile.
    ///
    /// LAR magnetic field is defined as `B(ψ, θ) = 1 − √(2𝜓)⋅cos𝜃`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let bfield = bfield::Lar::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl Bfield for Lar {
    /// Returns `1 − √(2𝜓)⋅cos𝜃`
    fn b(&self, psi: f64, theta: f64) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(1.0 - (2.0 * psi).sqrt() * theta.cos())
    }

    /// Returns `√(2𝜓)⋅sin𝜃`
    fn db_dtheta(&self, psi: f64, theta: f64) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok((2.0 * psi).sqrt() * theta.sin())
    }

    /// Returns `-cosθ/√(2𝜓)`
    fn db_dpsi(&self, psi: f64, theta: f64) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(-theta.cos() / (2.0 * psi).sqrt())
    }

    /// Returns `-cosθ/(2*𝜓)³ᐟ²`
    fn d2b_dpsi2(&self, psi: f64, theta: f64) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(theta.cos() / (2.0 * psi.sqrt()).powf(3.0 / 2.0))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    /// Values cross-tested with gcmotion.
    fn test_lar() {
        let bfield = Lar::new().unwrap();

        assert_eq!(bfield.b(0.01, 1.0).unwrap(), 0.9235897151259821);
        assert_eq!(bfield.db_dpsi(0.01, 1.0).unwrap(), -3.820514243700898);
        assert_eq!(bfield.db_dtheta(0.01, 1.0).unwrap(), 0.11900196790587718);
    }
}
