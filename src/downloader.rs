use crate::config::DownloaderConfig;
use transmission_rpc::types::{BasicAuth, PortTest, Result, RpcResponse};
use transmission_rpc::TransClient;
use url::Url;

pub struct Transmission {
    pub name: String,
    client: TransClient,
}

impl Transmission {
    pub fn new(name: String, config: &DownloaderConfig) -> Self {
        let url = Url::parse(&config.api_url);
        let client = TransClient::with_auth(
            url.unwrap(), 
            BasicAuth { 
                user: config.user.to_string(),
                password: config.password.to_string(),
            }
        );
        Transmission {
            name: name.to_string(),
            client: client,
        }
    }
    
    pub async fn is_alive(&mut self) -> bool {
        let resp: Result<RpcResponse<PortTest>> = 
            self.client.port_test().await;
        match resp {
            Ok(_) => true,
            Err(_) => false,
        }   
    }
}

