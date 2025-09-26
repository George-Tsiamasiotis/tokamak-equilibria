use rsl_interpolation::Accelerator;

use crate::Result;
use crate::qfactor::Qfactor;

/// Parabolic q-factor.
///
/// `q` is given by the equation `q(𝜓) = q₀ + (q_w − q₀)(𝜓/𝜓_w)²`.
pub struct Parabolic {
    /// The q-factor value at the magnetic axis.
    pub q0: f64,
    /// the q-factor value at the wall.
    pub qwall: f64,
    /// The toroidal flux value at the wall.
    pub psi_wall: f64,
    /// Intermediate quantity to avoid recalculation. Is equal to `qwall - q0`.
    pub(crate) diff: f64,
    /// Intermediate quantity to avoid recalculation. Is equal to `sqrt(diff)`,
    pub(crate) sqrt_diff: f64,
    /// Intermediate quantity to avoid recalculation. Is equal to `sqrt(q0)*sqrt(diff)`.
    pub(crate) sqrt_prod: f64,
    /// Intermediate quantity to avoid recalculation. Is equal to `sqrt(q0)*sqrt(psi_wall)`.
    pub(crate) sqrt_q0psi_wall: f64,
}

impl Parabolic {
    /// Crates a new parabolic q-factor profile.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(q0: f64, qwall: f64, psi_wall: f64) -> Result<Self> {
        let diff = qwall - q0;
        let sqrt_q0 = q0.sqrt();

        Ok(Self {
            q0,
            qwall,
            psi_wall,
            diff,
            sqrt_diff: diff.sqrt(),
            sqrt_prod: sqrt_q0 * diff.sqrt(),
            sqrt_q0psi_wall: sqrt_q0 * psi_wall,
        })
    }
}

impl Qfactor for Parabolic {
    #[allow(unused_variables)]
    fn q(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(self.q0 + self.diff * (psi / self.psi_wall).powi(2))
    }

    #[allow(unused_variables)]
    fn psip(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        let atan = (self.sqrt_diff * psi / self.sqrt_q0psi_wall).atan();
        let psip = self.psi_wall / self.sqrt_prod * atan;
        Ok(psip)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use is_close::is_close;

    #[test]
    /// Values cross-tested with gcmotion.
    fn test_intermediate_values() {
        let q0 = 1.1;
        let qwall = 3.8;
        let psi_wall = 0.04591368227731865;
        let qfactor = qfactor::Parabolic::new(q0, qwall, psi_wall).unwrap();

        assert!(is_close!(qfactor.q(0.0, None).unwrap(), q0));
        assert!(is_close!(qfactor.q(0.01, None).unwrap(), 1.228079468));
        assert!(is_close!(
            qfactor.q(0.03, None).unwrap(),
            2.2527152119999996
        ));
        assert!(is_close!(qfactor.q(psi_wall, None).unwrap(), qwall));

        assert!(is_close!(qfactor.psip(0.0, None).unwrap(), 0.0));
        assert!(is_close!(
            qfactor.psip(0.01, None).unwrap(),
            0.00876084223156207
        ));
        assert!(is_close!(
            qfactor.psip(0.03, None).unwrap(),
            0.021236184655956582
        ));
        assert!(is_close!(
            qfactor.psip(psi_wall, None).unwrap(),
            0.026713778215136246
        ));
    }
}
