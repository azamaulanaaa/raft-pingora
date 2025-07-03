use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct RaftConfig {
    #[serde(default = "id::default")]
    pub id: u64,
    #[serde(default = "listen_address::default")]
    pub listen_address: SocketAddr,
}

pub mod listen_address {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    pub fn default() -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3001))
    }
}

pub mod id {
    pub fn default() -> u64 {
        0
    }
}

impl Default for RaftConfig {
    fn default() -> Self {
        RaftConfig {
            id: id::default(),
            listen_address: listen_address::default(),
        }
    }
}
