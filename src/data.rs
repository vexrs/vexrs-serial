use core::prelude::rust_2021::*;
use alloc::vec::Vec;

/// The units to use in an encoder tick
#[derive(Default, Debug, Copy, Clone, bincode::Decode, bincode::Encode)]
pub enum MotorEncoderUnits {
    /// The units are in degrees
    #[default] Degrees,
    /// The units are in rotations
    Rotations,
    /// The units are in ticks
    Ticks,
}


/// The break mode of a motor
#[derive(Default, Debug, Copy, Clone, bincode::Decode, bincode::Encode)]
pub enum MotorBrakeMode {
    /// The motor will coast to a stop
    #[default] Coast,
    /// The motor will brake to a stop
    Brake,
    /// The motor will attempt to hold its current position
    /// reacting to outside forces
    Hold,
}

/// The gearbox a motor uses
#[derive(Default, Debug, Copy, Clone, bincode::Decode, bincode::Encode)]
pub enum MotorGearbox {
    Red,
    #[default] Green,
    Blue
}

/// Data sent about motors by the robot brain
#[derive(Debug, Copy, Clone, bincode::Decode, bincode::Encode)]
pub struct MotorData {
    /// The motor's encoder units
    pub encoder_units: MotorEncoderUnits,
    /// The motor's current position
    pub current_position: f64,
    /// The motor's target position
    pub target_position: f64,
    /// The motor's raw position
    pub raw_position: f64,

    /// The motor's break mode
    pub break_mode: MotorBrakeMode,

    /// The motor's current velocity in RPM
    pub current_velocity: i32,
    /// The motor's target velocity in RPM
    pub target_velocity: i32,

    /// The motor's torque in N*m
    pub torque: f64,

    /// The motor's direction
    pub direction: i32,

    /// The motor's temperature in Degrees Celcius
    pub temperature: f64,
    /// True if the motor is over temp
    pub over_temp: bool,

    /// The motor's current draw in mA
    pub current_draw: i32,
    /// The motor's current limit in mA
    pub current_limit: i32,
    /// The motor's voltage in mV
    pub voltage: i32,
    /// The motor's voltage limit
    pub voltage_limit: i32,
    /// The motor's power draw in Watts
    pub power_draw: f64,
    /// True if the motor is over current
    pub over_current: bool,

    /// The motor's efficiency in %
    pub efficiency: f64,

    /// The motor's gear box
    pub gearbox: MotorGearbox,

    /// True if the motor is reversed
    pub reversed: bool,
}


/// The type of logging data sent by the brain
#[repr(u8)]
#[derive(Debug, Clone, bincode::Decode, bincode::Encode)]
pub enum LogType {
    /// A log message
    Message(Vec<u8>),
    /// Plot a new data-point from the brain
    /// on a graph with the given ID
    Plot(u32, f64),
    /// Update motor data for a motor on a given port
    UpdateMotor(u32, MotorData),
}


/// Represents the type of data being sent
#[repr(u8)]
#[derive(Debug, Clone, bincode::Decode, bincode::Encode)]
pub enum DataType {
    Print(Vec<u8>),
    Error(Vec<u8>),
    KernelLog(LogType),
}
