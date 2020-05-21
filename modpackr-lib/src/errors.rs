use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ModError {
    #[snafu(display("Could not access mod: {}", error))]
    HttpError { error: reqwest::Error },
}

impl From<reqwest::Error> for ModError {
    fn from(error: reqwest::Error) -> ModError {
        ModError::HttpError { error }
    }
}

#[derive(Debug, Snafu)]
pub enum ManifestError {}

#[derive(Debug, Snafu)]
pub enum VersionError {
    #[snafu(display("'{}' is not a valid version string", input))]
    InvalidVersionString { input: String },
}
