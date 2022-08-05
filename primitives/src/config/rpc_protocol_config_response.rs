use super::ProtocolConfigView;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcProtocolConfigResponse {
    #[serde(flatten)]
    pub config_view: ProtocolConfigView,
}
