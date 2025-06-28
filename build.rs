fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.btree_map(&["."]);
    config
        .compile_protos(
            &[
                "main.proto",
                "common.proto",
                "command.proto",
                "sbg.proto",
                "gps.proto",
                "logging.proto",
            ],
            &["protos/"],
        )
        .unwrap();
}
