use {
    super::{errors::ConnectionError, NearConfig},
    crate::{client::Transport, signer::Signer},
};

#[derive(Clone)]
pub struct Connection {
    pub network_id: String,
    pub signer: Signer,
    pub jsvm_account_id: String,
    pub transport: Transport,
}

impl Connection {
    pub fn new(
        network_id: &str,
        transport: Transport,
        signer: Signer,
        jsvm_account_id: &str,
    ) -> Self {
        Self {
            network_id: network_id.to_string(),
            signer,
            transport,
            jsvm_account_id: jsvm_account_id.to_string(),
        }
    }
}

impl TryFrom<NearConfig> for Connection {
    type Error = ConnectionError;

    fn try_from(config: NearConfig) -> Result<Self, Self::Error> {
        let network_id = config.network_id.unwrap_or_else(|| "".to_string());
        let jsvm_account_id = config.jsvm_account_id.unwrap_or_else(|| "".to_string());
        let transport = match config.transport {
            Some(transport) => transport,
            _ => Err(Self::Error::InvalidParams {
                error_message: "Provider is required".to_string(),
            })?,
        };

        let signer = match config.signer {
            Some(signer) => signer,
            _ => Err(Self::Error::InvalidParams {
                error_message: "Signer is required".to_string(),
            })?,
        };

        Ok(Self {
            network_id,
            signer,
            transport,
            jsvm_account_id,
        })
    }
}
