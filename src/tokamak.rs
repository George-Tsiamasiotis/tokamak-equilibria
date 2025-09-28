use crate::Result;
use crate::bfield::Bfield;
use crate::current::Current;
use crate::efield::Efield;
use crate::qfactor::Qfactor;

/// Representation of a Tokamak Equilibrium.
///
/// Contains all the information of the magnetic field, electric field and q-factor.
#[non_exhaustive]
pub struct Tokamak<Q, B, C, E>
where
    Q: Qfactor,
    B: Bfield,
    C: Current,
    E: Efield,
{
    /// The equilibrium's [`q-factor`](Qfactor).
    pub qfactor: Q,
    /// The equilibrium's [`magnetic field`](Bfield).
    pub bfield: B,
    /// The equilibrium's [`currents`](Current).
    pub current: C,
    /// The equilibrium's [`electric field`](Efield).
    pub efield: E,
}

impl<Q, B, C, E> Tokamak<Q, B, C, E>
where
    Q: Qfactor,
    B: Bfield,
    C: Current,
    E: Efield,
{
    /// Constructs a `Tokamak` from analytical [`crate::qfactor`], [`crate::bfield`] and
    /// [`crate::efield`].
    ///
    /// # Example
    ///
    /// ```
    /// # use tokamak_equilibria::*;
    /// #
    /// # fn main() -> Result<()> {
    /// let qfactor = qfactor::Parabolic::new(1.1, 3.9, 0.125)?;
    /// let bfield = bfield::Lar::new()?;
    /// let current = current::Lar::new()?;
    /// let efield = efield::NoEfield::new()?;
    ///
    /// let eq = Tokamak::build(qfactor, bfield, current, efield)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(qfactor: Q, bfield: B, current: C, efield: E) -> Result<Self> {
        Ok(Self {
            qfactor,
            bfield,
            current,
            efield,
        })
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use rsl_interpolation::Accelerator;

    use crate::*;

    #[test]
    fn test_analytical_tokamak() {
        let qfactor = qfactor::Unity::new().unwrap();
        let bfield = bfield::Lar::new().unwrap();
        let current = current::Lar::new().unwrap();
        let efield = efield::NoEfield::new().unwrap();

        let mut psi_acc = Accelerator::new();
        let mut theta_acc = Accelerator::new();
        let eq = Tokamak::build(qfactor, bfield, current, efield).unwrap();

        eq.bfield
            .b(0.01, 3.14, &mut psi_acc, &mut theta_acc)
            .unwrap();
        eq.efield
            .phi(0.01, 3.14, &mut psi_acc, &mut theta_acc)
            .unwrap();
        eq.current.i(0.01, &mut psi_acc).unwrap();
        eq.qfactor.q(0.01, &mut psi_acc).unwrap();
    }

    #[test]
    #[ignore = "needs specific dataset"]
    fn test_numerical_tokamak() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");
        let typ = "Cubic";
        let typ2d = "Bicubic";

        let qfactor = crate::qfactor::Numerical::from_dataset(&path, typ).unwrap();
        let bfield = crate::bfield::Numerical::from_dataset(&path, typ2d).unwrap();
        let current = crate::current::Numerical::from_dataset(&path, typ).unwrap();
        let efield = crate::efield::NoEfield::new().unwrap();

        let _t = Tokamak {
            qfactor,
            bfield,
            current,
            efield,
        };
    }
}
