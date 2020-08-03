use leo_package::errors::ManifestError;

use std::ffi::OsString;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("main file {:?} does not exist", _0)]
    MainFileDoesNotExist(OsString),

    #[error("{}", _0)]
    ManifestError(#[from] ManifestError),
}
