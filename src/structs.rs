#[cfg(feature = "python")]
use pyo3::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathActionType {
    AbsolutePath(()),
    RelativePath(()),
    SubFolderPath(()),
    BackFolderPath(()),
    AllFilesFolder(()),
    SymlinkPath(()),    
    MatchStrPath(String),
    VectorStrPath(Vec<String>),
    LiteralCharPath(char),
    AnyCharPath(()),
    NextCharPath(()),
    EscapeCharPath(()),
    Done,
}

#[cfg(feature = "python")]
#[pyclass(name = "FileInfo")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileInfo {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub fullname: String,

    #[pyo3(get, set)]
    pub info: String,

    #[pyo3(get, set)]
    pub symlink: String,
}

#[cfg(not(feature = "python"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileInfo {
    pub name: String,
    pub fullname: String,
    pub info: String,
    pub symlink: String,
}
