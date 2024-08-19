// examples/common/mod.rs
pub mod common;
use ali_dns::types::RRType;
use ali_dns::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;
	let client = Client::from_env()?;
	client.add_domain_record("rust-cn", "www", RRType::A, "127.0.0.1", None).await?;
	Ok(())
}
