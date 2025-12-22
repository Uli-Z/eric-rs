use anyhow::Context;
use std::{
    ffi::{CString, OsStr},
    path::{Path, PathBuf},
};

pub trait ToCString {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error>;
}

impl ToCString for String {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error> {
        CString::new(self).context("Can't convert to CString")
    }
}

impl ToCString for &str {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error> {
        CString::new(self).context("Can't convert to CString")
    }
}

impl ToCString for PathBuf {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error> {
        self.to_str()
            .context("Can't convert path to CString")?
            .try_to_cstring()
    }
}

impl ToCString for &Path {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error> {
        self.to_str()
            .context("Can't convert path to CString")?
            .try_to_cstring()
    }
}

impl ToCString for &OsStr {
    fn try_to_cstring(self) -> Result<CString, anyhow::Error> {
        self.to_str()
            .context("Can't convert OsStr to CString")?
            .try_to_cstring()
    }
}
