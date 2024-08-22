use crate::types;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DelDomainRecordResponse {
	Success(types::DefaultResponseSuccess),
	Error(types::DefaultResponseError),
}
