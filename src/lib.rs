#![no_std]

pub mod sensor {
    use crate::common;

    pub mod sbg {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.sbg.rs"));
    }

    pub mod gps {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.gps.rs"));
    }

    pub mod madgwick {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.madgwick.rs"));
    }
}


pub mod common {
    include!(concat!(env!("OUT_DIR"), "/messages_prost.common.rs"));
}

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/messages_prost.state.rs"));
}

pub mod log {
    include!(concat!(env!("OUT_DIR"), "/messages_prost.log.rs"));
}

pub use mavlink;