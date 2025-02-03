use std::io::Write;
use std::path::Path;
use std::{io::Read, net::SocketAddr};

use super::adb_message_device::ADBMessageDevice;
use super::models::MessageCommand;
use super::ADBTransportMessage;
use crate::{ADBDeviceExt, ADBMessageTransport, ADBTransport, Result, TcpTransport};

/// Represent a device reached and available over USB.
#[derive(Debug)]
pub struct ADBTcpDevice {
    pub inner: ADBMessageDevice<TcpTransport>,
}

impl ADBTcpDevice {
    /// Instantiate a new [`ADBTcpDevice`]
    pub fn new(address: SocketAddr) -> Result<Self> {
        let mut device = Self {
            inner: ADBMessageDevice::new(TcpTransport::new(address)?),
        };

        device.connect().map_err(|e| format!("Failed to connect: {}", e));

        Ok(device)
    }

    /// Send initial connect
    pub fn connect(&mut self) -> Result<()> {
        self.get_transport_mut().connect()?;

        let message = ADBTransportMessage::new(
            MessageCommand::Cnxn,
            0x01000000,
            1048576,
            format!("host::{}\0", "").as_bytes(),
        );

        self.get_transport_mut().write_message(message)?;
        
        self.get_transport_mut()
            .read_message()
            .map_err(|e| format!("Failed to read message: {}", e));

        log::debug!("Connection successfully upgraded from TCP to TLS");
        
        Ok(())
    }

    #[inline]
    fn get_transport_mut(&mut self) -> &mut TcpTransport {
        self.inner.get_transport_mut()
    }

    #[inline]
    pub fn inner(&self) -> &ADBMessageDevice<TcpTransport> {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut ADBMessageDevice<TcpTransport> {
        &mut self.inner
    }
}

impl ADBDeviceExt for ADBTcpDevice {
    #[inline]
    fn shell_command(&mut self, command: &[&str], output: &mut dyn Write) -> Result<()> {
        self.inner.shell_command(command, output)
    }

    #[inline]
    fn shell(&mut self, reader: &mut dyn Read, writer: Box<(dyn Write + Send)>) -> Result<()> {
        self.inner.shell(reader, writer)
    }

    #[inline]
    fn stat(&mut self, remote_path: &str) -> Result<crate::AdbStatResponse> {
        self.inner.stat(remote_path)
    }

    #[inline]
    fn pull(&mut self, source: &dyn AsRef<str>, output: &mut dyn Write) -> Result<()> {
        self.inner.pull(source, output)
    }

    #[inline]
    fn push(&mut self, stream: &mut dyn Read, path: &dyn AsRef<str>) -> Result<()> {
        self.inner.push(stream, path)
    }

    #[inline]
    fn reboot(&mut self, reboot_type: crate::RebootType) -> Result<()> {
        self.inner.reboot(reboot_type)
    }

    #[inline]
    fn install(&mut self, apk_path: &dyn AsRef<Path>) -> Result<()> {
        self.inner.install(apk_path)
    }

    #[inline]
    fn framebuffer_inner(&mut self) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
        self.inner.framebuffer_inner()
    }
}

impl Drop for ADBTcpDevice {
    fn drop(&mut self) {
        // Best effort here
        let _ = self.get_transport_mut().disconnect();
    }
}
