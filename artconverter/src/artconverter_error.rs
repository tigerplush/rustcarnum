#[derive(Debug)]
pub enum ArtconverterError {
    IoError(std::io::Error),
    SliceConversion(std::array::TryFromSliceError),
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
