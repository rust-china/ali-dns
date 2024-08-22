// examples/common/mod.rs
pub mod common;
use ali_dns::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;
	let client: Client = Client::from_env()?;
	let ret = client.query_domain_records("zezeping.com").await?;
	println!("{:?}", ret);
	Ok(())
}
