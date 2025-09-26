use std::path::PathBuf;

use rsl_interpolation::{Accelerator, DynSpline};

use crate::EqError;
use crate::Qfactor;
use crate::Result;

/// q-factor reconstructed from a netCDF file.
pub struct Numerical {
    /// Spline over the q-factor data.
    pub q_spline: DynSpline<f64>,
    /// Spline over the 𝜓ₚ values data.
    pub psip_spline: DynSpline<f64>,
    /// The calculated 𝜓ₚ(ψ) values, for all ψ ∈ `psi_data`.
    pub psip_data: Box<[f64]>,
}

impl Numerical {
    /// Constructs a [`Qfactor`] from a netCDF file at `path`, with spline of `typ` interpolation type.
    ///
    /// # Note
    ///
    /// The value `ψ = 0.0` is prepended at the ψ data array, and the first value of the q array is
    /// prepended (duplicated) in the q array, to assure correct interpolation near the magnetic axis.
    ///
    /// # Example
    /// ```no_run
    /// # use tokamak_equilibria::*;
    /// # use std::path::PathBuf;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from("./data.nc");
    /// let qfactor = qfactor::Numerical::from_dataset(&path, "cubic")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_dataset(path: &PathBuf, typ: &str) -> Result<Self> {
        use rsl_interpolation::*;
        use tokamak_netcdf::variable_names::*;
        use tokamak_netcdf::*;

        let eq = Equilibrium::from_file(path)?;

        // Add 0.0 manualy, which corresponds to q0.
        let psi_data = extract_var_with_axis_value(&eq.file, PSI_COORD, 0.0)?
            .as_standard_layout()
            .to_vec();
        // Manually add q0 to the array.
        let q_data = extract_var_with_first_axis_value(&eq.file, Q_FACTOR)?
            .as_standard_layout()
            .to_vec();

        let q_spline = make_spline(typ, &psi_data, &q_data)?;

        let mut psip_data: Vec<f64> = Vec::with_capacity(psi_data.len());
        let mut acc = Accelerator::new();
        for psi in psi_data.iter() {
            let psip = q_spline.eval_integ(0.0, *psi, &mut acc)?;
            psip_data.push(psip);
        }

        let psip_spline = make_spline(typ, &psi_data, &psip_data)?;

        Ok(Self {
            q_spline,
            psip_spline,
            psip_data: psip_data.into(),
        })
    }
}

impl Qfactor for Numerical {
    fn q(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64> {
        match acc {
            Some(acc) => Ok(self.q_spline.eval(psi, acc)?),
            None => Err(EqError::AccError),
        }
    }

    fn psip(&self, psi: f64, acc: Option<&mut Accelerator>) -> Result<f64> {
        match acc {
            Some(acc) => Ok(self.psip_spline.eval(psi, acc)?),
            None => Err(EqError::AccError),
        }
    }
}

#[cfg(test)]
mod test {
    use is_close::is_close;
    use std::path::PathBuf;

    use rsl_interpolation::Accelerator;

    use crate::qfactor::Numerical;
    use crate::*;

    /// Returns early if no .nc file is found.
    /// Values cross-tested with gcmotion.
    #[test]
    fn test_numeric_qfactor() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");
        match tokamak_netcdf::Equilibrium::from_file(&path) {
            Ok(_) => (),
            Err(_) => return,
        };

        let mut accelerator = Accelerator::new();
        let acc = &mut accelerator;
        let qf = Numerical::from_dataset(&path, "Linear").unwrap();

        dbg!(qf.q(0.0, Some(acc)).unwrap());
        assert_eq!(qf.q(0.0, Some(acc)).unwrap(), 0.9164152189670636); // inserted value
        // Use a relatively high relative tolerance, since the splines are not exactly the same.
        assert!(is_close!(
            qf.q(0.1, Some(acc)).unwrap(),
            1.9514842302135769,
            rel_tol = 1e-4
        ));
        assert!(is_close!(
            // wall value
            qf.q(0.19889475414290547, Some(acc)).unwrap(),
            5.996391839022671,
            rel_tol = 1e-9
        ));
    }
}
