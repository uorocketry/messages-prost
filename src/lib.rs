#![no_std]

pub mod messages_prost {
    pub mod sbg {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.sbg.rs"));
    }

    pub mod gps {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.gps.rs"));
    }

    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.common.rs"));
    }

    pub mod state {
        include!(concat!(env!("OUT_DIR"), "/messages_prost.state.rs"));
    }
}
