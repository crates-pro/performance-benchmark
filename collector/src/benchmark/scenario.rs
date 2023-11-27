#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ArgEnum, serde::Deserialize, serde::Serialize,
)]
#[clap(rename_all = "PascalCase")]
pub enum Scenario {
    Full,
    IncrFull,
    IncrUnchanged,
    IncrPatched,
}

impl Scenario {
    pub fn is_increment(self) -> bool {
        matches!(
            self,
            Scenario::IncrFull | Scenario::IncrUnchanged | Scenario::IncrPatched
        )
    }
}
