use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Warping from `std::path::PathBuf`
#[derive(Debug, Clone)]
pub struct RPath {
    path_buf: PathBuf,
}

/// UnWarp the tuple sub object
// TODO: if it's necessary, it should have a marco to create plenty of Self-designed-tuple-struct
pub trait IntoInner {
    type Output;
    /// Unwarp from tuple type
    fn into_inner(self) -> Self::Output;
}
impl IntoInner for (RPath, Option<String>) {
    type Output = RPath;
    fn into_inner(self) -> Self::Output {
        self.0
    }
}

impl RPath {
    /// Create RPath from current project path
    pub fn from_project_path() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        Self { path_buf: root }
    }
    /// Append the add_path to the RPath
    pub fn append(mut self, add_path: &str) -> Self {
        self.path_buf.push(add_path);
        self
    }
    /// Coerces to a `std::path::Path` slice.
    pub fn as_path(&self) -> &Path {
        self.path_buf.as_path()
    }
    /// Set file name
    ///
    /// If it has name already it will return the pre name <Option.String>
    ///
    /// else return the None
    ///
    /// then push the file name to the back
    pub fn set_file_name(mut self, name: &str) -> (RPath, Option<String>) {
        let res = if self.path_buf.file_name().is_none() {
            None
        } else {
            self.path_buf
                .file_name()
                .map(|s| s.to_os_string().into_string().unwrap())
        };
        self.path_buf.set_file_name(name);
        (self, res)
    }
    /// Set the file extension
    ///
    /// Return `Error` when the `file_name` is None
    ///
    /// Return `Ok(Self)` when the `file_name` is exists
    pub fn set_extension(mut self, extensions: &str) -> Result<Self> {
        if self.path_buf.set_extension(extensions) {
            Ok(self)
        } else {
            Err(anyhow!("Error in set_extension() for RPath"))
        }
    }
    /// Get the `file_name` from the RPath
    ///
    /// Return `None` if it doesn't exist
    ///
    /// Return `Some(String)`  if it exists
    pub fn get_file_name(&self) -> Option<String> {
        self.path_buf
            .file_name()
            .map(|s| s.to_os_string().into_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_from_project_path_should_ok() {}
    #[test]
    fn test_append_should_ok() {}
    #[test]
    fn test_set_file_name_should_ok() {}
}
