#[derive(Debug)]
pub enum ArtconverterError {
    IoError(std::io::Error),
    SliceConversion(std::array::TryFromSliceError),
    BitmapSaveError(String),
    BitmapPixelError(String),
    BitmapError(bmp_rust::bmp::ErrorKind)
}

impl From<std::array::TryFromSliceError> for ArtconverterError {
    fn from(error: std::array::TryFromSliceError) -> Self {
        ArtconverterError::SliceConversion(error)
    }
}

impl From<std::io::Error> for ArtconverterError {
    fn from(error: std::io::Error) -> Self {
        ArtconverterError::IoError(error)
    }
}

impl From<String> for ArtconverterError {
    fn from(error: String) -> Self {
        ArtconverterError::BitmapSaveError(error)
    }
}

impl From<&str> for ArtconverterError {
    fn from(error: &str) -> Self {
        ArtconverterError::BitmapPixelError(error.to_string())
    }
}

impl From<bmp_rust::bmp::ErrorKind> for ArtconverterError {
    fn from(error: bmp_rust::bmp::ErrorKind) -> Self {
        ArtconverterError::BitmapError(error)
    }
}
