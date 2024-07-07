//! Safe WiringX Rust bindings.

pub mod platform;

pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod uart;

use std::{any::TypeId, io::Error, os::fd::RawFd, sync::OnceLock};

use gpio::{Input, Output, Pin};
use platform::Platform;
use wiringx_sys::{
    pinMode, pinmode_t_PINMODE_INPUT, pinmode_t_PINMODE_OUTPUT, wiringXGC, wiringXSelectableFd,
    wiringXSetup, wiringXValidGPIO,
};

static WIRINGX: OnceLock<WiringX> = OnceLock::new();

/// WiringX functionality.
#[derive(Clone, Debug)]
pub struct WiringX {
    platform: Platform,
}

impl WiringX {
    /// Sets up WiringX for the given board.
    pub fn new(platform: Platform) -> Result<&'static Self, WiringXError> {
        let error = OnceLock::new();

        let wiringx = WIRINGX.get_or_init(|| {
            let result = unsafe { wiringXSetup(platform.as_c_addr(), None) };

            if result != 0 {
                error.get_or_init(|| "Failed to initialize WiringX");
            };

            WiringX { platform }
        });

        if let Some(error) = error.get() {
            Err(WiringXError::InitError(error.to_string()))
        } else {
            Ok(wiringx)
        }
    }

    /// Returns the platform this struct got initialized for.
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Returns true if the given GPIO number is valid for the selected platform.
    pub fn valid_gpio(&self, gpio_pin: i32) -> bool {
        let result = unsafe { wiringXValidGPIO(gpio_pin) };

        result == 0
    }

    /// Returns the raw file descriptor to the given GPIO pin.
    pub fn selectable_fd(&self, gpio_pin: i32) -> Result<RawFd, Error> {
        if !self.valid_gpio(gpio_pin) {
            return Err(Error::other("Invalid GPIO pin."));
        }

        let fd = unsafe { wiringXSelectableFd(gpio_pin) };
        if fd < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(fd)
        }
    }

    /// Returns a pin marked as input or output
    pub fn pin<State: 'static>(&self, number: i32) -> Result<Pin<State>, WiringXError> {
        if !self.valid_gpio(number) {
            return Err(WiringXError::InvalidPin);
        }

        let type_id = TypeId::of::<State>();

        if type_id == TypeId::of::<Input>() {
            unsafe { pinMode(number, pinmode_t_PINMODE_INPUT) }
        } else if type_id == TypeId::of::<Output>() {
            unsafe { pinMode(number, pinmode_t_PINMODE_OUTPUT) }
        } else {
            return Err(WiringXError::InvalidStateType);
        };

        Ok(Pin::new(number))
    }
}

impl Drop for WiringX {
    fn drop(&mut self) {
        unsafe {
            wiringXGC();
        }
    }
}

#[derive(Debug)]
pub enum WiringXError {
    InitError(String),
    Other(String),
    InvalidPin,
    InvalidStateType,
    Io(Error),
}
