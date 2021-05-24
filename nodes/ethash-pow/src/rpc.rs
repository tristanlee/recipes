#![warn(missing_docs)]

use std::sync::Arc;

use futures::channel::mpsc::Sender;
use runtime::{opaque::Block, Hash};

pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use sc_consensus_pow::{MiningWorker, MiningMetadata, MiningBuild};
use sc_consensus_pow::{Error, PowAlgorithm};
use parking_lot::Mutex;

/// Full client dependencies.
pub struct FullDeps<C, P, B, Algorithm, C1> where
	B: BlockT,
	Algorithm: PowAlgorithm<B>,
	C1: ProvideRuntimeApi<B>,
{
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	// /// A command stream to send authoring commands to manual seal consensus engine
	// pub command_sink: Sender<EngineCommand<Hash>>,
	pub worker: Arc<Mutex<MiningWorker<B, Algorithm, C1>>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, B, Algorithm, C1>(deps: FullDeps<C, P, B, Algorithm, C1>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: BlockBuilder<Block>,
	C::Api: sum_storage_runtime_api::SumStorageApi<Block>,
	P: TransactionPool + 'static,
	B: BlockT,
	Algorithm: PowAlgorithm<B>,
	C1: ProvideRuntimeApi<B>,
{
	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		worker,
		..
	} = deps;

	// Add a silly RPC that returns constant values
	io.extend_with(crate::ethash_rpc::EthashRpc::to_delegate(
		crate::ethash_rpc::EthashData::new(client, worker),
	));

	// Add a second RPC extension
	// Because this one calls a Runtime API it needs a reference to the client.
	io.extend_with(sum_storage_rpc::SumStorageApi::to_delegate(
		sum_storage_rpc::SumStorage::new(client),
	));

	io
}
