use std::io::Result;

/// All events need to implement both the [`core::fmt::Display`] trait and the [`defmt::Format`]
/// trait. Since they are both similar to implement, this macro helps with this task.
macro_rules! display_event {
    ($( [$e:ident, $s:literal$(,)? $( $p:ident ),* ] ),*) => {
        impl core::fmt::Display for Event {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                match self {
                    $ (
                        Event::$e($($p),*) => write!(f, $s, $($p),*),
                    )*
                }
            }
        }

        impl defmt::Format      for Event {
            fn format(&self, f: defmt::Formatter) {
                match self {
                    $ (
                        Event::$e($($p),*) => defmt::write!(f, $s, $($p),*),
                    )*
                }
            }
        }
    };
}

/// All errors need to implement both the [`core::fmt::Display`] trait and the [`defmt::Format`]
/// trait. Since they are both similar to implement, this macro helps with this task.
macro_rules! display_context {
    ($( [$e:ident, $s:literal] ),*) => {
        impl core::fmt::Display for ErrorContext {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                match self {
                    $ (
                        ErrorContext::$e => write!(f, $s),
                    )*
                }
            }
        }

        impl defmt::Format for ErrorContext {
            fn format(&self, f: defmt::Formatter) {
                match self {
                    $ (
                        ErrorContext::$e => defmt::write!(f, $s),
                    )*
                }
            }
        }
    };
}


fn main() -> Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
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
