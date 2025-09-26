use rsl_interpolation::Accelerator;

use crate::Result;
use crate::efield::Efield;

pub struct NoEfield;

/// No electric field
///
/// Exists for compatibility reasons.
impl NoEfield {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Efield for NoEfield {
    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn phi(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> crate::Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(0.0)
    }

    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn e(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> crate::Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(0.0)
    }

    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn dphi_dpsi(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> crate::Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(0.0)
    }

    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn dphi_dtheta(
        &self,
        psi: f64,
        theta: f64,
        xacc: Option<&mut Accelerator>,
        yacc: Option<&mut Accelerator>,
    ) -> crate::Result<f64> {
        debug_assert!(psi.is_sign_positive());
        Ok(0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_no_efield() {
        let efield = efield::NoEfield::new().unwrap();

        assert_eq!(efield.phi(0.0, 0.0, None, None).unwrap(), 0.0);
        assert_eq!(efield.e(0.0, 0.0, None, None).unwrap(), 0.0);
        assert_eq!(efield.dphi_dpsi(0.0, 0.0, None, None).unwrap(), 0.0);
        assert_eq!(efield.dphi_dtheta(0.0, 0.0, None, None).unwrap(), 0.0);
    }
}
