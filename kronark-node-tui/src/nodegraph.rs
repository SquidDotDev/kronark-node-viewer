use crate::node_tui::NodeTui;

#[derive(Debug, PartialEq, Clone)]
pub struct InternalGraph {
    nodes: Vec<NodeTui>,
    input: (i32, i32),
    output: (i32, i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeGraph {
    internal: InternalGraph,
    external: NodeTui
}

