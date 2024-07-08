//! I2C related objects.

use std::{ffi::CString, os::fd::RawFd, path::PathBuf};

use thiserror::Error;
use wiringx_sys::{
    wiringXI2CRead, wiringXI2CReadReg16, wiringXI2CReadReg8, wiringXI2CSetup, wiringXI2CWrite,
    wiringXI2CWriteReg8,
};

use crate::{Hand, WiringXError};

/// Implementations for the I2C protocol.
#[derive(Debug)]
pub struct I2C {
    id: (PathBuf, i32),
    i2c_handles: Hand<(PathBuf, i32)>,
    fd: RawFd,
}

impl I2C {
    pub(super) fn new(
        node: PathBuf,
        addr: i32,
        handles: Hand<(PathBuf, i32)>,
    ) -> Result<Self, WiringXError> {
        if handles.lock().contains(&(node.clone(), addr)) {
            return Err(WiringXError::PinUsed);
        }

        let path_string = CString::new(node.to_str().ok_or(WiringXError::Other(
            "Path contains illegal symbols.".to_string(),
        ))?)
        .map_err(|e| WiringXError::Other(e.to_string()))?;

        let fd_result = unsafe { wiringXI2CSetup(path_string.as_ptr(), addr) };

        if fd_result < 0 {
            return Err(WiringXError::Unsupported);
        }

        handles.lock().insert((node.clone(), addr));

        Ok(Self {
            id: (node, addr),
            i2c_handles: handles,
            fd: fd_result,
        })
    }

    /// Reads one byte of data.
    pub fn read(&self) -> Result<u8, I2CError> {
        let result = unsafe { wiringXI2CRead(self.fd) };
        if result < 0 {
            Err(I2CError::Read)
        } else {
            Ok(result as u8)
        }
    }

    /// Reads one byte of data from the given register.
    pub fn read_reg8(&self, reg: i32) -> Result<u8, I2CError> {
        let result = unsafe { wiringXI2CReadReg8(self.fd, reg) };
        if result < 0 {
            Err(I2CError::Read)
        } else {
            Ok(result as u8)
        }
    }

    /// Reads two bytes of data from the given register.
    pub fn read_reg16(&self, reg: i32) -> Result<u16, I2CError> {
        let result = unsafe { wiringXI2CReadReg16(self.fd, reg) };
        if result < 0 {
            Err(I2CError::Read)
        } else {
            Ok(result as u16)
        }
    }

    /// Writes the address of the register, preparing data writes on the device.
    pub fn write(&self, register: i32) -> Result<(), I2CError> {
        let result = unsafe { wiringXI2CWrite(self.fd, register) };
        if result < 0 {
            Err(I2CError::Write)
        } else {
            Ok(())
        }
    }

    /// Writes one byte of data to the given register.
    pub fn write_reg8(&self, register: i32, value: u8) -> Result<(), I2CError> {
        let result = unsafe { wiringXI2CWriteReg8(self.fd, register, value as i32) };
        if result < 0 {
            Err(I2CError::Write)
        } else {
            Ok(())
        }
    }

    /// Writes two bytes of data to the given register.
    pub fn write_reg16(&self, register: i32, value: u16) -> Result<(), I2CError> {
        let result = unsafe { wiringXI2CWriteReg8(self.fd, register, value as i32) };
        if result < 0 {
            Err(I2CError::Write)
        } else {
            Ok(())
        }
    }
}

impl Drop for I2C {
    fn drop(&mut self) {
        self.i2c_handles.lock().remove(&self.id);
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum I2CError {
    #[error("Failed to read from I2C device.")]
    Read,
    #[error("Failed to write to I2C device.")]
    Write,
}
