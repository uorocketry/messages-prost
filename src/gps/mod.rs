// This module will contain GPS-related data types and logic.

use crate::messages::Gps;
use ublox::PacketRef;

macro_rules! to_gps_from_packet {
    ($packet:expr, $name:expr) => {{
        let p = $packet;
        Gps {
            message_type: $name.into(),
            data: p.as_bytes().into(),
        }
    }};
}

pub fn gps_from_ublox(packet: &PacketRef) -> Gps {
    match packet {
        PacketRef::NavPosLlh(p) => to_gps_from_packet!(p, "NavPosLlh"),
        PacketRef::NavStatus(p) => to_gps_from_packet!(p, "NavStatus"),
        PacketRef::NavDop(p) => to_gps_from_packet!(p, "NavDop"),
        PacketRef::NavPvt(p) => to_gps_from_packet!(p, "NavPvt"),
        PacketRef::NavSolution(p) => to_gps_from_packet!(p, "NavSolution"),
        PacketRef::NavVelNed(p) => to_gps_from_packet!(p, "NavVelNed"),
        PacketRef::NavHpPosLlh(p) => to_gps_from_packet!(p, "NavHpPosLlh"),
        PacketRef::NavHpPosEcef(p) => to_gps_from_packet!(p, "NavHpPosEcef"),
        PacketRef::NavTimeUTC(p) => to_gps_from_packet!(p, "NavTimeUTC"),
        PacketRef::NavTimeLs(p) => to_gps_from_packet!(p, "NavTimeLs"),
        PacketRef::NavSat(p) => to_gps_from_packet!(p, "NavSat"),
        PacketRef::NavEoe(p) => to_gps_from_packet!(p, "NavEoe"),
        PacketRef::NavOdo(p) => to_gps_from_packet!(p, "NavOdo"),
        PacketRef::CfgOdo(p) => to_gps_from_packet!(p, "CfgOdo"),
        PacketRef::MgaAck(p) => to_gps_from_packet!(p, "MgaAck"),
        PacketRef::MgaGpsIono(p) => to_gps_from_packet!(p, "MgaGpsIono"),
        PacketRef::MgaGpsEph(p) => to_gps_from_packet!(p, "MgaGpsEph"),
        PacketRef::MgaGloEph(p) => to_gps_from_packet!(p, "MgaGloEph"),
        PacketRef::AlpSrv(p) => to_gps_from_packet!(p, "AlpSrv"),
        PacketRef::AckAck(p) => to_gps_from_packet!(p, "AckAck"),
        PacketRef::AckNak(p) => to_gps_from_packet!(p, "AckNak"),
        PacketRef::CfgItfm(p) => to_gps_from_packet!(p, "CfgItfm"),
        PacketRef::CfgPrtI2c(p) => to_gps_from_packet!(p, "CfgPrtI2c"),
        PacketRef::CfgPrtSpi(p) => to_gps_from_packet!(p, "CfgPrtSpi"),
        PacketRef::CfgPrtUart(p) => to_gps_from_packet!(p, "CfgPrtUart"),
        PacketRef::CfgNav5(p) => to_gps_from_packet!(p, "CfgNav5"),
        PacketRef::CfgAnt(p) => to_gps_from_packet!(p, "CfgAnt"),
        PacketRef::CfgTmode2(p) => to_gps_from_packet!(p, "CfgTmode2"),
        PacketRef::CfgTmode3(p) => to_gps_from_packet!(p, "CfgTmode3"),
        PacketRef::CfgTp5(p) => to_gps_from_packet!(p, "CfgTp5"),
        PacketRef::InfError(p) => to_gps_from_packet!(p, "InfError"),
        PacketRef::InfWarning(p) => to_gps_from_packet!(p, "InfWarning"),
        PacketRef::InfNotice(p) => to_gps_from_packet!(p, "InfNotice"),
        PacketRef::InfTest(p) => to_gps_from_packet!(p, "InfTest"),
        PacketRef::InfDebug(p) => to_gps_from_packet!(p, "InfDebug"),
        PacketRef::RxmRawx(p) => to_gps_from_packet!(p, "RxmRawx"),
        PacketRef::TimTp(p) => to_gps_from_packet!(p, "TimTp"),
        PacketRef::TimTm2(p) => to_gps_from_packet!(p, "TimTm2"),
        PacketRef::MonVer(p) => to_gps_from_packet!(p, "MonVer"),
        PacketRef::MonGnss(p) => to_gps_from_packet!(p, "MonGnss"),
        PacketRef::MonHw(p) => to_gps_from_packet!(p, "MonHw"),
        PacketRef::RxmRtcm(p) => to_gps_from_packet!(p, "RxmRtcm"),
        PacketRef::EsfMeas(p) => to_gps_from_packet!(p, "EsfMeas"),
        PacketRef::EsfIns(p) => to_gps_from_packet!(p, "EsfIns"),
        PacketRef::HnrAtt(p) => to_gps_from_packet!(p, "HnrAtt"),
        PacketRef::HnrIns(p) => to_gps_from_packet!(p, "HnrIns"),
        PacketRef::HnrPvt(p) => to_gps_from_packet!(p, "HnrPvt"),
        PacketRef::NavAtt(p) => to_gps_from_packet!(p, "NavAtt"),
        PacketRef::NavClock(p) => to_gps_from_packet!(p, "NavClock"),
        PacketRef::NavVelECEF(p) => to_gps_from_packet!(p, "NavVelECEF"),
        PacketRef::MgaGpsEPH(p) => to_gps_from_packet!(p, "MgaGpsEPH"),
        PacketRef::RxmSfrbx(p) => to_gps_from_packet!(p, "RxmSfrbx"),
        PacketRef::EsfRaw(p) => to_gps_from_packet!(p, "EsfRaw"),
        PacketRef::TimSvin(p) => to_gps_from_packet!(p, "TimSvin"),
        PacketRef::SecUniqId(p) => to_gps_from_packet!(p, "SecUniqId"),
        PacketRef::Unknown(_) => Gps {
            message_type: "Unknown".into(),
            data: Default::default(),
        },
    }
}
