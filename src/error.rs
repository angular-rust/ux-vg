use std::error::Error;
use std::ffi::NulError;
use std::fmt::{
    self,
    Display,
    Formatter,
};
use std::io;

/// Enum with all possible canvas errors that could occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Unknown error
    UnknownError,
    /// General error
    GeneralError(String),
    /// Image error
    #[cfg(feature = "image-loading")]
    ImageError(::image::ImageError),
    /// IO error
    IoError(io::Error),
    /// Font parse error
    FontParseError,
    /// Not found error
    NoFontFound,
    /// Font info extraction error
    FontInfoExtracionError,
    /// Font size too large for atlas error
    FontSizeTooLargeForAtlas,
    /// Shader compile error
    ShaderCompileError(String),
    /// Shader link error
    ShaderLinkError(String),
    /// Render target error
    RenderTargetError(String),
    /// Image Id not found error
    ImageIdNotFound,
    /// Image update out of bounds error
    ImageUpdateOutOfBounds,
    /// Image update with different format error
    ImageUpdateWithDifferentFormat,
    /// Unsuported image format error
    UnsuportedImageFromat,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "canvas error")
    }
}

#[cfg(feature = "image-loading")]
impl From<::image::ImageError> for ErrorKind {
    fn from(error: ::image::ImageError) -> Self {
        Self::ImageError(error)
    }
}

impl From<io::Error> for ErrorKind {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<NulError> for ErrorKind {
    fn from(error: NulError) -> Self {
        Self::GeneralError(error.to_string())
    }
}

impl Error for ErrorKind {}
