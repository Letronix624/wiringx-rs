//! Universal asynchronous receiver/transmitter serial communication related objects.

use std::{
    ffi::{c_uchar, c_uint, CString},
    os::fd::RawFd,
    path::PathBuf,
};

use thiserror::Error;
use wiringx_sys::{
    wiringXSerialClose, wiringXSerialDataAvail, wiringXSerialFlush, wiringXSerialGetChar,
    wiringXSerialOpen, wiringXSerialPutChar, wiringXSerialPuts, wiringXSerial_t,
};

use crate::{Hand, WiringXError};

/// Configuration of the serial connection.
#[derive(Clone, Copy, Debug)]
pub struct SerialConfig {
    pub baud_rate: u32,
    pub data_bits: u32,
    pub parity: Parity,
    pub stop_bits: u32,
    pub flow_control: FlowControl,
}

impl SerialConfig {
    /// Checks if the configuration provided in this struct is valid and usable in wiringX.
    pub fn check(&self) -> Result<(), InvalidUARTConfig> {
        match self.baud_rate {
            50 => (),
            75 => (),
            110 => (),
            134 => (),
            150 => (),
            200 => (),
            300 => (),
            600 => (),
            1200 => (),
            1800 => (),
            2400 => (),
            4800 => (),
            9600 => (),
            19200 => (),
            38400 => (),
            57600 => (),
            115200 => (),
            230400 => (),
            _ => return Err(InvalidUARTConfig::BaudRate),
        };

        match self.data_bits {
            7 => (),
            8 => (),
            _ => return Err(InvalidUARTConfig::DataBits),
        };

        match self.stop_bits {
            1 => (),
            2 => (),
            _ => return Err(InvalidUARTConfig::StopBits),
        };

        Ok(())
    }
}

impl From<SerialConfig> for wiringXSerial_t {
    fn from(rh: SerialConfig) -> Self {
        let parity = match rh.parity {
            Parity::Odd => 'o' as c_uint,
            Parity::Even => 'e' as c_uint,
            Parity::None => 'n' as c_uint,
        };
        let flow_control = match rh.flow_control {
            FlowControl::XOnOff => 'x' as c_uint,
            FlowControl::None => 'n' as c_uint,
        };
        wiringXSerial_t {
            baud: rh.baud_rate,
            databits: rh.data_bits,
            parity,
            stopbits: rh.stop_bits,
            flowcontrol: flow_control,
        }
    }
}

/// A serial UART instance.
#[derive(Debug)]
pub struct Uart {
    fd: RawFd,
    dev: PathBuf,
    handles: Hand<PathBuf>,
}

impl Uart {
    pub(super) fn new(
        dev: PathBuf,
        config: SerialConfig,
        handles: Hand<PathBuf>,
    ) -> Result<Self, WiringXError> {
        config.check().map_err(WiringXError::InvalidUARTConfig)?;

        if handles.lock().contains(&dev.clone()) {
            return Err(WiringXError::PinUsed);
        }

        let path_string = CString::new(dev.to_str().ok_or(WiringXError::Other(
            "Path contains illegal symbols.".to_string(),
        ))?)
        .map_err(|e| WiringXError::Other(e.to_string()))?;

        let fd_result = unsafe { wiringXSerialOpen(path_string.as_ptr(), config.into()) };

        if fd_result < 0 {
            return Err(WiringXError::Unsupported);
        }

        handles.lock().insert(dev.clone());

        Ok(Self {
            fd: fd_result,
            dev,
            handles,
        })
    }

    /// Flushes the buffer.
    pub fn flush(&self) {
        unsafe { wiringXSerialFlush(self.fd) }
    }

    /// Outputs a character.
    pub fn put_char(&self, character: char) {
        unsafe { wiringXSerialPutChar(self.fd, character as c_uchar) }
    }

    /// Outputs a string.
    pub fn put_string(&self, string: &str) {
        let c_string = CString::new(string).unwrap();

        unsafe { wiringXSerialPuts(self.fd, c_string.as_ptr()) }
    }

    /// Returns the number of bytes present in the receiving buffer.
    pub fn data_available(&self) -> usize {
        unsafe { wiringXSerialDataAvail(self.fd) as usize }
    }

    /// Returns a character from the receiving buffer.
    pub fn read_char(&self) -> char {
        unsafe { char::from_u32_unchecked(wiringXSerialGetChar(self.fd) as u32) }
    }
}

impl Drop for Uart {
    fn drop(&mut self) {
        unsafe { wiringXSerialClose(self.fd) }
        self.handles.lock().remove(&self.dev);
    }
}

/// UART error correction parity.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Parity {
    /// No parity at all
    None,
    /// Even parity
    Even,
    /// Odd parity
    Odd,
}

/// UART flow control
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FlowControl {
    /// No flow control
    None,
    /// Software flow control using special control characters
    XOnOff,
}

/// When a setting in the config is not supported, this error gets returned.
#[derive(Error, Debug, Clone, Copy)]
pub enum InvalidUARTConfig {
    #[error("The provided baud rate is not valid.")]
    BaudRate,
    #[error("The data size is not valid.")]
    DataBits,
    #[error("The number of stop bits is not valid.")]
    StopBits,
}
