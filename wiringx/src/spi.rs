use std::{ffi::c_uchar, os::fd::RawFd};

use wiringx_sys::{wiringXSPIDataRW, wiringXSPIGetFd, wiringXSPISetup};

use crate::{Hand, WiringXError};

/// A SPI instance.
#[derive(Debug)]
pub struct Spi {
    channel: i32,
    handle: Hand<i32>,
}

impl Spi {
    pub(super) fn new(channel: i32, speed: i32, handle: Hand<i32>) -> Result<Self, WiringXError> {
        if handle.lock().contains(&channel) {
            return Err(WiringXError::PinUsed);
        }

        let result = unsafe { wiringXSPISetup(channel, speed) };

        if result < 0 {
            return Err(WiringXError::Unsupported);
        }

        handle.lock().insert(channel);

        Ok(Self { channel, handle })
    }

    /// Returns the raw file descriptor of this spi instance.
    pub fn get_fd(&self) -> RawFd {
        unsafe { wiringXSPIGetFd(self.channel) }
    }

    /// Writes the data to the SPI device and overwrites the provided data with the read data from the device.
    pub fn read_write(&self, data: &mut [u8]) -> Result<(), WiringXError> {
        let len = data.len();
        let result = unsafe {
            wiringXSPIDataRW(self.channel, data.as_mut_ptr() as *mut c_uchar, len as i32)
        };

        if result < 0 {
            Err(WiringXError::Other(
                "Failed to read and write to SPI device.".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        self.handle.lock().remove(&self.channel);
    }
}
