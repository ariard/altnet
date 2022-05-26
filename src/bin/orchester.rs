//! The main daemon orchestring the operations of the other AltNet daemons

extern crate capnp;

use capnp::Error;
use capnp::primitive_list;

use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use altnet::control_capnp::control;
use altnet::interfaces::control::Control;
use capnp::capability::Promise;

use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use futures::future;



//todo: move async/future to rust

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	tokio::task::LocalSet::new().run_until(async move {
		let listener = tokio::net::TcpListener::bind("127.0.0.1:10031").await?;
		let control: control::Client = capnp_rpc::new_client(Control);

		loop {
			let (stream, _) = listener.accept().await?;
			stream.set_nodelay(true)?;
			let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
			let network =
				twoparty::VatNetwork::new(reader, writer,
							rpc_twoparty_capnp::Side::Server, Default::default());

			let rpc_system = RpcSystem::new(Box::new(network), Some(control.clone().client));
			tokio::task::spawn_local(Box::pin(rpc_system.map_err(|e| println!("error: {:?}", e)).map(|_| ())));
		}
	}).await
}
