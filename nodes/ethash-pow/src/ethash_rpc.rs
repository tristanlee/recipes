use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use runtime::{self, opaque::Block, RuntimeApi};
use sc_consensus_pow::{MiningWorker, MiningMetadata, MiningBuild};
use sc_consensus_pow::{Error, PowAlgorithm};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use sp_api::ProvideRuntimeApi;
use std::sync::Arc;
use parking_lot::Mutex;

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
pub struct EthashData<B, Algorithm, C> where
	B: BlockT,
	Algorithm: PowAlgorithm<B>,
	C: ProvideRuntimeApi<B>,
{
	client: Arc<C>,
	worker: Arc<Mutex<MiningWorker<B, Algorithm, C>>>,
}

impl<B, Algorithm, C> EthashData<B, Algorithm, C> where
	B: BlockT,
	Algorithm: PowAlgorithm<B>  + Send ,
	C: ProvideRuntimeApi<B> + Send + Sync ,
{
	/// Create new `EthashData` instance with the given reference to the client.
	pub fn new(client: Arc<C>, worker: Arc<Mutex<MiningWorker<B, Algorithm, C>>>) -> Self {
		Self {
			client,
			worker,
		}
	}
}

impl<B, Algorithm, C> EthashRpc for EthashData<B, Algorithm, C> where
	B: BlockT,
	Algorithm: PowAlgorithm<B>  + Send ,
	C: ProvideRuntimeApi<B>  + Send + Sync ,
{
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
