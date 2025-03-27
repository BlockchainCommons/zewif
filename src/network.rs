use anyhow::{Result, bail};

pub use zcash_protocol::consensus::NetworkType as Network;

pub fn network_for_identifier(identifier: &str) -> Result<Network> {
    if identifier == "main" {
        Ok(Network::Main)
    } else if identifier == "test" {
        Ok(Network::Test)
    } else if identifier == "regtest" {
        Ok(Network::Regtest)
    } else {
        bail!("Invalid network identifier: {}", identifier)
    }
}
