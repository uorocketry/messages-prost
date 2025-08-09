use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.btree_map(&["."]);

    config
        .compile_protos(
            &[
                "src/log.proto",
                "src/gps.proto",
                "src/sbg.proto",
                "src/common.proto",
                "src/state.proto",
                "src/sensor/madgwick.proto",
                "src/sensor/iim20670.proto",
                "src/command.proto",
                "src/radio.proto",
            ],
            &["src/"],
        )
        .unwrap();

    Ok(())
}
