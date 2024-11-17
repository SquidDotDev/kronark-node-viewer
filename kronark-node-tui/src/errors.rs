#[derive(Debug, PartialEq, Clone)]
pub enum NodeConversionError{
    NodeVersionNotSupported,
    UnknownNodeType,
}