use crate::errors::ModError;
use crate::version::Version;

pub trait Mod {
    type DownloadHandle;

    fn get_name(&self) -> String;
    fn get_latest_stable_for_version(
        &self,
        version: Version,
    ) -> Result<Option<Self::DownloadHandle>, ModError>;
    fn get_latest_for_version(
        &self,
        version: Version,
    ) -> Result<Option<Self::DownloadHandle>, ModError>;
}
