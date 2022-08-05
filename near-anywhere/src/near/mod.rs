mod auth_data;
mod connection;
mod errors;
mod near;
mod near_config;
mod wallet_connection;

pub use {
    auth_data::AuthData,
    connection::Connection,
    errors::ConnectionError,
    near::Near,
    near_config::NearConfig,
    wallet_connection::WalletConnection,
};

pub type WalletAccount = WalletConnection;
pub fn connect(config: NearConfig) -> Near {
    Near::new(config)
}
