use std::collections::HashSet;

use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::{DataType, SocketType};
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};
use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_type(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // channel port

    let channel = validate_socket(&instance, sockets.len(), SocketType::IncomingNamed)?;

    sockets.push(SocketTui{ 
        name: "channel".to_string(),
        socket: channel.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
    });

    // default port 

    let default = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: default.clone(),
        default: SocketDefault::Bool(true),
        additional: Additional::Switch { on: "default".to_string(), off: "not default".to_string() },
        connective: false,
    });

    // built_in port 

    let built_in = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: built_in.clone(),
        default: SocketDefault::Bool(true),
        additional: Additional::Switch { on: "built in".to_string(), off: "not built in".to_string() },
        connective: false,
    });

    // type port

    let type_socket = validate_socket(&instance, sockets.len(), SocketType::IncomingSelect)?;

    let mut options = HashSet::new();
    options.insert("compiler/any".to_string());
    options.insert("compiler/number".to_string());
    options.insert("compiler/select".to_string());
    options.insert("compiler/truth".to_string());
    options.insert("compiler/text".to_string());
    options.insert("compiler/bytes".to_string());

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: type_socket.clone(),
        default: SocketDefault::String("compiler/any".to_string()),
        additional: Additional::Select { options },
        connective: false,
    });

    if let Some(DataType::Connection(_, socket)) =  channel.data {
        if socket != 1 {
            // connective port 

            let connective = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

            sockets.push(SocketTui{ 
                name: "".to_string(),
                socket: connective.clone(),
                default: SocketDefault::Bool(true),
                additional: Additional::Switch { on: "connective".to_string(), off: "not connective".to_string() },
                connective: false,
            });
        }
    }
    
    // data port
    
    let data = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;
    

    sockets.push(SocketTui{ 
        name: "data".to_string(),
        socket: data.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
        connective: true,
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
    Ok(node)
}