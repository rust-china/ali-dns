// examples/common/mod.rs
pub mod common;
use ali_dns::types::RRType;
use ali_dns::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;
	let client: Client = Client::from_env()?;
	let ret = client.delete_domain_record("913239007242594304").await?;
	println!("{:?}", ret);
	Ok(())
}
