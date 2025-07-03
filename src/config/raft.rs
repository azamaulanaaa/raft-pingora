use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Raft {
    #[serde(default = "listen_address::default")]
    pub listen_address: SocketAddr,
}

pub mod listen_address {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    pub fn default() -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3001))
    }
}

impl Default for Raft {
    fn default() -> Self {
        Raft {
            listen_address: listen_address::default(),
        }
    }
}
