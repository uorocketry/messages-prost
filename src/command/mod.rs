// This module will contain command-related data types and logic.

use crate::messages::{
    command_message, CommandMessage, DeployDrogue, DeployMain, Online, PowerDown, RadioRateChange,
};

impl From<DeployDrogue> for CommandMessage {
    fn from(v: DeployDrogue) -> Self {
        CommandMessage {
            command: Some(command_message::Command::DeployDrogue(v)),
        }
    }
}

impl From<DeployMain> for CommandMessage {
    fn from(v: DeployMain) -> Self {
        CommandMessage {
            command: Some(command_message::Command::DeployMain(v)),
        }
    }
}

impl From<PowerDown> for CommandMessage {
    fn from(v: PowerDown) -> Self {
        CommandMessage {
            command: Some(command_message::Command::PowerDown(v)),
        }
    }
}

impl From<RadioRateChange> for CommandMessage {
    fn from(v: RadioRateChange) -> Self {
        CommandMessage {
            command: Some(command_message::Command::RadioRateChange(v)),
        }
    }
}

impl From<Online> for CommandMessage {
    fn from(v: Online) -> Self {
        CommandMessage {
            command: Some(command_message::Command::Online(v)),
        }
    }
}
