//use capnp::capability::Promise;
//use capnp_rpc::{pry, RpcSystem};
//use capnp_rpc::twoparty::{VatNetwork};
//use capnp_rpc::rpc_twoparty_capnp::{Side};
//use futures::{AsyncReadExt, FutureExt, StreamExt};
//use futures::task::LocalSpawn;
//
//use crate::control_capnp::{control};
//
//pub struct Control;
//
//impl control::Server for Control {
//	fn print_hello(&mut self, params: control::PrintHelloParams, mut results: control::PrintHelloResults) -> Promise<(), capnp::Error> {
//		println!("Hello World");
//		Promise::from_future(async move {
//			Ok(())
//		})
//	}
//	fn shutdown(&mut self, params: control::ShutdownParams, mut results: control::ShutdownResults) -> Promise<(), capnp::Error> {
//		Promise::from_future(async move {
//			Ok(())
//		})
//	}
//}
