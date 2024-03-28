use crate::rpc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitializeRequest {
    pub rpc: String,
    pub id: i32,
    pub method: String,
    pub params: InitializeParams,
}

#[derive(Serialize, Deserialize)]
pub struct InitializeParams {
    pub clientInfo: Option<ClientInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

impl From<rpc::BaseMessage> for InitializeRequest {
    fn from(value: rpc::BaseMessage) -> Self {
        let to_string = serde_json::to_string(&value.params);
        let client_params: InitializeParams =
            serde_json::from_str::<InitializeParams>(&to_string.unwrap()[..]).unwrap();
        Self {
            rpc: String::from("2.0"),
            id: value.id.unwrap(),
            method: value.method,
            params: client_params,
        }
    }
}
