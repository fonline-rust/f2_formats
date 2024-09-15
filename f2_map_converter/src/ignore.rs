use std::path::Path;

use crate::ValidationError;

pub fn fallout2(path: &Path, res: &Result<&str, ValidationError>) -> bool {
    matches!(res, Err(ValidationError::NonNullTail))
        && (path.ends_with("NewR1a.map") || path.ends_with("NewR2a.map"))
}

pub fn sonora(_path: &Path, _res: &Result<&str, ValidationError>) -> bool {
    false
}

pub fn nevada(path: &Path, _res: &Result<&str, ValidationError>) -> bool {
    path.components().any(|comp| comp.as_os_str() == "delete")
}

pub fn olympus(path: &Path, res: &Result<&str, ValidationError>) -> bool {
    path.ends_with("02test.MAP")
        || (path.ends_with("rbfabric.MAP") && matches!(res, Err(ValidationError::NonNullTail)))
}
