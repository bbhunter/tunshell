use crate::shell::proto::WindowSize;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(super) trait Shell {
    async fn read(&mut self, buff: &mut [u8]) -> Result<usize>;

    async fn write(&mut self, buff: &[u8]) -> Result<()>;

    fn resize(&mut self, size: WindowSize) -> Result<()>;

    fn exit_code(&self) -> Result<u8>;
}
