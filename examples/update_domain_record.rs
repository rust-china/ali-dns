// examples/common/mod.rs
pub mod common;
use ali_dns::types::RRType;
use ali_dns::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;
	let client: Client = Client::from_env()?;
	let ret = client.update_domain_record("913231357824065536", "b2", RRType::A, "127.0.0.1", None).await?;
	println!("{:?}", ret);
	Ok(())
}
