use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/sensor.proto"], &["src/"])?;
    Ok(())
}
