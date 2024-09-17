//! The edge is the connection between two nodes.

use super::node::Node;

//--------------------------------------------------------------------------------------------------
// Type
//--------------------------------------------------------------------------------------------------

/// The edge is the connection between two nodes.
pub trait Edge {
    /// The out node is the node that the edge is coming from.
    fn out_node(&self) -> Box<dyn Node>;

    /// The in node is the node that the edge is going to.
    fn in_node(&self) -> Box<dyn Node>;
}
