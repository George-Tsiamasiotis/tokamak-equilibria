use std::path::PathBuf;

use ndarray::concatenate;
use ndarray::{Array2, Axis};
use rsl_interpolation::{Accelerator, DynSpline2d};

use crate::Bfield;
use crate::Result;

/// Magnetic field reconstructed from a netCDF file.
#[allow(dead_code)]
pub struct Numerical {
    /// Spline over the magnetic field strength data.
    b_spline: DynSpline2d<f64>,
    /// The magnetic field data used to construct the spline.
    b_data: Array2<f64>,
}

impl Numerical {
    /// Constructs a [`Bfield`] from a netCDF file at `path`, with spline of `typ` interpolation type.
    ///
    ///
    /// # Example
    /// ```no_run
    /// # use tokamak_equilibria::*;
    /// # use std::path::PathBuf;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from("./data.nc");
    /// let bfield = bfield::Numerical::from_dataset(&path, "bicubic")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_dataset(path: &PathBuf, typ: &str) -> Result<Self> {
        use rsl_interpolation::*;
        use tokamak_netcdf::variable_names::*;
        use tokamak_netcdf::*;

        let eq = Equilibrium::from_file(path)?;

        // Add 0.0 manualy, which corresponds to the axis value.
        let psi_data = extract_var_with_axis_value(&eq.file, PSI_COORD, 0.0)?
            .as_standard_layout()
            .to_vec();
        let theta_data = eq.get_1d(THETA_COORD)?.to_vec();

        let b_data = eq.get_2d(B_FIELD)?;

        // Transpose of gcmotion
        let b_axis_values = Array2::from_elem((1, b_data.ncols()), 1.0); // B0 = 1 [NU]
        let b_data = concatenate![Axis(0), b_axis_values, b_data]; // e.g. [101, 3620]
        let b_data_flat = b_data.flatten().to_vec();

        let b_spline = make_spline2d(typ, &psi_data, &theta_data, &b_data_flat)?;

        Ok(Self { b_spline, b_data })
    }
}

impl Bfield for Numerical {
    fn b(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        Ok(self.b_spline.eval(psi, theta, xacc, yacc)?)
    }

    fn db_dtheta(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        // Ok(self.db_dtheta_spline.eval(psi, theta, xacc, yacc)?)
        Ok(self.b_spline.eval_deriv_y(psi, theta, xacc, yacc)?)
    }

    fn db_dpsi(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        // Ok(self.db_dpsi_spline.eval(psi, theta, xacc, yacc)?)
        Ok(self.b_spline.eval_deriv_x(psi, theta, xacc, yacc)?)
    }

    fn d2b_dpsi2(
        &self,
        psi: f64,
        theta: f64,
        xacc: &mut Accelerator,
        yacc: &mut Accelerator,
    ) -> Result<f64> {
        // Ok(self.d2b_dpsi2_spline.eval(psi, theta, xacc, yacc)?)
        Ok(self.b_spline.eval_deriv_xx(psi, theta, xacc, yacc)?)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::bfield::Numerical;

    #[test]
    /// Returns early if no .nc file is found.
    /// Specific b-values alues cross-tested with gcmotion.
    fn test_numeric_bfield_indices() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");
        match tokamak_netcdf::Equilibrium::from_file(&path) {
            Ok(_) => (),
            Err(_) => return,
        };

        let bf = Numerical::from_dataset(&path, "Bicubic").unwrap();
        let b = bf.b_data;

        assert_eq!(b.shape(), [101, 3620]);

        assert!(b.row(0).iter().all(|x| *x == 1.0));
        assert_eq!(b[[100, 3619]], 0.6756920402998402);
        assert_eq!(b[[50, 2000]], 1.342405803626943);
        assert_eq!(b[[80, 20]], 0.707178368383484);
    }
}
