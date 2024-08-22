mod add_domain_record_response;
mod del_domain_record_response;
mod query_domain_records_response;
mod update_domain_record_response;

pub use add_domain_record_response::AddDomainRecordResponse;
pub use del_domain_record_response::DelDomainRecordResponse;
pub use query_domain_records_response::QueryDomainRecordsResponse;
pub use update_domain_record_response::UpdateDomainRecordResponse;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultResponseSuccess {
	#[serde(rename = "RequestId")]
	pub request_id: String,
	#[serde(rename = "RecordId")]
	pub record_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultResponseError {
	#[serde(rename = "RequestId")]
	pub request_id: String,
	#[serde(rename = "HostId")]
	pub host_id: String,
	#[serde(rename = "Code")]
	pub code: String,
	#[serde(rename = "Message")]
	pub message: String,
	#[serde(rename = "Recommend")]
	pub recommend: Option<String>,
}
