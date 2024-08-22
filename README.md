# AliDNS &emsp;

[![ci](https://github.com/rust-china/ali-dns/workflows/Rust/badge.svg)](https://github.com/rust-china/ali-dns/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/ali-dns.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/ali-dns.svg
[crates.io]: https://crates.io/crates/ali-dns

### Usage

```rust
let dns_client = crate::Client::from_env()?;
let response = dns_client.add_domain_record(domain_name, rr, rr_type, value, line).await?;
println!("Response: {:?}", response);


```

env config

```
ALI_DNS_ACCESS_KEY_ID=xxx
ALI_DNS_ACCESS_KEY_SECRET=xxx

```

### Methods:

- query_domain_records(domain_name)
- add_domain_record(domain_name, rr, rr_type, value, Option<line>)
- update_domain_record(record_id, rr, rr_type, value, Option<line>)
- delete_domain_record(record_id)

---

官方文档： https://api.aliyun.com/document/Alidns/2015-01-09/AddDomainRecord?spm=a2c4g.11186623.0.0.50a63261yDDaNE&updateTime=2023-10-11
