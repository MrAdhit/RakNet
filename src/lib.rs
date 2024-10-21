#![doc = include_str!("../README.md")]

/// A client implementation of RakNet, allowing you to connect to a RakNet server.
pub mod client;
/// The connection implementation of RakNet, allowing you to send and receive packets.
/// This is barebones, and you should use the client or server implementations instead, this is mainly
/// used internally.
pub mod connection;
/// The error implementation of RakNet, allowing you to handle errors.
pub mod error;
/// The packet implementation of RakNet.
/// This is a lower level implementation responsible for serializing and deserializing packets.
pub mod protocol;
/// The server implementation of RakNet, allowing you to create a RakNet server.
pub mod server;
/// Utilties for RakNet, like epoch time.
pub mod util;

pub use server::Listener;

/// An internal module for notifying the connection of state updates.
pub(crate) mod notify;
