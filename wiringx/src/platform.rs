//! All supported platforms of this library.

use std::{ffi::CString, os::raw::c_char};

use thiserror::Error;

/// All supported platforms of WiringX
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Platform {
    Odriodc1,
    Odriodc2,
    Odriodxu4,

    BananaPi1,
    BananaPim2,

    Pcduino1,

    MilkVDuo,
    MilkVDuo256M,
    MilkVDuoS,

    Rock4,
    Rock5b,

    RaspberryPi1b1,
    RaspberryPi1b2,
    RaspberryPi1bPlus,
    RaspberryPi2,
    RaspberryPi3,
    RaspberryPi4,
    RaspberryPiZero,

    HummingboardBasedq,
    HummingboardProdq,
    HummingboardBasesdl,
    HummingboardProsdl,
    HummingboardGatedq,
    HummingboardEdgedq,
    HummingboardGatesdl,
    HummingboardEdgesdl,

    OrangePiPC2,
    OrangePiPCPlus,
}

impl Platform {
    pub(crate) fn as_c_addr(&self) -> *mut c_char {
        let string = match self {
            Self::Odriodc1 => "odroidc1",
            Self::Odriodc2 => "odroidc2",
            Self::Odriodxu4 => "odroidxu4",
            Self::BananaPi1 => "bananapi1",
            Self::BananaPim2 => "bananapim2",
            Self::Pcduino1 => "pcduino1",
            Self::MilkVDuo => "milkv_duo",
            Self::MilkVDuo256M => "milkv_duo256m",
            Self::MilkVDuoS => "milkv_duos",
            Self::Rock4 => "rock4",
            Self::Rock5b => "rock5b",
            Self::RaspberryPi1b1 => "raspberrypi1b1",
            Self::RaspberryPi1b2 => "raspberrypi1b2",
            Self::RaspberryPi1bPlus => "raspberrypi1b+",
            Self::RaspberryPi2 => "raspberrypi2",
            Self::RaspberryPi3 => "raspberrypi3",
            Self::RaspberryPi4 => "raspberrypi4",
            Self::RaspberryPiZero => "raspberrypizero",
            Self::HummingboardBasedq => "hummingboard_base_dq",
            Self::HummingboardProdq => "hummingboard_pro_dq",
            Self::HummingboardBasesdl => "hummingboard_base_sdl",
            Self::HummingboardProsdl => "hummingboard_pro_sdl",
            Self::HummingboardEdgedq => "hummingboard_edge_dq",
            Self::HummingboardGatedq => "hummingboard_gate_dq",
            Self::HummingboardEdgesdl => "hummingboard_edge_sdl",
            Self::HummingboardGatesdl => "hummingboard_gate_sdl",
            Self::OrangePiPC2 => "orangepipc2",
            Self::OrangePiPCPlus => "orangepipc+",
        };

        let cstring = CString::new(string).unwrap();

        cstring.into_raw() as *mut c_char
    }

    /// Parses a string to the platform type.
    pub fn from_string(string: &str) -> Result<Self, PlatformParseError> {
        let platform = match string.to_lowercase().as_str() {
            "odroidc1" => Self::Odriodc1,
            "odroidc2" => Self::Odriodc2,
            "odroidxu4" => Self::Odriodxu4,
            "bananapi1" => Self::BananaPi1,
            "bananapim2" => Self::BananaPim2,
            "pcduino1" => Self::Pcduino1,
            "milkv_duo" => Self::MilkVDuo,
            "milkv_duo256m" => Self::MilkVDuo256M,
            "milkv_duos" => Self::MilkVDuoS,
            "rock4" => Self::Rock4,
            "rock5b" => Self::Rock5b,
            "raspberrypi1b1" => Self::RaspberryPi1b1,
            "raspberrypi1b2" => Self::RaspberryPi1b2,
            "raspberrypi1b+" => Self::RaspberryPi1bPlus,
            "raspberrypi1bplus" => Self::RaspberryPi1bPlus,
            "raspberrypi2" => Self::RaspberryPi2,
            "raspberrypi3" => Self::RaspberryPi3,
            "raspberrypi4" => Self::RaspberryPi4,
            "raspberrypizero" => Self::RaspberryPiZero,
            "hummingboard_base_dq" => Self::HummingboardBasedq,
            "hummingboard_pro_dq" => Self::HummingboardProdq,
            "hummingboard_base_sdl" => Self::HummingboardBasesdl,
            "hummingboard_pro_sdl" => Self::HummingboardProsdl,
            "hummingboard_edge_dq" => Self::HummingboardEdgedq,
            "hummingboard_gate_dq" => Self::HummingboardGatedq,
            "hummingboard_edge_sdl" => Self::HummingboardEdgesdl,
            "hummingboard_gate_sdl" => Self::HummingboardGatesdl,
            "orangepipc2" => Self::OrangePiPC2,
            "orangepipc+" => Self::OrangePiPCPlus,
            "orangepipcplus" => Self::OrangePiPCPlus,
            _ => return Err(PlatformParseError(string.to_string())),
        };

        Ok(platform)
    }
}

/// Returns when the given platform string is invalid.
#[derive(Debug, Error)]
#[error("Can not determine a valid platform from {0}.")]
pub struct PlatformParseError(String);
