use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait EthashRpc {
	#[rpc(name = "eth_getWork")]
	fn eth_getWork(&self) -> Result<u64>;

	#[rpc(name = "eth_submitWork")]
	fn eth_submitWork(&self, val: u64) -> Result<u64>;

	#[rpc(name = "eth_submitHashrate")]
	fn eth_submitHashrate(&self, val: u64) -> Result<u64>;
}

/// A struct that implements the `EthashRpc`
pub struct EthashData;

impl EthashRpc for EthashData {
	fn eth_getWork(&self) -> Result<u64> {
		Ok(0)
	}

	fn eth_submitWork(&self, val: u64) -> Result<u64> {
		Ok(0)
	}

	fn eth_submitHashrate(&self, val: u64) -> Result<u64> {
		Ok(0)
	}
}
