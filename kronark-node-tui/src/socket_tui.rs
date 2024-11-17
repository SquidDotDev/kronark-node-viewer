use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypeTui {
    Output,
    Named,
    Text(String),
    Number { range: RangeInclusive<i64>, value: i64 },
    Switch { on: String, off: String, value: bool },
    Select { options: Vec<String>, value: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataTui {
    pub data_type: DataTypeTui,
    pub connection: Option<Connection>
}

impl DataTui {
    pub fn get_size(&self) -> usize {
        match &self.data_type {
            DataTypeTui::Output | DataTypeTui::Named => 0,
            DataTypeTui::Number { value, .. } => value.to_string().len(),
            DataTypeTui::Switch { on, off, value } => if *value { on.len() } else { off.len() },
            DataTypeTui::Text(s) => s.len(),
            DataTypeTui::Select { value, .. } => value.len(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SocketTuiType {
    Repetitive(Vec<DataTui>),
    Single(DataTui),
}

impl SocketTuiType {
    pub fn get_size(&self) -> usize {
        match self {
            SocketTuiType::Repetitive(v) => v.iter().fold(0, |a, d| {a.max(d.get_size())} ),
            Self::Single(d) => d.get_size(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    pub node: u8,
    pub port_index: u8
} 

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTui {
    pub name: String,
    pub socket_type: SocketTuiType,
    pub type_index: u8,
}

impl SocketTui {
    pub fn get_size(&self) -> usize {
        self.socket_type.get_size()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SocketTuiTransform {
    pub width: usize,
    pub name_width: usize,
    pub value_width: usize,
    pub y: i32,
    pub x: i32, 
}

