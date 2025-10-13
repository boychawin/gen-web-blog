use std::{
    io,
    net::{IpAddr, SocketAddr},
};

use tokio::signal::ctrl_c;
use warpy::{
    constants::{KEY_FILE, PEM_FILE},
    port::next_port_in_range,
    server::routes,
};
use crate::shared::run_server::{cert, display, local_ipaddress};
use qr2term;
use log::{info, error};

pub async fn run(
    folder: String,
    ip: [u8; 4],
    footer: String,
    port: Option<u16>,
    has_tls: bool,
) -> io::Result<()> {
    let ip_addr = IpAddr::from(ip);
    let port = match port {
        Some(p) => p,
        None => next_port_in_range(8080..9000).expect("Cannot open any port in requested range."),
    };
    let socket_addr = SocketAddr::from((ip_addr, port));

    let local_ip = local_ipaddress::get();
    if local_ip.is_none() {
        error!("Unable to determine local IP address for display/QR output");
    }

    let logger = warp::log::custom(|info| {
        let remote_addr = info
            .remote_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "❌ Unknown IP".to_string());
        let status_code = info.status().as_u16();
        let path = info.path();

        info!("🌍 {remote_addr} | {status_code} | {path}");
    });

    if has_tls {
        cert::try_generate_certificate(local_ip.clone(), port);
    }

    let handle = if has_tls {
        tokio::spawn(
            warp::serve(routes(folder, footer, logger))
                .tls()
                .cert_path(PEM_FILE)
                .key_path(KEY_FILE)
                .bind(socket_addr),
        )
    } else {
        tokio::spawn(warp::serve(routes(folder, footer, logger)).bind(socket_addr))
    };

    let display_host = display::display_host(local_ip.clone(), ip_addr);
    let url: String = if has_tls {
        format!("https://{}:{}", display_host, port)
    } else {
        format!("http://{}:{}", display_host, port)
    };

    info!("│  🚀 Server running at: {url}");
    qr2term::print_qr(&url).unwrap();
    info!("│  ├─ 🔄 Press Ctrl+C to stop");
    info!("│  └─ Logs:");

    match ctrl_c().await {
        Ok(()) => {
            info!("\n│  🛑 Shutting down server gracefully...");
        }
        Err(err) => {
            error!("│  ❌ Error listening for shutdown signal: {err}");
        }
    }

    handle.abort();

    match handle.await {
        Ok(_) => {
            info!("│  ✅ Server stopped successfully");
        }
        Err(e) if e.is_cancelled() => {
            info!("│  ✅ Server stopped successfully");
        }
        Err(e) => {
            error!("│  ⚠️  Server stop warning: {e}");
        }
    }

    Ok(())
}
