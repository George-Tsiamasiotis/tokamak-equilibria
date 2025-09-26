#[derive(thiserror::Error, Debug)]
pub enum EqError {
    /// Interpolation domain error.
    #[error("Interpolation domain error.")]
    DomainError(#[from] rsl_interpolation::DomainError),

    /// Error from [`tokamak_netcdf`].
    #[error("netCDF error: {0}")]
    NcError(#[from] tokamak_netcdf::NcError),

    /// Error creating Spline.
    #[error("Error creating Spline: {0}")]
    SplineError(#[from] rsl_interpolation::InterpolationError),

    /// Spline evaluation called without Accelerator.
    #[error("Spline evaluation called without Accelerator.")]
    AccError,
}
