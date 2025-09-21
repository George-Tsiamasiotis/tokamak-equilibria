use std::path::PathBuf;

use crate::Bfield;
use crate::Efield;
use crate::Qfactor;
use crate::Result;

/// Representation of a Tokamak Equilibrium.
///
/// Contains all the information of the magnetic field, electric field and q-factor.
#[non_exhaustive]
pub struct Equilibrium<Q, B, E>
where
    Q: Qfactor,
    B: Bfield,
    E: Efield,
{
    /// The equilibrium's [`q-factor`](Qfactor).
    pub qfactor: Q,
    /// The equilibrium's [`magnetic field`](Bfield).
    pub bfield: B,
    /// The equilibrium's [`electric field`](Efield).
    pub efield: E,
}

impl<Q, B, E> Equilibrium<Q, B, E>
where
    Q: Qfactor,
    B: Bfield,
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
    /// let efield = efield::NoEfield::new()?;
    /// let eq = Equilibrium::from_analytical(qfactor, bfield, efield)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_analytical(qfactor: Q, bfield: B, efield: E) -> Result<Self> {
        Ok(Self {
            qfactor,
            bfield,
            efield,
        })
    }

    /// Constructs an Equilibrium with data from a netCDF file.
    pub fn from_dataset(path: &PathBuf) -> Result<Self> {
        let _file = tokamak_netcdf::Equilibrium::from_file(path)?;
        todo!()
    }
}
