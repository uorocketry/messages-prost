#![no_std]

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
