use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRecords {
	#[serde(rename = "Record")]
	pub records: Vec<DomainRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRecord {
	#[serde(rename = "Status")]
	pub status: String,
	#[serde(rename = "RR")]
	pub rr: String,
	#[serde(rename = "Line")]
	pub line: String,
	#[serde(rename = "Locked")]
	pub locked: bool,
	#[serde(rename = "Type")]
	pub r#type: String,
	#[serde(rename = "DomainName")]
	pub domain_name: String,
	#[serde(rename = "Value")]
	pub value: String,
	#[serde(rename = "RecordId")]
	pub record_id: String,
	#[serde(rename = "UpdateTimestamp")]
	pub update_timestamp: usize,
	#[serde(rename = "TTL")]
	pub ttl: usize,
	#[serde(rename = "CreateTimestamp")]
	pub create_timestamp: usize,
	#[serde(rename = "Weight")]
	pub weight: usize,
}
