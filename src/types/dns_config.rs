use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{header, Method, Url};

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
	pub fn get_request(&self, method: Method, body: Option<bytes::Bytes>) -> anyhow::Result<reqwest::Request> {
		let mut url = Url::parse("https://alidns.aliyuncs.com/")?;
		let nonce = format!("{}", rand::random::<u64>());
		let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

		url.query_pairs_mut().append_pair("AccessKeyId", &self.access_key_id);
		// url.query_pairs_mut().append_pair("Action", "AddDomainRecord");
		url.query_pairs_mut().append_pair("Format", "JSON");
		url.query_pairs_mut().append_pair("Version", "2015-01-09");
		url.query_pairs_mut().append_pair("SignatureMethod", "HMAC-SHA1");
		url.query_pairs_mut().append_pair("SignatureNonce", &nonce);
		url.query_pairs_mut().append_pair("SignatureVersion", "1.0");
		url.query_pairs_mut().append_pair("Timestamp", &now);

		let mut request = reqwest::Request::new(method, url);
		Ok(request)
	}

	pub(crate) fn sign_request(&self, request: &mut reqwest::Request) -> anyhow::Result<()> {
		let mut query_pairs: Vec<(String, String)> = request.url().query_pairs().into_owned().collect();
		query_pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

		let canonical_query_string = query_pairs.into_iter().map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v))).collect::<Vec<String>>().join("&");
		let string_to_sign = format!("{}&{}&{}", request.method().as_str(), percent_encode(request.url().path()), percent_encode(&canonical_query_string));
		let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, format!("{}&", self.access_key_secret).as_bytes());
		let hmac_signature = ring::hmac::sign(&key, string_to_sign.as_bytes());
		let signature = STANDARD.encode(hmac_signature.as_ref());
		request.url_mut().query_pairs_mut().append_pair("Signature", &signature);

		Ok(())
	}

	pub(crate) fn get_request_builder(&self, request: reqwest::Request) -> anyhow::Result<reqwest::RequestBuilder> {
		let req_client = reqwest::Client::new();
		Ok(reqwest::RequestBuilder::from_parts(req_client, request))
	}
}

fn percent_encode(input: &str) -> String {
	let mut encoded = String::new();
	for byte in input.as_bytes() {
		if *byte == b'*' {
			encoded.push_str("%2A");
		} else {
			let temp = url::form_urlencoded::byte_serialize(&[*byte]).collect::<String>();
			encoded.push_str(&temp);
		}
	}
	encoded
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_percent_encode() {
		assert_eq!(percent_encode("hello"), "hello".to_string());
		assert_eq!(percent_encode("a/b"), "a%2Fb".to_string());
		assert_eq!(percent_encode("a+b"), "a%2Bb".to_string());
		assert_eq!(percent_encode("a b"), "a+b".to_string());
		assert_eq!(percent_encode("*"), "%2A".to_string());
		assert_eq!(percent_encode("%"), "%25".to_string());
		assert_eq!(percent_encode("你好"), "%E4%BD%A0%E5%A5%BD".to_string());
	}
}
