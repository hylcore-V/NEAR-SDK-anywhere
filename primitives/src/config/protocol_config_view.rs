use super::{ProtocolConfig, RuntimeConfig};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolConfigView {
    pub runtime_config: RuntimeConfig,
}

impl From<ProtocolConfig> for ProtocolConfigView {
    fn from(protocol_config: ProtocolConfig) -> Self {
        let ProtocolConfig { runtime_config } = protocol_config;

        ProtocolConfigView { runtime_config }
    }
}
