use std::path::PathBuf;

use rsl_interpolation::{Accelerator, DynSpline};

use crate::Current;
use crate::Result;

/// Plasma current reconstructed from a netCDF file.
pub struct Numerical {
    /// Spline over the I-current data.
    pub i_spline: DynSpline<f64>,
    /// Spline over the g-current data.
    pub g_spline: DynSpline<f64>,
}

impl Numerical {
    /// Constructs a [`Current`] from a netCDF file at `path`, with spline of `typ` interpolation type.
    ///
    /// # Note
    ///
    /// The value `ψ = 0.0` is prepended at the ψ data array, and the first values of the i and g arrays
    /// is prepended (duplicated) in each array, to assure correct interpolation near the magnetic axis.
    ///
    /// # Example
    /// ```no_run
    /// # use tokamak_equilibria::*;
    /// # use std::path::PathBuf;
    /// #
    /// # fn main() -> Result<()> {
    /// let path = PathBuf::from("./data.nc");
    /// let cur = current::Numerical::from_dataset(&path, "cubic")?;
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
        let i_data = extract_var_with_first_axis_value(&eq.file, CURRENT_I)?
            .as_standard_layout()
            .to_vec();
        let g_data = extract_var_with_first_axis_value(&eq.file, CURRENT_G)?
            .as_standard_layout()
            .to_vec();

        let i_spline = make_spline(typ, &psi_data, &i_data)?;
        let g_spline = make_spline(typ, &psi_data, &g_data)?;

        Ok(Self { i_spline, g_spline })
    }
}

impl Current for Numerical {
    fn i(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        Ok(self.i_spline.eval(psi, acc)?)
    }

    fn g(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        Ok(self.g_spline.eval(psi, acc)?)
    }

    fn i_der(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        Ok(self.i_spline.eval_deriv(psi, acc)?)
    }

    fn g_der(&self, psi: f64, acc: &mut Accelerator) -> Result<f64> {
        Ok(self.g_spline.eval_deriv(psi, acc)?)
    }
}

#[cfg(test)]
mod test {
    use is_close::is_close;
    use std::path::PathBuf;

    use rsl_interpolation::Accelerator;

    use crate::current::Numerical;
    use crate::*;

    /// Values cross-tested with gcmotion.
    #[test]
    #[ignore = "needs specific dataset"]
    fn test_numeric_current_values() {
        let path = PathBuf::from("./reconstructed/smart_positive.nc");

        let mut acc = Accelerator::new();
        let cur = Numerical::from_dataset(&path, "Akima").unwrap();

        assert_eq!(cur.i(0.0, &mut acc).unwrap(), 0.0012294990364400897); // inserted value
        assert_eq!(cur.g(0.0, &mut acc).unwrap(), 0.9985398705655125); // inserted value
        // Use a relatively high relative tolerance, since the splines are not exactly the same.
        assert!(is_close!(
            cur.i(0.1, &mut acc).unwrap(),
            0.1433092088696332,
            rel_tol = 1e-4
        ));
        assert!(is_close!(
            cur.g(0.1, &mut acc).unwrap(),
            0.8575838128118375,
            rel_tol = 1e-4
        ));
        assert!(is_close!(
            cur.i(0.19889475414290547, &mut acc).unwrap(),
            0.17214836970426942,
            rel_tol = 1e-4
        ));
        assert!(is_close!(
            cur.g(0.19889475414290547, &mut acc).unwrap(),
            0.8416486417160426,
            rel_tol = 1e-4
        ));
    }
}
