//! Pulse width modulation related objects.

use std::{io, time::Duration};

use wiringx_sys::{
    wiringXPWMEnable, wiringXPWMSetDuty, wiringXPWMSetPeriod, wiringXPWMSetPolarity,
};

use crate::{Hand, WiringXError};

/// A pulse width modulated pin.
#[derive(Debug)]
pub struct PwmPin {
    number: i32,
    handles: Hand<i32>,
}

impl PwmPin {
    pub(super) fn new(number: i32, handles: Hand<i32>) -> Result<Self, WiringXError> {
        if handles.lock().contains(&number) {
            return Err(WiringXError::PinUsed);
        }

        let result = unsafe { wiringXPWMEnable(number, 1) };

        if result < 0 {
            return Err(WiringXError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "PWM capabilities are not supported on this platform.",
            )));
        }

        handles.lock().insert(number);

        Ok(Self { number, handles })
    }

    /// Sets the period of time a PWM cycle takes.
    pub fn set_pwm_period(&self, period: Duration) -> Result<(), WiringXError> {
        let result = unsafe { wiringXPWMSetPeriod(self.number, period.as_nanos() as i64) };

        if result < 0 {
            return Err(WiringXError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "PWM capabilities are not supported on this platform.",
            )));
        }

        Ok(())
    }

    /// Sets the duty cycle of the pin.
    ///
    /// The duty cycle is the proportion of the period the signal is high.
    ///
    /// For example setting this to the half duration of the pwm period makes the signal on 50% of the time.
    pub fn set_pwm_duty(&self, duty_cycle: Duration) -> Result<(), WiringXError> {
        let result = unsafe { wiringXPWMSetDuty(self.number, duty_cycle.as_nanos() as i64) };

        if result < 0 {
            return Err(WiringXError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "PWM capabilities are not supported on this platform.",
            )));
        }

        Ok(())
    }

    /// Sets the polarity of the PWM pin.
    pub fn set_pwm_polarity(&self, polarity: Polarity) -> Result<(), WiringXError> {
        let result = unsafe { wiringXPWMSetPolarity(self.number, polarity as i32) };

        if result < 0 {
            return Err(WiringXError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "PWM capabilities are not supported on this platform.",
            )));
        }

        Ok(())
    }
}

impl Drop for PwmPin {
    fn drop(&mut self) {
        self.handles.lock().remove(&self.number);
        unsafe { wiringXPWMEnable(self.number, 0) };
    }
}

/// PWM polarity of a pin.
#[derive(Debug, Clone, Copy)]
pub enum Polarity {
    Normal = 0,
    Inversed = 1,
}
