use crate::types::{self, DnsConfig, RRType};

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
	pub async fn query_domain_records(&self, domain_name: &str) -> anyhow::Result<types::QueryDomainRecordsResponse> {
		static ACTION: &str = "DescribeDomainRecords";
		let mut request = self.dns_config.get_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("Action", ACTION);
		request.url_mut().query_pairs_mut().append_pair("DomainName", domain_name);
		let _ = self.dns_config.sign_request(&mut request)?;
		let response = self.dns_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Ok(types::QueryDomainRecordsResponse::Error(response.json().await?));
		}
		Ok(types::QueryDomainRecordsResponse::Success(response.json().await?))
	}
	pub async fn add_domain_record(&self, domain_name: &str, rr: &str, rr_type: RRType, value: &str, line: Option<&str>) -> anyhow::Result<types::AddDomainRecordResponse> {
		static ACTION: &str = "AddDomainRecord";
		let mut request = self.dns_config.get_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("Action", ACTION);
		request.url_mut().query_pairs_mut().append_pair("DomainName", domain_name);
		request.url_mut().query_pairs_mut().append_pair("RR", rr);
		request.url_mut().query_pairs_mut().append_pair("Type", rr_type.to_str());
		request.url_mut().query_pairs_mut().append_pair("Value", value);
		if let Some(line) = line {
			request.url_mut().query_pairs_mut().append_pair("Line", line);
		}
		let _ = self.dns_config.sign_request(&mut request)?;
		let response = self.dns_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Ok(types::AddDomainRecordResponse::Error(response.json().await?));
		}
		Ok(types::AddDomainRecordResponse::Success(response.json().await?))
	}
	pub async fn update_domain_record(&self, record_id: &str, rr: &str, rr_type: RRType, value: &str, line: Option<&str>) -> anyhow::Result<types::UpdateDomainRecordResponse> {
		static ACTION: &str = "UpdateDomainRecord";
		let mut request = self.dns_config.get_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("Action", ACTION);
		request.url_mut().query_pairs_mut().append_pair("RecordId", record_id);
		request.url_mut().query_pairs_mut().append_pair("RR", rr);
		request.url_mut().query_pairs_mut().append_pair("Type", rr_type.to_str());
		request.url_mut().query_pairs_mut().append_pair("Value", value);
		if let Some(line) = line {
			request.url_mut().query_pairs_mut().append_pair("Line", line);
		}
		let _ = self.dns_config.sign_request(&mut request)?;
		let response = self.dns_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Ok(types::UpdateDomainRecordResponse::Error(response.json().await?));
		}
		Ok(types::UpdateDomainRecordResponse::Success(response.json().await?))
	}
	pub async fn delete_domain_record(&self, record_id: &str) -> anyhow::Result<types::DelDomainRecordResponse> {
		static ACTION: &str = "DeleteDomainRecord";
		let mut request = self.dns_config.get_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("Action", ACTION);
		request.url_mut().query_pairs_mut().append_pair("RecordId", record_id);
		let _ = self.dns_config.sign_request(&mut request)?;
		let response = self.dns_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Ok(types::DelDomainRecordResponse::Error(response.json().await?));
		}
		Ok(types::DelDomainRecordResponse::Success(response.json().await?))
	}
}
