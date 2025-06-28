#![no_std]

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub mod command;
pub mod common;
pub mod gps;
pub mod logging;
pub mod sbg;
