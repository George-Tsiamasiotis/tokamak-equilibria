use crate::Result;
use crate::current::Current;

/// Representation of Large Aspect Ratio plasma currents.
pub struct Lar;

impl Lar {
    /// Crates a new Large Aspect Ration current profile.
    ///
    /// LAR plasma currents are always `I = 0` and `g = 1`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let cur = current::Lar::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl Current for Lar {
    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn i(&self, psi: f64) -> Result<f64> {
        Ok(0.0)
    }

    /// Always returns `1.0`.
    #[allow(unused_variables)]
    fn g(&self, psi: f64) -> Result<f64> {
        Ok(1.0)
    }

    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn i_der(&self, psi: f64) -> Result<f64> {
        Ok(0.0)
    }

    /// Always returns `0.0`.
    #[allow(unused_variables)]
    fn g_der(&self, psi: f64) -> Result<f64> {
        Ok(0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_lar_current() {
        let current = current::Lar::new().unwrap();

        assert_eq!(current.i(0.0).unwrap(), 0.0);
        assert_eq!(current.g(0.0).unwrap(), 1.0);
        assert_eq!(current.i_der(0.0).unwrap(), 0.0);
        assert_eq!(current.g_der(0.0).unwrap(), 0.0);
    }
}
