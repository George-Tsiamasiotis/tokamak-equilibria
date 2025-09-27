use rsl_interpolation::Accelerator;

use crate::Result;
use crate::qfactor::Qfactor;

/// q-factor of 1
///
/// Exists for compatibility reasons.
pub struct Unity;

impl Unity {
    /// Crates a new q-factor profile of q = `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let qfactor = qfactor::Unity::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl Qfactor for Unity {
    /// Always returns `1.0`.
    #[allow(unused_variables)]
    fn q(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(1.0)
    }

    /// Always returns `psi`.
    #[allow(unused_variables)]
    fn psip(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(psi)
    }
}

#[cfg(test)]
mod test {
    use rsl_interpolation::Accelerator;

    use crate::*;

    #[test]
    fn test_unity() {
        let mut acc = Accelerator::new();
        let qfactor = qfactor::Unity::new().unwrap();

        assert_eq!(qfactor.q(0.01, &mut acc).unwrap(), 1.0);
        assert_eq!(qfactor.psip(0.01, &mut acc).unwrap(), 0.01);
    }
}
