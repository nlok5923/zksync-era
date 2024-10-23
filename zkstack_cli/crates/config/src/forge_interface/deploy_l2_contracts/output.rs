use ethers::types::{Bytes,Address};
use serde::{Deserialize, Serialize};

use crate::traits::ZkStackConfig;

impl ZkStackConfig for InitializeBridgeOutput {}
impl ZkStackConfig for DefaultL2UpgradeOutput {}
impl ZkStackConfig for ConsensusRegistryOutput {}
impl ZkStackConfig for Multicall3Output {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeBridgeOutput {
    pub l2_shared_bridge_implementation: Address,
    pub l2_shared_bridge_proxy: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultL2UpgradeOutput {
    pub l2_default_upgrader: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRegistryOutput {
    pub consensus_registry_implementation: Address,
    pub consensus_registry_proxy: Address,
    pub consensus_registry_proxy_constructor_data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multicall3Output {
    pub multicall3: Address,
}
