// TODO: Реализация HTTP, gRPC и WebSocket handlers

pub mod http;
pub mod grpc;
pub mod websocket;

// Re-exports для удобства
pub use websocket::{WebSocketServer, WebSocketEvent, ClientCommand, Connection};
pub use http::*; 