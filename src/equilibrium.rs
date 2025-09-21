use std::path::PathBuf;

use crate::Result;
use crate::bfield::Bfield;
use crate::current::Current;
use crate::efield::Efield;
use crate::qfactor::Qfactor;

/// Representation of a Tokamak Equilibrium.
///
/// Contains all the information of the magnetic field, electric field and q-factor.
#[non_exhaustive]
pub struct Equilibrium<Q, B, C, E>
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

impl<Q, B, C, E> Equilibrium<Q, B, C, E>
where
    Q: Qfactor,
    B: Bfield,
    C: Current,
    E: Efield,
{
    /// Constructs an Equilibrium from analytical [`crate::qfactor`], [`crate::bfield`] and
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
    /// let eq = Equilibrium::from_analytical(qfactor, bfield, current, efield)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_analytical(qfactor: Q, bfield: B, current: C, efield: E) -> Result<Self> {
        Ok(Self {
            qfactor,
            bfield,
            current,
            efield,
        })
    }

    /// Constructs an Equilibrium with data from a netCDF file.
    pub fn from_dataset(path: &PathBuf) -> Result<Self> {
        let _file = tokamak_netcdf::Equilibrium::from_file(path)?;
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_analytical_eq() {
        let qfactor = qfactor::Unity::new().unwrap();
        let bfield = bfield::Lar::new().unwrap();
        let current = current::Lar::new().unwrap();
        let efield = efield::NoEfield::new().unwrap();

        let eq = Equilibrium::from_analytical(qfactor, bfield, current, efield).unwrap();

        eq.bfield.b(0.01, 3.14).unwrap();
        eq.efield.phi(0.01, 3.14).unwrap();
        eq.current.i(0.01).unwrap();
        eq.qfactor.q(0.01).unwrap();
    }
}
