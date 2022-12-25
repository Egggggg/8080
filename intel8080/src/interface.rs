pub(crate) struct DataBus {}
pub(crate) struct AddressBus {}
pub(crate) struct ControlBus {
    pub latch: u8,
}

pub(crate) trait Bus {
    type Data;

    fn send(&self, data: Self::Data);
}
