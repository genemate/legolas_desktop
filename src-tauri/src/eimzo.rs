// E-IMZO native WebSocket bridge for Tauri Desktop.
//
// In the browser, E-IMZO is accessed via a local WebSocket proxy
// (eimzo-proxy.js: ws://localhost:8080 → wss://127.0.0.1:64443).
//
// In Tauri, we bypass the proxy entirely and connect directly to
// the E-IMZO daemon's WSS endpoint using native Rust TLS — no CORS,
// no Mixed Content issues, no separate proxy process needed.

use futures_util::{SinkExt, StreamExt};
use native_tls::TlsConnector;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::Message, Connector};

const EIMZO_URL: &str = "wss://127.0.0.1:64443/service/cryptapi";

/// Send a JSON command to the local E-IMZO daemon and return its response.
///
/// Accepts self-signed certificate (E-IMZO uses a locally generated cert).
/// Mirrors the behaviour of eimzo-proxy.js `rejectUnauthorized: false`.
#[tauri::command]
pub async fn eimzo_send(payload: String) -> Result<String, String> {
    // Build TLS connector accepting self-signed certs (E-IMZO local daemon)
    let tls = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .map_err(|e| format!("TLS build error: {e}"))?;

    let connector = Connector::NativeTls(tls);

    let (mut ws, _) =
        connect_async_tls_with_config(EIMZO_URL, None, false, Some(connector))
            .await
            .map_err(|e| format!("E-IMZO не запущен или недоступен: {e}"))?;

    ws.send(Message::Text(payload))
        .await
        .map_err(|e| format!("Send error: {e}"))?;

    match ws.next().await {
        Some(Ok(Message::Text(response))) => Ok(response),
        Some(Ok(other)) => Err(format!("Unexpected message type: {other:?}")),
        Some(Err(e)) => Err(format!("Receive error: {e}")),
        None => Err("E-IMZO closed connection without response".to_string()),
    }
}

/// Check if E-IMZO daemon is running and reachable.
#[tauri::command]
pub async fn eimzo_check() -> bool {
    let tls = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build();

    match tls {
        Ok(tls) => {
            let connector = Connector::NativeTls(tls);
            connect_async_tls_with_config(EIMZO_URL, None, false, Some(connector))
                .await
                .is_ok()
        }
        Err(_) => false,
    }
}
