// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php

use tokio::runtime::Runtime;

use altnet::nostr::NostrClient;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
	#[clap(short, long, default_value = "8334")]
	port: String,
}

pub struct BitcoindClient {
	host: String,
	port: u16,
}

impl BitcoindClient {
	fn new(port: u16) -> Self {
		BitcoindClient {
			host: String::new(),
			port: port,
		}
	}

	pub async fn run(&self) {
	}
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

	let cli = Cli::parse();

	let bitcoind_client = BitcoindClient::new(u16::from_str_radix(&cli.port[..], 10).unwrap());

	let nostr_client = NostrClient::new();

	let rt = Runtime::new()?;

	rt.block_on(async {

	tokio::spawn(async move {
		nostr_client.run().await;
	});

	tokio::spawn(async move {
		bitcoind_client.run().await;
	});

	loop {}

	});

	Ok(())
}
