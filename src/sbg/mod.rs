// This module will contain SBG-related data types and logic.

use crate::messages::{
    sbg_data, Air, AirStatus, ClockStatus, EkfNav, EkfQuat, EkfSolutionMode, EkfStatus, GpsPos,
    GpsPositionStatus, GpsPositionStatusE, GpsPositionType, GpsVel, GpsVelStatus, GpsVelStatusE,
    GpsVelType, Imu, ImuStatus, SbgData, UtcStatus, UtcTime, UtcTimeStatus,
};

// SbgData constructors from inner types
impl From<UtcTime> for SbgData {
    fn from(v: UtcTime) -> Self {
        SbgData {
            data: Some(sbg_data::Data::UtcTime(v)),
        }
    }
}
impl From<Air> for SbgData {
    fn from(v: Air) -> Self {
        SbgData {
            data: Some(sbg_data::Data::Air(v)),
        }
    }
}
impl From<EkfQuat> for SbgData {
    fn from(v: EkfQuat) -> Self {
        SbgData {
            data: Some(sbg_data::Data::EkfQuat(v)),
        }
    }
}
impl From<EkfNav> for SbgData {
    fn from(v: EkfNav) -> Self {
        SbgData {
            data: Some(sbg_data::Data::EkfNav(v)),
        }
    }
}
impl From<Imu> for SbgData {
    fn from(v: Imu) -> Self {
        SbgData {
            data: Some(sbg_data::Data::Imu(v)),
        }
    }
}
impl From<GpsVel> for SbgData {
    fn from(v: GpsVel) -> Self {
        SbgData {
            data: Some(sbg_data::Data::GpsVel(v)),
        }
    }
}
impl From<GpsPos> for SbgData {
    fn from(v: GpsPos) -> Self {
        SbgData {
            data: Some(sbg_data::Data::GpsPos(v)),
        }
    }
}

// Status conversions
impl EkfStatus {
    pub fn from_u32(status: u32) -> Self {
        let solution_mode = match status & 0x0000000F {
            0 => EkfSolutionMode::Uninitialized,
            1 => EkfSolutionMode::VerticalGyro,
            2 => EkfSolutionMode::Ahrs,
            3 => EkfSolutionMode::NavVelocity,
            4 => EkfSolutionMode::NavPosition,
            _ => EkfSolutionMode::Uninitialized,
        };

        Self {
            solution_mode: solution_mode.into(),
            attitude_valid: (status & (1 << 4)) != 0,
            heading_valid: (status & (1 << 5)) != 0,
            velocity_valid: (status & (1 << 6)) != 0,
            position_valid: (status & (1 << 7)) != 0,
            vert_ref_used: (status & (1 << 8)) != 0,
            mag_ref_used: (status & (1 << 9)) != 0,
            gps1_vel_used: (status & (1 << 10)) != 0,
            gps1_pos_used: (status & (1 << 11)) != 0,
            gps1_hdt_used: (status & (1 << 13)) != 0,
            gps2_vel_used: (status & (1 << 14)) != 0,
            gps2_pos_used: (status & (1 << 15)) != 0,
            gps2_hdt_used: (status & (1 << 17)) != 0,
            odo_used: (status & (1 << 18)) != 0,
            dvl_bt_used: (status & (1 << 19)) != 0,
            dvl_wt_used: (status & (1 << 20)) != 0,
            user_pos_used: (status & (1 << 21)) != 0,
            user_vel_used: (status & (1 << 22)) != 0,
            user_heading_used: (status & (1 << 23)) != 0,
            usbl_used: (status & (1 << 24)) != 0,
            air_data_used: (status & (1 << 25)) != 0,
            zupt_used: (status & (1 << 26)) != 0,
            align_valid: (status & (1 << 27)) != 0,
            depth_used: (status & (1 << 28)) != 0,
        }
    }
}

impl UtcTimeStatus {
    pub fn from_u16(status: u16) -> Self {
        let clock_status = match (status >> 1) & 0x000F {
            0 => ClockStatus::ClockError,
            1 => ClockStatus::FreeRunning,
            2 => ClockStatus::Steering,
            3 => ClockStatus::ClockValid,
            _ => ClockStatus::ClockError,
        };

        let utc_status = match (status >> 6) & 0x000F {
            0 => UtcStatus::UtcInvalid,
            1 => UtcStatus::NoLeapSec,
            2 => UtcStatus::UtcValid,
            _ => UtcStatus::UtcInvalid,
        };

        Self {
            clock_status: clock_status.into(),
            utc_status: utc_status.into(),
        }
    }
}

impl AirStatus {
    pub fn from_u16(status: u16) -> Self {
        Self {
            time_is_delay: (status & (1 << 0)) != 0,
            pressure_abs_valid: (status & (1 << 1)) != 0,
            altitude_valid: (status & (1 << 2)) != 0,
            pressure_diff_valid: (status & (1 << 3)) != 0,
            airspeed_valid: (status & (1 << 4)) != 0,
            temperature_valid: (status & (1 << 5)) != 0,
        }
    }
}

impl ImuStatus {
    pub fn from_u16(status: u16) -> Self {
        Self {
            com_ok: (status & (1 << 0)) != 0,
            status_test_pass: (status & (1 << 1)) != 0,
            accel_x_test_pass: (status & (1 << 2)) != 0,
            accel_y_test_pass: (status & (1 << 3)) != 0,
            accel_z_test_pass: (status & (1 << 4)) != 0,
            gyro_x_test_pass: (status & (1 << 5)) != 0,
            gyro_y_test_pass: (status & (1 << 6)) != 0,
            gyro_z_test_pass: (status & (1 << 7)) != 0,
            accels_in_range: (status & (1 << 8)) != 0,
            gyros_in_range: (status & (1 << 9)) != 0,
        }
    }
}

impl GpsPositionStatus {
    pub fn from_u32(status: u32) -> Self {
        let status_e = match status & 0x000F {
            0 => GpsPositionStatusE::SolComputed,
            1 => GpsPositionStatusE::InsufficientObs,
            2 => GpsPositionStatusE::GpsInternalError,
            3 => GpsPositionStatusE::HeightLimit,
            _ => GpsPositionStatusE::GpsInternalError,
        };

        let type_e = match (status >> 4) & 0x000F {
            0 => GpsPositionType::NoSolution,
            1 => GpsPositionType::UnknownType,
            2 => GpsPositionType::Single,
            3 => GpsPositionType::PseudoRangeDiff,
            4 => GpsPositionType::Sbas,
            5 => GpsPositionType::OmniStar,
            6 => GpsPositionType::RtkFloat,
            7 => GpsPositionType::RtkInt,
            8 => GpsPositionType::PppFloat,
            9 => GpsPositionType::PppInt,
            10 => GpsPositionType::Fixed,
            _ => GpsPositionType::UnknownType,
        };

        Self {
            status: status_e.into(),
            r#type: type_e.into(),
        }
    }
}

impl GpsVelStatus {
    pub fn from_u32(status: u32) -> Self {
        let status_e = match status & 0x000F {
            0 => GpsVelStatusE::VelSolComputed,
            1 => GpsVelStatusE::VelInsufficientObs,
            2 => GpsVelStatusE::VelInternalError,
            3 => GpsVelStatusE::VelLimit,
            _ => GpsVelStatusE::VelInternalError,
        };

        let type_e = match (status >> 4) & 0x000F {
            0 => GpsVelType::VelNoSolution,
            1 => GpsVelType::VelUnknownType,
            2 => GpsVelType::Doppler,
            3 => GpsVelType::Differential,
            _ => GpsVelType::VelUnknownType,
        };

        Self {
            status: status_e.into(),
            r#type: type_e.into(),
        }
    }
}
