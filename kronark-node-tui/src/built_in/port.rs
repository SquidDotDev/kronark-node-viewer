use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::{DataType, SocketType};
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};

use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_port(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // i/o socket 

    let io_switch = validate_socket(&instance, 0, SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: io_switch.clone(),
        default: SocketDefault::Bool(true),
        additional: Additional::Switch { on: "input".to_string(), off: "output".to_string() },
    });

    // channel socket

    let channel= if io_switch.flags.is_switch_on() {
        validate_socket(&instance, 1, SocketType::IncomingNamed)?
    } else {
        validate_socket(&instance, 1, SocketType::OutgoingNamed)?
    };

    sockets.push(SocketTui{ 
        name: "channel".to_string(),
        socket: channel.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });

    // slot socket 

    let slot = validate_socket(&instance, 2, SocketType::IncomingNumber)?;

    sockets.push(SocketTui{ 
        name: "slot".to_string(),
        socket: slot.clone(),
        default: SocketDefault::String("0".to_string()),
        additional: Additional::Number { minimum: "0".to_string(), maximum: "255".to_string(), step: "1".to_string() }
    });

    // name socket 

    let name = validate_socket(&instance, 3, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "name".to_string(),
        socket: name.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "64".to_string(), valid: "qwertyuiopasdfghjklzxcvbnm_123456789".to_string() }
    });

    // for output only

    if !io_switch.flags.is_switch_on() {
        let data = validate_socket(&instance, 4, SocketType::IncomingNamed)?;

        sockets.push(SocketTui{ 
            name: "data".to_string(),
            socket: data.clone(),
            default: SocketDefault::None,
            additional: Additional::None,
        });

        let node = NodeTui {
            name: instance.name,
            x: instance.position_x as i32,
            y: instance.position_y as i32,
            color: ratatui::style::Color::DarkGray,
            type_index: instance.node_type as u8,
            sockets: sockets
        };
        return Ok(node);
    }

    // type socket

    let type_socket = validate_socket(&instance, 4, SocketType::IncomingSelect)?;

    let options = vec!["named".to_string(), "number".to_string(), "select".to_string(), "switch".to_string(), "text".to_string()];

    sockets.push(SocketTui{ 
        name: "type".to_string(),
        socket: type_socket.clone(),
        default: SocketDefault::String("compiler/any".to_string()),
        additional: Additional::Select { options }
    });

    // repetition socket

    let repetition = validate_socket(&instance, 5, SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: repetition.clone(),
        default: SocketDefault::Bool(false),
        additional: Additional::Switch { on: "repetition".to_string(), off: "no repetition".to_string() }
    });

    match data_get_constant(&type_socket.data.clone()).unwrap_or("named".to_string()).as_str() {
        "named" => (),
        "text" => parse_text(instance.clone(), &mut sockets)?,
        _ => ()
    };

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

fn parse_text(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {
    // let maximum = validate_socket(&instance, 6, SocketType::IncomingNumber)?;
    // let maximum_connection = Connection::from_data(maximum.data.clone());

    // sockets.push(SocketTui{ 
    //     name: "maximum".to_string(),
    //     socket_type: SocketTuiType::Single(DataTui {
    //         data_type: DataTypeTui::Number { range: 0..=i64::MAX, value: data_get_constant(maximum.data.clone()).unwrap_or(String::new()).parse().unwrap_or(i64::MAX), default: i64::MAX },
    //         connection: maximum_connection }),
    //     type_index: maximum.type_index as u8,
    // });

    // let minimum = validate_socket(&instance, 7, SocketType::IncomingNumber)?;
    // let minimum_connection = Connection::from_data(minimum.data.clone());

    // sockets.push(SocketTui{ 
    //     name: "minimum".to_string(),
    //     socket_type: SocketTuiType::Single(DataTui {
    //         data_type: DataTypeTui::Number { range: 0..=i64::MAX, value: data_get_constant(minimum.data.clone()).unwrap_or(String::new()).parse().unwrap_or(0), default: 0 },
    //         connection: minimum_connection }),
    //     type_index: minimum.type_index as u8,
    // });

    // let valid = validate_socket(&instance, 8, SocketType::IncomingText)?;
    // let valid_connection = Connection::from_data(valid.data.clone());

    // sockets.push(SocketTui{ 
    //     name: "valid".to_string(),
    //     socket_type: SocketTuiType::Single(DataTui {
    //         data_type: DataTypeTui::Text{value: data_get_constant(valid.data.clone()).unwrap_or("".to_string()), range: 0..=i64::MAX, valid: String::new(), default: String::new()},
    //         connection: valid_connection }),
    //     type_index: valid.type_index as u8,
    // });

    // let default = validate_socket(&instance, 9, SocketType::IncomingText)?;
    // let default_connection = Connection::from_data(default.data.clone());

    // sockets.push(SocketTui{ 
    //     name: "default".to_string(),
    //     socket_type: SocketTuiType::Single(DataTui {
    //         data_type: DataTypeTui::Text{value: data_get_constant(default.data.clone()).unwrap_or("".to_string()), range: 0..=i64::MAX, valid: String::new(), default: String::new()},
    //         connection: default_connection }),
    //     type_index: default.type_index as u8,
    // });
    
    Ok(())
}
