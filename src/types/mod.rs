mod dns_config;
mod rr_type;

pub mod domain_record;
pub mod response_type;

pub use dns_config::DnsConfig;
pub use rr_type::RRType;

pub use domain_record::{DomainRecord, DomainRecords};
pub use response_type::{AddDomainRecordResponse, DefaultResponseError, DefaultResponseSuccess, DelDomainRecordResponse, QueryDomainRecordsResponse, UpdateDomainRecordResponse};
