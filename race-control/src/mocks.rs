use std::io::{Read, Result, Write};
use std::time::Duration;

use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};

pub struct DummySerialPort {}

impl DummySerialPort {
    pub fn new() -> Self {
        DummySerialPort {}
    }
}

impl Read for DummySerialPort {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        todo!()
    }
}

impl Write for DummySerialPort {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}

impl SerialPort for DummySerialPort {
    fn name(&self) -> Option<String> {
        todo!()
    }

    fn baud_rate(&self) -> serialport::Result<u32> {
        todo!()
    }

    fn data_bits(&self) -> serialport::Result<DataBits> {
        todo!()
    }

    fn flow_control(&self) -> serialport::Result<FlowControl> {
        todo!()
    }

    fn parity(&self) -> serialport::Result<Parity> {
        todo!()
    }

    fn stop_bits(&self) -> serialport::Result<StopBits> {
        todo!()
    }

    fn timeout(&self) -> Duration {
        todo!()
    }

    fn set_baud_rate(&mut self, _baud_rate: u32) -> serialport::Result<()> {
        todo!()
    }

    fn set_data_bits(&mut self, _data_bits: DataBits) -> serialport::Result<()> {
        todo!()
    }

    fn set_flow_control(&mut self, _flow_control: FlowControl) -> serialport::Result<()> {
        todo!()
    }

    fn set_parity(&mut self, _parity: Parity) -> serialport::Result<()> {
        todo!()
    }

    fn set_stop_bits(&mut self, _stop_bits: StopBits) -> serialport::Result<()> {
        todo!()
    }

    fn set_timeout(&mut self, _timeout: Duration) -> serialport::Result<()> {
        todo!()
    }

    fn write_request_to_send(&mut self, _level: bool) -> serialport::Result<()> {
        todo!()
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> serialport::Result<()> {
        todo!()
    }

    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn bytes_to_read(&self) -> serialport::Result<u32> {
        todo!()
    }

    fn bytes_to_write(&self) -> serialport::Result<u32> {
        todo!()
    }

    fn clear(&self, _buffer_to_clear: ClearBuffer) -> serialport::Result<()> {
        todo!()
    }

    fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
        todo!()
    }

    fn set_break(&self) -> serialport::Result<()> {
        todo!()
    }

    fn clear_break(&self) -> serialport::Result<()> {
        todo!()
    }
}
