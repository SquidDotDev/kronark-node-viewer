use kronark_node_parser::kronarknode::socket::Socket;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeConversionError{
    NodeVersionNotSupported,
    UnknownNodeType(usize),
    InvalidSocketCount(usize),
    InvalidSocketType(String),
}