use std::net::IpAddr;

pub fn display_host(local_ip: Option<String>, fallback_ip: IpAddr) -> String {
    match local_ip {
        Some(s) => s,
        None => fallback_ip.to_string(),
    }
}
