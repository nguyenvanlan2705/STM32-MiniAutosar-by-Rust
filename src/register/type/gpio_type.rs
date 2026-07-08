#![allow(dead_code)]
#![allow(non_camel_case_types)]


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
// The pin enum defines the available GPIO pins (P0 to P15).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}
// The mode enum defines the available GPIO modes (input, output, alternate function, analog).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE {
    INPUT,
    OUTPUT,
    ALTERNATE,
    ANALOG,
}
// The output_type enum defines the available GPIO output types (push-pull, open-drain).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUTTYPE {
    PUSHPULL,
    OPENDRAIN,
}
// The output_speed enum defines the available GPIO output speeds (low, medium, high, very high).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUTSPEED {
    LOW,
    MEDIUM,
    HIGH,
    VERYHIGH,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULL {
    NONE,
    PULLUP,
    PULLDOWN,
}

// The pin_config struct defines the configuration for a GPIO pin, including the port, pin, mode, output type, and output speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortPinConfig {
    pub port: PORT,
    pub pin: PIN,
    pub mode: MODE,
    pub output_type: OUTPUTTYPE,
    pub output_speed: OUTPUTSPEED,
    pub pull: PULL,
    pub alternate_function: Dio_AlternateFunctionType, // Added field for alternate function
}
// The port_config struct defines the configuration for a GPIO port, including an array of pin configurations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]    
pub struct PortConfig{
    pub ports: &'static [PortPinConfig]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortRegister{
    pub moder : u32,
    pub otyper : u32,
    pub ospeedr : u32,
    pub pupdr : u32,
    pub idr : u32,
    pub odr : u32,
    pub bsrr : u32,
    pub lckr : u32,
    pub afrl : u32,
    pub afrh : u32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dio_LevelType{
    LOW = 0,
    HIGH = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dio_AlternateFunctionType{
    AF0,
    AF1,
    AF2,
    AF3,
    AF4,
    AF5,
    AF6,
    AF7,
    AF8,
    AF9,
    AF10,
    AF11,
    AF12,
    AF13,
    AF14,
    AF15,
    NONE,
}
