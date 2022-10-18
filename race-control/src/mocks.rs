use std::io::{Read, Result, Write};

pub struct DummySerialPort {

}

impl DummySerialPort {
    pub fn new() -> Self {
        DummySerialPort {}
    }
}

impl Read for DummySerialPort {
    fn read(&mut self, _: &mut [u8]) -> Result<usize> { todo!() }
}

impl Write for DummySerialPort {
    fn write(&mut self, _: &[u8]) -> Result<usize> { todo!() }
    fn flush(&mut self) -> Result<()> { todo!() }
}

