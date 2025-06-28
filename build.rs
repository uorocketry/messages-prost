fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.btree_map(&["."]);
    config.file_descriptor_set_path(out_dir.join("file_descriptor_set.bin"));
    config
        .compile_protos(&["protos/all.proto"], &["."])
        .unwrap();
}
