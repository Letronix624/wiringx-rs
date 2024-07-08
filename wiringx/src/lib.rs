//! Safe WiringX Rust bindings.

pub mod platform;

pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod uart;

use i2c::I2C;
use pwm::PwmPin;
use spi::Spi;
use thiserror::Error;

use std::{
    any::TypeId,
    collections::HashSet,
    io,
    os::fd::RawFd,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use parking_lot::Mutex;

use gpio::{Input, Output, Pin};
use platform::Platform;
use wiringx_sys::{
    pinMode, pinmode_t_PINMODE_INPUT, pinmode_t_PINMODE_OUTPUT, wiringXGC, wiringXSelectableFd,
    wiringXSetup, wiringXValidGPIO,
};

static WIRINGX: OnceLock<WiringX> = OnceLock::new();

type Hand<T> = Arc<Mutex<HashSet<T>>>;

/// WiringX functionality.
#[derive(Clone, Debug)]
pub struct WiringX {
    platform: Platform,
    gpio_handles: Hand<i32>,
    pwm_handles: Hand<i32>,
    i2c_handles: Hand<(PathBuf, i32)>,
    spi_handles: Hand<i32>,
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

            WiringX {
                platform,
                gpio_handles: Mutex::new(HashSet::new()).into(),
                pwm_handles: Mutex::new(HashSet::new()).into(),
                i2c_handles: Mutex::new(HashSet::new()).into(),
                spi_handles: Mutex::new(HashSet::new()).into(),
            }
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
    pub fn selectable_fd(&self, gpio_pin: i32) -> Result<RawFd, WiringXError> {
        if !self.valid_gpio(gpio_pin) {
            return Err(WiringXError::InvalidPin);
        }

        let fd = unsafe { wiringXSelectableFd(gpio_pin) };
        if fd < 0 {
            Err(WiringXError::Io(io::Error::last_os_error()))
        } else {
            Ok(fd)
        }
    }

    /// Returns a handle to a pin marked as input or output
    pub fn gpio_pin<State: 'static>(&self, pin_number: i32) -> Result<Pin<State>, WiringXError> {
        if self.gpio_handles.lock().contains(&pin_number) {
            return Err(WiringXError::PinUsed);
        }

        if !self.valid_gpio(pin_number) {
            return Err(WiringXError::InvalidPin);
        }

        let type_id = TypeId::of::<State>();

        if type_id == TypeId::of::<Input>() {
            unsafe { pinMode(pin_number, pinmode_t_PINMODE_INPUT) }
        } else if type_id == TypeId::of::<Output>() {
            unsafe { pinMode(pin_number, pinmode_t_PINMODE_OUTPUT) }
        } else {
            return Err(WiringXError::InvalidStateType);
        };

        self.gpio_handles.lock().insert(pin_number);

        Ok(Pin::new(pin_number, self.gpio_handles.clone()))
    }

    /// Enables and returns a handle to a PWM pin, if supported.
    pub fn pwm_pin(&self, pin_number: i32) -> Result<PwmPin, WiringXError> {
        PwmPin::new(pin_number, self.pwm_handles.clone())
    }

    /// Sets up an I2C instance for the given I2C device path, for example `/dev/i2c-1`, and device address.
    pub fn setup_i2c(&self, path: PathBuf, addr: i32) -> Result<I2C, WiringXError> {
        I2C::new(path, addr, self.i2c_handles.clone())
    }

    /// Sets up an SPI instance for the given device channel.
    ///
    /// Speed is measured in Hertz here.
    pub fn setup_spi(&self, channel: i32, speed: u32) -> Result<Spi, WiringXError> {
        Spi::new(channel, speed as i32, self.spi_handles.clone())
    }
}

impl Drop for WiringX {
    fn drop(&mut self) {
        unsafe {
            wiringXGC();
        }
    }
}

/// Errors that can occur from wiringX.
#[derive(Debug, Error)]
pub enum WiringXError {
    /// Gets returned when an error occurs when starting wiringX.
    ///
    /// Maybe the device this gets used on does not support wiringX.
    #[error("Failed to initialize wiringX: {0}")]
    InitError(String),
    #[error("An unexpected error occured: {0}")]
    Other(String),
    /// A function was used with a pin that is not supported for the given platform.
    #[error("The given pin does not exist for this platform.")]
    InvalidPin,
    /// The provided pin already has an instance. Pins can only exist once.
    #[error("The given pin is already used. Pin instances can only exist once.")]
    PinUsed,
    /// When using the `pin` function of `WiringX` with a generic other than `Input` or `Output`.
    #[error("A pin can not be created with generics other than `Input` or `Output`.")]
    InvalidStateType,
    /// Gets returned when a a function gets called that is not supported on the set platform.
    #[error("The function you are trying to call is not supported on your platform.")]
    Unsupported,
    /// Io os error.
    #[error("IO error: {0}")]
    Io(io::Error),
}
