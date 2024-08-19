use crate::types::{DnsConfig, RRType};

#[derive(Debug)]
pub struct Client {
	dns_config: DnsConfig,
}

impl Client {
	pub fn from_env() -> anyhow::Result<Self> {
		let dns_config = DnsConfig::from_env()?;
		Ok(Self { dns_config })
	}
	pub fn new<T: ToString>(access_key_id: T, access_key_secret: T) -> Self {
		let dns_config = DnsConfig::new(access_key_id.to_string(), access_key_secret.to_string());
		Self { dns_config }
	}
}

impl Client {
	pub async fn add_domain_record(&self, domain_name: &str, rr: &str, rr_type: RRType, value: &str, line: Option<&str>) -> anyhow::Result<()> {
		Ok(())
	}
}
