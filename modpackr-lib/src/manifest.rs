use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::forge_mod::ForgeMod;
use crate::version::Version;

fn deserialize_version<'de, D>(d: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    struct Helper;

    impl<'de> Visitor<'de> for Helper {
        type Value = Version;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "valid version")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Version::parse(value).map_err(serde::de::Error::custom)
        }
    }

    d.deserialize_str(Helper)
}

#[derive(Deserialize)]
pub struct Manifest {
    pub minecraft: MinecraftSettings,
    pub mods: Option<ModSettings>,
}

#[derive(Deserialize)]
pub struct MinecraftSettings {
    #[serde(deserialize_with = "deserialize_version")]
    pub version: Version,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSettings {
    pub curse_forge: Vec<ForgeMod>,
}

#[cfg(test)]
mod test {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn parse_example() {
        let test: Manifest = from_str(
            r#"
minecraft:
    version: "1.12.2"

mods:
    curseForge:
        - # Recipes
          name: jei
          id: 238222
"#,
        )
        .unwrap();

        assert_eq!(format!("{}", test.minecraft.version), "1.12.2");

        let mods = test.mods.unwrap();
        assert_eq!(mods.curse_forge[0].name, "jei");
        assert_eq!(mods.curse_forge[0].id, 238_222);
    }
}
