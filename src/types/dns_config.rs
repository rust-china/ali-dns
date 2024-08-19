#[derive(Debug, Clone)]
pub struct DnsConfig {
	pub access_key_id: String,
	pub access_key_secret: String,
}

#[derive(Debug, Clone)]
pub struct DNSConfig {
	pub access_key_id: String,
	pub access_key_secret: String,
}

impl DnsConfig {
	pub fn from_env() -> anyhow::Result<Self> {
		let access_key_id = std::env::var("ALI_DNS_ACCESS_KEY_ID")?;
		let access_key_secret = std::env::var("ALI_DNS_ACCESS_KEY_SECRET")?;

		let client = Self::new(access_key_id, access_key_secret);
		Ok(client)
	}
	pub fn new(access_key_id: String, access_key_secret: String) -> Self {
		Self {
			access_key_id: access_key_id.to_string(),
			access_key_secret: access_key_secret.to_string(),
		}
	}
}
