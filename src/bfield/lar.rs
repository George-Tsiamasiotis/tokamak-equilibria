use rsl_interpolation::Accelerator;

use crate::Result;
use crate::bfield::Bfield;

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
    #[allow(unused_variables)]
    fn b(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(1.0 - (2.0 * psi).sqrt() * theta.cos())
    }

    /// Returns `√(2𝜓)⋅sin𝜃`
    #[allow(unused_variables)]
    fn db_dtheta(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok((2.0 * psi).sqrt() * theta.sin())
    }

    /// Returns `-cosθ/√(2𝜓)`
    #[allow(unused_variables)]
    fn db_dpsi(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(-theta.cos() / (2.0 * psi).sqrt())
    }

    /// Returns `-cosθ/(2*𝜓)³ᐟ²`
    #[allow(unused_variables)]
    fn d2b_dpsi2(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(theta.cos() / (2.0 * psi.sqrt()).powf(3.0 / 2.0))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use rsl_interpolation::*;

    #[test]
    /// Values cross-tested with gcmotion.
    fn test_lar() {
        let mut psi_acc = Accelerator::new();
        let mut theta_acc = Accelerator::new();
        let bfield = bfield::Lar::new().unwrap();

        assert_eq!(
            bfield.b(0.01, 1.0, &mut psi_acc, &mut theta_acc).unwrap(),
            0.9235897151259821
        );
        assert_eq!(
            bfield
                .db_dpsi(0.01, 1.0, &mut psi_acc, &mut theta_acc)
                .unwrap(),
            -3.820514243700898
        );
        assert_eq!(
            bfield
                .db_dtheta(0.01, 1.0, &mut psi_acc, &mut theta_acc)
                .unwrap(),
            0.11900196790587718
        );
    }
}
