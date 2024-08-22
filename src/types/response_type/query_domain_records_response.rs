use crate::types;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum QueryDomainRecordsResponse {
	Success(QueryDomainRecordsResponseSuccess),
	Error(types::DefaultResponseError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryDomainRecordsResponseSuccess {
	#[serde(rename = "RequestId")]
	pub request_id: String,
	#[serde(rename = "TotalCount")]
	pub total_count: usize,
	#[serde(rename = "PageSize")]
	pub page_size: usize,
	#[serde(rename = "DomainRecords")]
	pub domain_records: types::DomainRecords,
}
