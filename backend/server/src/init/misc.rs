use git2::opts::{set_server_connect_timeout_in_milliseconds, set_server_timeout_in_milliseconds};
use tracing::warn;

pub fn init() {
    // git2 option
    unsafe {
        if let Err(e) = set_server_connect_timeout_in_milliseconds(10_000) {
            warn!("failed to set git2 server connect timeout: {}", e);
        }
        if let Err(e) = set_server_timeout_in_milliseconds(60_000) {
            warn!("failed to set git2 server timeout: {}", e);
        }
    }
}
