pub enum RRType {
	A,
	NS,
	MX,
	TXT,
	CNAME,
	SRV,
	AAAA,
	CAA,
	REDIRECT_URL,
	FORWARD_URL,
}

impl RRType {
	pub fn to_str(&self) -> &str {
		match self {
			Self::A => "A",
			Self::NS => "NS",
			Self::MX => "MX",
			Self::TXT => "TXT",
			Self::CNAME => "CNAME",
			Self::SRV => "SRV",
			Self::AAAA => "AAAA",
			Self::CAA => "CAA",
			Self::REDIRECT_URL => "REDIRECT_URL",
			Self::FORWARD_URL => "FORWARD_URL",
		}
	}
}
