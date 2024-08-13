use gdnative::prelude::*;
use serialport::prelude::*;
use std::time::Duration;
use std::io::Read;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SerialReader {
    port_name: String,
    baud_rate: u32,
}

#[methods]
impl SerialReader {
    fn new(_owner: &Node) -> Self {
        SerialReader {
            port_name: String::new(),
            baud_rate: 9600,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("SerialReader is ready!");
    }

    #[export]
    fn set_port_name(&mut self, _owner: &Node, port_name: String) {
        self.port_name = port_name;
    }

    #[export]
    fn set_baud_rate(&mut self, _owner: &Node, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }

    #[export]
    fn read_serial_data(&self, _owner: &Node) -> GodotString {
        let mut serial_data = String::new();
        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(1000))
            .open();

        match port {
            Ok(mut port) => {
                port.read_to_string(&mut serial_data).expect("Failed to read from serial port");
                serial_data.into()
            }
            Err(e) => {
                godot_error!("Failed to open serial port: {:?}", e);
                "".into()
            }
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<SerialReader>();
}

godot_init!(init);
