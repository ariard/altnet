//! The daemon driving the lightning modules

use altnet::control_capnp::control;

use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use futures::{AsyncReadExt, FutureExt};

struct Chain {

}

struct ControlClient {

}

struct SyncClient {

}

struct Controller {

}

#[tokio::main(flavor = "current thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	tokio::task::LocalSet::new().run_until(async move {
		let socket_path = "127.0.0.1:10031";

		let stream = tokio::net::TcpStream::connect(socket_path).await?;
		stream.set_nodelay(true)?;

		let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

		let network =
			Box::new(twoparty::VatNetwork::new(reader, writer,
						rpc_twoparty_capnp::Side::Client,
						Default::default()));

		let mut rpc_system = RpcSystem::new(network, None);
		let control: control::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

		tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
		{
			let request = control.print_hello_request();
			let request_promise = request.send();

			let response = request_promise.await?;

			loop {}
		}

		Ok(())
	}).await
}
