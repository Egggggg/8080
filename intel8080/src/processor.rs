use crate::cpu::*;
use crate::interface::*;

pub struct Processor {
    cpu: Cpu,
    data_bus: DataBus,
    address_bus: AddressBus,
    control_bus: ControlBus,
    memory: [u8; 65536],
    io_ports: [u8; 256],
}

impl Processor {
    pub fn new() -> Self {
        env_logger::init();

        Self {
            cpu: Cpu::new(),
            data_bus: DataBus {},
            address_bus: AddressBus {},
            control_bus: ControlBus { latch: 0 },
            memory: [0; 65536],
            io_ports: [0; 256],
        }
    }
}
