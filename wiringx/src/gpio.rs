//! GPIO related objects.

use std::time::Duration;

use wiringx_sys::{
    digitalRead, digitalWrite, digital_value_t_HIGH, digital_value_t_LOW, pinMode,
    pinmode_t_PINMODE_INPUT, pinmode_t_PINMODE_OUTPUT, waitForInterrupt, wiringXISR,
};

use crate::WiringXError;

/// Representation of a GPIO pin.
pub struct Pin<T>(i32, std::marker::PhantomData<T>);

impl<T> Pin<T> {
    pub(super) fn new(number: i32) -> Self {
        Self(number, std::marker::PhantomData)
    }
}

impl Pin<Output> {
    /// Returns a writable pin.
    pub fn switch_mode(self) -> Pin<Input> {
        unsafe { pinMode(self.0, pinmode_t_PINMODE_INPUT) };

        Pin::new(self.0)
    }

    /// Writes a value to the GPIO pin.
    pub fn write(&self, value: Value) {
        let value = match value {
            Value::High => digital_value_t_HIGH,
            Value::Low => digital_value_t_LOW,
        };

        unsafe { digitalWrite(self.0, value) };
    }
}

impl Pin<Input> {
    /// Returns a readable pin.
    pub fn switch_mode(self) -> Pin<Output> {
        unsafe { pinMode(self.0, pinmode_t_PINMODE_OUTPUT) };

        Pin::new(self.0)
    }

    /// Reads the state of the GPIO pin.
    pub fn read(&self) -> Value {
        let result = unsafe { digitalRead(self.0) };

        if result == 1 {
            Value::High
        } else {
            Value::Low
        }
    }

    /// Sets the interrupt service routine mode of this pin, when to trigger using the `wait_for_interrupt` method.
    pub fn set_isr_mode(&self, mode: IsrMode) -> Result<(), WiringXError> {
        let result = unsafe { wiringXISR(self.0, mode as u32) };

        if result < 0 {
            return Err(WiringXError::Other(
                "Cannot set isr mode of pin to this setting.".to_string(),
            ));
        }

        Ok(())
    }

    /// Suspends the thread until input to this pin was detected or the function times out.
    ///
    /// Returns Ok(()) on successful interrupt read and InterruptTimeOut on timeout.
    pub fn wait_for_interrupt(&self, timeout_dur: Duration) -> Result<(), InterruptTimeOut> {
        let result = unsafe { waitForInterrupt(self.0, timeout_dur.as_millis() as i32) };

        if result < 0 {
            Err(InterruptTimeOut)
        } else {
            Ok(())
        }
    }
}

/// Sets the pin mode to output, allowing writing to the pin value.
#[derive(Debug, Clone, Copy)]
pub struct Output;

/// Sets the pin mode to input, allowing reading the physical value.
#[derive(Debug, Clone, Copy)]
pub struct Input;

/// Digital voltage value of the pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    High,
    Low,
}

/// Returned if a interrupt function times out.
#[derive(Debug, Clone, Copy)]
pub struct InterruptTimeOut;

/// Mode for the interrupt service routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsrMode {
    Unknown = 0,
    Rising = 2,
    Falling = 4,
    Both = 8,
    None = 16,
}
