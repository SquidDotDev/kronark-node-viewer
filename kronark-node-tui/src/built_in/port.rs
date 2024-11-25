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
            sockets: sockets,
            key: instance.key as u8,
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
        sockets: sockets,
        key: instance.key as u8,
    };
    Ok(node)
}

fn parse_text(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {

    // minimum port

    let minimum = validate_socket(&instance, 6, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "minimum".to_string(),
        socket: minimum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "4".to_string(), valid: "123456789".to_string() }
    });

    // maximum port

    let maximum = validate_socket(&instance, 7, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "maximum".to_string(),
        socket: maximum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "4".to_string(), valid: "123456789".to_string() }
    });

    // valid port

    let valid = validate_socket(&instance, 8, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "valid".to_string(),
        socket: valid.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "256".to_string(), valid: String::new() }
    });

    // default port

    let default = validate_socket(&instance, 9, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "default".to_string(),
        socket: default.clone(),
        default: SocketDefault::String("".to_string()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: String::new() }
    });
    
    Ok(())
}
