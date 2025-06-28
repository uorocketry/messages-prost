#![no_std]

pub mod messages_prost {
    pub mod sensor {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.sensor.rs"));
    }
}