// This module will contain command-related data types and logic.

use crate::messages::{
    command, Command, DeployDrogue, DeployMain, Online, PowerDown, RadioRateChange,
};

impl From<DeployDrogue> for Command {
    fn from(v: DeployDrogue) -> Self {
        Command {
            command: Some(command::Command::DeployDrogue(v)),
        }
    }
}

impl From<DeployMain> for Command {
    fn from(v: DeployMain) -> Self {
        Command {
            command: Some(command::Command::DeployMain(v)),
        }
    }
}

impl From<PowerDown> for Command {
    fn from(v: PowerDown) -> Self {
        Command {
            command: Some(command::Command::PowerDown(v)),
        }
    }
}

impl From<RadioRateChange> for Command {
    fn from(v: RadioRateChange) -> Self {
        Command {
            command: Some(command::Command::RadioRateChange(v)),
        }
    }
}

impl From<Online> for Command {
    fn from(v: Online) -> Self {
        Command {
            command: Some(command::Command::Online(v)),
        }
    }
}
