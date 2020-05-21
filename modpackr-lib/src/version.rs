use std::fmt;

use crate::errors::VersionError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: Option<u64>,
    is_snapshot: bool,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(patch) = self.patch {
            write!(f, "{}.{}.{}", self.major, self.minor, patch)?;
        } else {
            write!(f, "{}.{}", self.major, self.minor)?;
        }

        if self.is_snapshot {
            write!(f, "-snapshot")?;
        }

        Ok(())
    }
}

impl Version {
    pub fn compatible_with(&self, other: Version) -> bool {
        let patch_compatibility = match (self.patch, other.patch) {
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => true,
            (Some(x), Some(y)) => x == y,
        };

        if self.is_snapshot {
            unimplemented!()
        }

        if other.is_snapshot {
            return false;
        }

        (self.major, self.minor) == (other.major, other.minor) && patch_compatibility
    }

    pub fn parse(mut input: &str) -> Result<Self, VersionError> {
        let is_snapshot = input.ends_with("-snapshot");

        if is_snapshot {
            input = input.trim_end_matches("-snapshot");
        }

        let numbers: Vec<&str> = input.split('.').collect();

        if numbers.len() > 3 {
            Err(VersionError::InvalidVersionString {
                input: input.to_string(),
            })
        } else {
            let major =
                numbers[0]
                    .parse::<u64>()
                    .map_err(|_| VersionError::InvalidVersionString {
                        input: input.to_string(),
                    })?;
            let minor =
                numbers[1]
                    .parse::<u64>()
                    .map_err(|_| VersionError::InvalidVersionString {
                        input: input.to_string(),
                    })?;

            let patch =
                if numbers.len() == 3 {
                    Some(numbers[2].parse::<u64>().map_err(|_| {
                        VersionError::InvalidVersionString {
                            input: input.to_string(),
                        }
                    })?)
                } else {
                    None
                };

            Ok(Version {
                major,
                minor,
                patch,
                is_snapshot,
            })
        }
    }
}
