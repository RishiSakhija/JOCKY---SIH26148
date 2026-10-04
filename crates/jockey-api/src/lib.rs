//! JOCKY API — Axum HTTP Server, WebSocket, OpenAPI
//! 
//! Phase 2: Stub implementation for CI validation.
//! Actual implementation begins in Phase 6.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-exports for public API
pub mod server;
pub mod routes;
pub mod websocket;
pub mod state;

/// Server module
pub mod server {
    use std::net::SocketAddr;
    
    /// Application state
    pub struct AppState;
    
    /// Serve the API
    pub async fn serve(_addr: SocketAddr, _state: AppState) -> Result<(), ServerError> {
        unimplemented!("Phase 6 implementation")
    }
    
    /// Server error
    #[derive(Debug, thiserror::Error)]
    pub enum ServerError {
        #[error("bind error: {0}")]
        BindError(String),
        #[error("runtime error: {0}")]
        RuntimeError(String),
    }
}

/// Routes module
pub mod routes {
    /// Router placeholder
    pub struct Router;
}

/// WebSocket module
pub mod websocket {
    /// WebSocket handler placeholder
    pub struct WsHandler;
}

/// State module
pub mod state {
    /// App state placeholder
    pub struct AppState;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stub_compiles() {
        assert!(true);
    }
}