use anyhow::{Error, Result};
use futures::StreamExt;
use log::*;
use std::time::Duration;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    time::timeout,
};
use tokio_util::compat::*;
use tunshell_shared::{ClientMessage, KeyPayload, MessageStream, ServerMessage};

type ClientMessageStream<IO> = MessageStream<ServerMessage, ClientMessage, IO>;

pub(super) struct Connection<IO: AsyncRead + AsyncWrite + Unpin> {
    stream: ClientMessageStream<Compat<IO>>,
}

impl<IO: AsyncRead + AsyncWrite + Unpin> Connection<IO> {
    pub(super) fn new(stream: IO) -> Self {
        Self {
            stream: ClientMessageStream::new(stream.compat()),
        }
    }

    pub(super) async fn next(&mut self) -> Result<ClientMessage> {
        match self.stream.next().await {
            Some(result) => result,
            None => Err(Error::msg("no messages are left in stream")),
        }
    }

    pub(super) async fn wait_for_key(&mut self, timeout_duration: Duration) -> Result<KeyPayload> {
        let message = timeout(timeout_duration, self.next()).await??;

        match message {
            ClientMessage::Key(key) => Ok(key),
            message @ _ => Err(Error::msg(format!(
                "unexpected message received from client, expecting key, got {:?}",
                message
            ))),
        }
    }

    pub(super) async fn write(&mut self, message: ServerMessage) -> Result<()> {
        self.stream.write(&message).await
    }
}