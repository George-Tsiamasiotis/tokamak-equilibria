#[derive(thiserror::Error, Debug)]
pub enum EqError {
    /// Interpolation domain error.
    #[error("Interpolation domain error.")]
    DomainError(#[from] rsl_interpolation::DomainError),

    /// Error from [`tokamak_netcdf`].
    #[error("netCDF error: {0}")]
    NcError(#[from] tokamak_netcdf::NcError),
}
