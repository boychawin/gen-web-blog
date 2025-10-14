use warpy::certificate::new_certificate;

pub fn try_generate_certificate(local_ip: Option<String>, port: u16) {
    if let Some(ip_str) = local_ip {
        if let Err(e) = std::panic::catch_unwind(|| new_certificate(ip_str, port)) {
            log::error!("Failed to generate certificate: {e:?}");
        }
    } else {
        log::error!(
            "TLS requested but no local IP available; proceeding without generating certificate"
        );
    }
}
