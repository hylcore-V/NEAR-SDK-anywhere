#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Version {
    pub version: String,
    pub build: String,
    #[serde(default)]
    pub rustc_version: String,
}
