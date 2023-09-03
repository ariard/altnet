//! The daemon to sync

//use altnet::control_capnp::control;
//use altnet::common::Capabilities;
//
//use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
//
//use futures::{AsyncReadExt, FutureExt};
//
//use bitcoin::BlockHeader;
//
//
//
//struct SyncClient {
//
//
//}
//
//struct SyncContext {
//
//	sync_clients: Vec<SyncClient>
//
//}
//
//struct SyncEngine {
//
//	best_header: BlockHeader,
//}
//
//impl SyncEngine {
//	fn new() {
//
//	}
//
//	fn request_headers() {
//
//	}
//}
//
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

	//tokio::task::LocalSet::new().run_until(async move {
	//	let socket_path = "127.0.0.1:10031";

	//	let stream = tokio::net::TcpStream::connect(socket_path).await?;
	//	stream.set_nodelay(true)?;

	//	let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

	//	let network =
	//		Box::new(twoparty::VatNetwork::new(reader, writer,
	//					rpc_twoparty_capnp::Side::Client,
	//					Default::default()));

	//	let mut rpc_system = RpcSystem::new(network, None);
	//	let control: control::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
	//	tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
	//	{
	//		let request = control.print_hello_request();
	//		let request_promise = request.send();

	//		let response = request_promise.promise.await?;

	//		loop {}
	//	}

	//	// Step X : Boostrap the sync engine
	//	let sync_engine = SyncEngine::new();

	//	loop {
	//		sync_engine.process_headers();
	//		sync_engine.relay_headers();
	//	}


	//	Ok(())
	//}).await
	Ok(())
}
