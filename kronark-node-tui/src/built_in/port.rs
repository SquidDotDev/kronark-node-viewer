use std::collections::HashSet;

use kronark_node_parser::kronarknode::instance::Instance;
use kronark_node_parser::kronarknode::socket::SocketType;
use crate::socket_tui::{Additional, SocketDefault, SocketTui};
use crate::utils::{data_get_constant, validate_socket};

use crate::{errors::NodeConversionError, node_tui::NodeTui};

pub fn parse_port(instance: Instance) -> Result<NodeTui, NodeConversionError> {
    let mut sockets = Vec::new();

    // i/o port 

    let io_switch = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: io_switch.clone(),
        default: SocketDefault::Bool(true),
        additional: Additional::Switch { on: "input".to_string(), off: "output".to_string() },
    });

    // channel port

    let channel= if io_switch.flags.is_switch_on() {
        validate_socket(&instance, sockets.len(), SocketType::IncomingNamed)?
    } else {
        validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?
    };

    sockets.push(SocketTui{ 
        name: "channel".to_string(),
        socket: channel.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });

    // slot port 

    let slot = validate_socket(&instance, sockets.len(), SocketType::IncomingNumber)?;

    sockets.push(SocketTui{ 
        name: "slot".to_string(),
        socket: slot.clone(),
        default: SocketDefault::String("0".to_string()),
        additional: Additional::Number { minimum: "0".to_string(), maximum: "255".to_string(), step: "1".to_string() }
    });

    // name port 

    let name = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "name".to_string(),
        socket: name.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "64".to_string(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() }
    });

    // for output only

    if !io_switch.flags.is_switch_on() {
        let data = validate_socket(&instance, sockets.len(), SocketType::IncomingNamed)?;

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

    // type port

    let type_socket = validate_socket(&instance, sockets.len(), SocketType::IncomingSelect)?;

    let mut options = HashSet::new();
    options.insert("named".to_string());
    options.insert("number".to_string());
    options.insert("select".to_string());
    options.insert("switch".to_string());
    options.insert("text".to_string());

    sockets.push(SocketTui{ 
        name: "type".to_string(),
        socket: type_socket.clone(),
        default: SocketDefault::String("named".to_string()),
        additional: Additional::Select { options }
    });

    // repetition port

    let repetition = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "".to_string(),
        socket: repetition.clone(),
        default: SocketDefault::Bool(false),
        additional: Additional::Switch { on: "repetition".to_string(), off: "no repetition".to_string() }
    });

    match data_get_constant(&type_socket.data.clone()).unwrap_or("named".to_string()).as_str() {
        "named" => parse_default(instance.clone(), &mut sockets)?,
        "text" => parse_text(instance.clone(), &mut sockets)?,
        "number" => parse_number(instance.clone(), &mut sockets)?,
        "select" => parse_default(instance.clone(), &mut sockets)?,
        "switch" => parse_switch(instance.clone(), &mut sockets)?,
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

fn parse_default(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {
    // default port

    let default = validate_socket(&instance, 6, SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "default".to_string(),
        socket: default.clone(),
        default: SocketDefault::String("".to_string()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: String::new() }
    });

    // data port

    let data = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "data".to_string(),
        socket: data.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });
    
    Ok(())
}

fn parse_switch(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {

    // inactive port

    let inactive = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "inactive".to_string(),
        socket: inactive.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() }
    });

    // active port

    let active = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "active".to_string(),
        socket: active.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: "qwertyuiopasdfghjklzxcvbnm_1234567890 ".to_string() }
    });

    // default port

    let default = validate_socket(&instance, sockets.len(), SocketType::IncomingSwitch)?;

    sockets.push(SocketTui{ 
        name: "default".to_string(),
        socket: default.clone(),
        default: SocketDefault::Bool(false),
        additional: Additional::Switch { on: "on".to_string(), off: "off".to_string() },
    });

    // truth port

    let truth = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "truth".to_string(),
        socket: truth.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });
    
    Ok(())
}

fn parse_text(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {

    // minimum port

    let minimum = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "minimum".to_string(),
        socket: minimum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "4".to_string(), valid: "1234567890".to_string() }
    });

    // maximum port

    let maximum = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "maximum".to_string(),
        socket: maximum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "4".to_string(), valid: "1234567890".to_string() }
    });

    // valid port

    let valid = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "valid".to_string(),
        socket: valid.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "256".to_string(), valid: String::new() }
    });

    // default port

    let default = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "default".to_string(),
        socket: default.clone(),
        default: SocketDefault::String("".to_string()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: String::new() }
    });

    // data port

    let data = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "data".to_string(),
        socket: data.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });
    
    Ok(())
}

fn parse_number(instance: Instance, sockets: &mut Vec<SocketTui>) -> Result<(), NodeConversionError> {

    // minimum port

    let minimum = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "minimum".to_string(),
        socket: minimum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "".to_string(), valid: "-.1234567890".to_string() }
    });

    // maximum port

    let maximum = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "maximum".to_string(),
        socket: maximum.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "".to_string(), valid: "-.1234567890".to_string() }
    });

    // step port

    let step = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "step".to_string(),
        socket: step.clone(),
        default: SocketDefault::String(String::new()),
        additional: Additional::Text { minimum: "0".to_string(), maximum: "".to_string(), valid: "-.1234567890".to_string() }
    });

    // default port

    let default = validate_socket(&instance, sockets.len(), SocketType::IncomingText)?;

    sockets.push(SocketTui{ 
        name: "default".to_string(),
        socket: default.clone(),
        default: SocketDefault::String("".to_string()),
        additional: Additional::Text { minimum: String::new(), maximum: String::new(), valid: "-.1234567890".to_string() }
    });

    // data port

    let data = validate_socket(&instance, sockets.len(), SocketType::OutgoingNamed)?;

    sockets.push(SocketTui{ 
        name: "data".to_string(),
        socket: data.clone(),
        default: SocketDefault::None,
        additional: Additional::None,
    });
    
    Ok(())
}