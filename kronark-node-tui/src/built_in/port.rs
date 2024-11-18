use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::{DataType, SocketType};
use crate::socket_tui::{Connection, DataTui, DataTypeTui, SocketTui, SocketTuiType};
use crate::utils::{data_get_constant, validate_socket};

use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_port(instance: Instance) -> Result<NodeTui, NodeConversionError> {

    let mut sockets = Vec::new();
    let io_switch = validate_socket(&instance, 0, SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket_type: SocketTuiType::Single(DataTui {
            data_type: DataTypeTui::Switch {
                on: "input".to_string(),
                off: "output".to_string(),
                value: io_switch.flags.is_switch_on() },
            connection: None }),
        type_index: io_switch.type_index as u8,
    });

    let channel= if io_switch.flags.is_switch_on() {
        validate_socket(&instance, 1, SocketType::IncomingNamed)?
    } else {
        validate_socket(&instance, 1, SocketType::OutgoingNamed)?
    };

    let channel_connection = Connection::from_data(channel.data.clone());

    sockets.push(SocketTui{ 
        name: "channel".to_string(),
        socket_type: SocketTuiType::Single(DataTui {
            data_type: if  io_switch.flags.is_switch_on() {DataTypeTui::Named} else {DataTypeTui::Output},
            connection: channel_connection }),
        type_index: io_switch.type_index as u8,
    });

    let slot = validate_socket(&instance, 2, SocketType::IncomingNumber)?;

    sockets.push(SocketTui{ 
        name: "slot".to_string(),
        socket_type: SocketTuiType::Single(DataTui {
            data_type: DataTypeTui::Number { range: 0..=255, value: data_get_constant(slot.data.clone()).unwrap_or("0".to_string()).parse().unwrap_or(0) },
            connection: None }),
        type_index: io_switch.type_index as u8,
    });

    let name = validate_socket(&instance, 3, SocketType::IncomingText)?;

    let name_connection = Connection::from_data(channel.data.clone());

    sockets.push(SocketTui{ 
        name: "name".to_string(),
        socket_type: SocketTuiType::Single(DataTui {
            data_type: DataTypeTui::Text(data_get_constant(name.data.clone()).unwrap_or("".to_string())),
            connection: name_connection }),
        type_index: io_switch.type_index as u8,
    });

    if !io_switch.flags.is_switch_on() {
        let data = validate_socket(&instance, 4, SocketType::IncomingNamed)?;

        let data_connection = Connection::from_data(channel.data.clone());

        sockets.push(SocketTui{ 
            name: "data".to_string(),
            socket_type: SocketTuiType::Single(DataTui {
                data_type: DataTypeTui::Named,
                connection: data_connection }),
            type_index: io_switch.type_index as u8,
        });
    }

    let node = NodeTui {
        name: instance.name,
        x: instance.position_x as i32,
        y: instance.position_y as i32,
        color: ratatui::style::Color::DarkGray,
        type_index: instance.node_type as u8,
        sockets: sockets
    };
    Ok(node)
}

