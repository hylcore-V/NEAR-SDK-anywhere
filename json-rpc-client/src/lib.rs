#![allow(clippy::integer_arithmetic)]
#[macro_use]
extern crate serde_derive;

mod rpc_client;
mod rpc_error;
mod rpc_request;
mod rpc_response;
mod transports;

pub use {
    rpc_client::RpcClient,
    rpc_error::RpcError,
    rpc_request::RpcRequest,
    rpc_response::RpcResponse,
    transports::{Http, Transport},
};
