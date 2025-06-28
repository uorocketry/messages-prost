// This module will contain common data types and logic.

use crate::messages::Node;

impl From<Node> for u16 {
    fn from(node: Node) -> Self {
        node as i32 as u16
    }
}
