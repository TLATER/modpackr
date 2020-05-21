use std::fmt;

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::errors::{ModError, VersionError};
use crate::r#mod::Mod;
use crate::version::Version;

#[derive(Deserialize)]
pub struct ForgeMod {
    pub name: String,
    pub id: usize,
    pub file_id: Option<usize>,
    pub hash: Option<String>,
}

impl Mod for ForgeMod {
    type DownloadHandle = usize;

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_latest_stable_for_version(
        &self,
        _version: Version,
    ) -> Result<Option<Self::DownloadHandle>, ModError> {
        unimplemented!()
    }

    fn get_latest_for_version(
        &self,
        version: Version,
    ) -> Result<Option<Self::DownloadHandle>, ModError> {
        let url = format!(
            "https://addons-ecs.forgesvc.net/api/v2/addon/{}/files",
            self.id
        );

        let files: Vec<File> = reqwest::blocking::get(url.as_str())?.json()?;

        Ok(files
            .iter()
            .filter(|file| {
                file.game_version
                    .iter()
                    .any(|game_version| version.compatible_with(*game_version))
            })
            .max_by_key(|file| file.id)
            .map(|file| file.id))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct File {
    // display_name: String,
    id: usize,
    #[serde(deserialize_with = "deserialize_versions")]
    game_version: Vec<Version>,
    // release_type: ReleaseType,
}

fn parse_game_version(input: &str) -> Result<Version, VersionError> {
    Version::parse(input)
}

fn deserialize_versions<'de, D>(d: D) -> Result<Vec<Version>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Helper;

    impl<'de> Visitor<'de> for Helper {
        type Value = Vec<Version>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "valid versions")
        }

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            while let Some(element) = visitor.next_element()? {
                // "Fabric" and "Forge" are not part of a game version
                // in our version model
                if element != "Fabric" && element != "Forge" {
                    let element: &str = element;

                    vec.push(
                        parse_game_version(&element.to_lowercase())
                            .map_err(serde::de::Error::custom)?,
                    );
                }
            }

            Ok(vec)
        }
    }

    d.deserialize_seq(Helper)
}

#[derive(Deserialize, PartialEq)]
enum ReleaseType {
    Stable = 1,
    Beta = 2,
    Alpha = 3,
}
