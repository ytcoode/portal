use std::net::SocketAddr;

pub fn validate_port(s: &str) -> Result<(), String> {
    s.parse::<u16>()
        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
}

pub fn validate_socket_addr(s: &str) -> Result<(), String> {
    s.parse::<SocketAddr>()
        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
}
