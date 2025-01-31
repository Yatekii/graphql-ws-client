//! # graphql-ws-client
//!
//! graphql-ws-client implements asynchronous GraphQL over websockets using the
//! [graphql-transport-ws protocol][protocol].  It tries to be websocket client,
//! graphql client _and_ async executor and provides built in support for:
//!
//! - [Cynic][cynic] as a GraphQL client.
//! - [async-tungstenite][async-tungstenite] as a Websocket Client.
//! - The [async-std][async-std] & [tokio][tokio] async runtimes.
//!
//! If you'd like to use another client or runtime adding support should
//! hopefully be trivial.
//!
//! [protocol]: https://github.com/enisdenjo/graphql-ws/blob/HEAD/PROTOCOL.md
//! [cynic]: https://cynic-rs.dev
//! [async-tungstenite]: https://github.com/sdroege/async-tungstenite
//! [async-std]: https://async.rs/
//! [tokio]: https://tokio.rs/

#![warn(missing_docs)]

mod client;
mod protocol;

pub mod graphql;
pub mod websockets;

#[cfg(feature = "ws_stream_wasm")]
mod wasm;
#[cfg(feature = "ws_stream_wasm")]
pub use wasm::*;

pub use client::{AsyncWebsocketClient, AsyncWebsocketClientBuilder, Error, SubscriptionStream};

/// A websocket client for the cynic graphql crate
#[cfg(feature = "cynic")]
pub type CynicClient<WsMessage> = AsyncWebsocketClient<graphql::Cynic, WsMessage>;
/// A websocket client builder for the cynic graphql crate
#[cfg(feature = "cynic")]
pub type CynicClientBuilder = AsyncWebsocketClientBuilder<graphql::Cynic>;
