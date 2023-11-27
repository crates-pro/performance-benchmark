use std::{fmt::Display, str::FromStr};

#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ArgEnum, serde::Deserialize, serde::Serialize,
)]
#[clap(rename_all = "PascalCase")]
pub enum Profile {
    Check,
    Debug,
    Doc,
    Release,
}

impl FromStr for Profile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" => Ok(Self::Check),
            "debug" => Ok(Self::Debug),
            "doc" => Ok(Self::Doc),
            "release" => Ok(Self::Release),
            _ => Err(format!("Unknown Profile {}", s)),
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Profile::Check => f.write_str("check"),
            Profile::Debug => f.write_str("debug"),
            Profile::Doc => f.write_str("doc"),
            Profile::Release => f.write_str("release"),
        }
    }
}

#[derive(Debug)]
pub struct Profiles {
    pub profiles: Vec<Profile>,
}

impl FromStr for Profiles {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut profiles = vec![];
        for profile in s.split(',') {
            profiles.push(Profile::from_str(profile)?);
        }

        Ok(Self { profiles })
    }
}
