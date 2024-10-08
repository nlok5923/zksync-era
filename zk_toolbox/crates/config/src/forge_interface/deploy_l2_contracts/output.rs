use ethers::types::Address;
use serde::{Deserialize, Serialize};

use crate::traits::ZkToolboxConfig;

impl ZkToolboxConfig for InitializeBridgeOutput {}
impl ZkToolboxConfig for DefaultL2UpgradeOutput {}
impl ZkToolboxConfig for ConsensusRegistryOutput {}
impl ZkToolboxConfig for Multicall3Output {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeBridgeOutput {
    pub l2_da_validator_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultL2UpgradeOutput {
    pub l2_default_upgrader: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRegistryOutput {
    pub consensus_registry_implementation: Address,
    pub consensus_registry_proxy: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multicall3Output {
    pub multicall3: Address,
}
