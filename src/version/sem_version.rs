use std::{cmp::Ordering, str::FromStr};

use crate::error::AuError;

#[derive(Debug)]
pub struct SemVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SemVersion {
    /// Add `v` prefix like `v1.8.0`
    pub fn to_tag_name(&self) -> String {
        format!("v{}", self.to_string())
    }
}

impl ToString for SemVersion {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for SemVersion {
    type Err = AuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.chars().skip_while(|c| !c.is_ascii_digit()).collect();
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(AuError::StdError(String::from("invalid number of parts")));
        }

        let major = parts[0].parse()?;
        let minor = parts[1].parse()?;
        let patch = parts[2].parse()?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl PartialEq for SemVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for SemVersion {}

impl PartialOrd for SemVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major != other.major {
            self.major.cmp(&other.major)
        } else if self.minor != other.minor {
            self.minor.cmp(&other.minor)
        } else {
            self.patch.cmp(&other.patch)
        }
    }
}
