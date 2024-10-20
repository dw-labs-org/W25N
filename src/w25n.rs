use embedded_hal::{
    i2c::Operation,
    spi::{self},
};

use crate::{
    mem::ColumnAddress,
    registers::{Jedec, Status1, Status2, Status3},
};

pub struct W25N<SPI> {
    spi: SPI,
}

#[derive(Debug, Clone, Copy)]
enum Error<SPI>
where
    SPI: spi::SpiDevice,
{
    SPI(SPI::Error),
    WriteEnable,
    WriteDisable,
    EraseFail,
    BlockProtect(u8),
}

type WResult<T, SPI> = Result<T, Error<SPI>>;

impl<SPI> W25N<SPI>
where
    SPI: spi::SpiDevice,
{
    fn write(&mut self, buf: &[u8]) -> WResult<(), SPI> {
        self.spi.write(buf).map_err(|e| Error::SPI(e))
    }
    fn read(&mut self, buf: &mut [u8]) -> WResult<(), SPI> {
        self.spi.read(buf).map_err(|e| Error::SPI(e))
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> WResult<(), SPI> {
        self.spi.transfer(read, write).map_err(|e| Error::SPI(e))
    }

    fn transfer_in_place(&mut self, buf: &mut [u8]) -> WResult<(), SPI> {
        self.spi.transfer_in_place(buf).map_err(|e| Error::SPI(e))
    }

    fn transaction(&mut self, operations: &mut [spi::Operation<'_, u8>]) -> WResult<(), SPI> {
        self.spi.transaction(operations).map_err(|e| Error::SPI(e))
    }

    /// Send the Reset Command 0xFF
    pub fn reset(&mut self) -> WResult<(), SPI> {
        self.write(&[0xFF])
    }

    pub fn jedec(&mut self) -> WResult<Jedec, SPI> {
        let mut result = [0; 3];
        let mut ops = [
            spi::Operation::Write(&[0x9F, 0x00]),
            spi::Operation::Read(&mut result),
        ];

        self.transaction(&mut ops)?;
        Ok(result.into())
    }

    pub fn write_enable(&mut self) -> WResult<(), SPI> {
        self.write(&[0x06])?;
        if self.read_status_3()?.wel() {
            Ok(())
        } else {
            Err(Error::WriteEnable)
        }
    }

    pub fn write_disable(&mut self) -> WResult<(), SPI> {
        self.write(&[0x04])?;
        if !self.read_status_3()?.wel() {
            Ok(())
        } else {
            Err(Error::WriteDisable)
        }
    }

    pub fn wait_for_operation(&mut self) -> WResult<(), SPI> {
        while self.read_status_3()?.busy() {}
        Ok(())
    }

    pub fn read_status_1(&mut self) -> WResult<Status1, SPI> {
        let mut data = [0x05, 0xA0, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status1::from_bytes([data[2]]))
    }

    pub fn read_status_2(&mut self) -> WResult<Status2, SPI> {
        let mut data = [0x05, 0xB0, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status2::from_bytes([data[2]]))
    }

    pub fn read_status_3(&mut self) -> WResult<Status3, SPI> {
        let mut data = [0x05, 0xC0, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status3::from_bytes([data[2]]))
    }

    pub fn write_status_1(&mut self, status: Status1) -> WResult<(), SPI> {
        self.write(&[0x01, 0xA0, status.into()])
    }

    pub fn write_status_2(&mut self, status: Status2) -> WResult<(), SPI> {
        self.write(&[0x01, 0xB0, status.into()])
    }

    pub fn disable_block_protect(&mut self) -> WResult<(), SPI> {
        let status = self.read_status_1()?.with_bp(0);
        self.write_status_1(status);
        match self.read_status_1()?.bp() {
            0x0 => Ok(()),
            x => Err(Error::BlockProtect(x)),
        }
    }
    pub fn block_erase(&mut self, ca: ColumnAddress) -> WResult<(), SPI> {
        self.write_enable()?;
        self.transaction(&mut [
            spi::Operation::Write(&[0xD8]),
            spi::Operation::Write(&ca.to_array()),
        ])?;
        self.wait_for_operation()?;
        if self.read_status_3()?.e_fail() {
            Err(Error::EraseFail)
        } else {
            Ok(())
        }
    }

    pub fn load_program_data(&mut self, ca: ColumnAddress, data: &[u8]) -> Result<(), Error<SPI>> {
        self.write_enable()?;
        self.transaction(&mut [
            spi::Operation::Write(&[0x02]),
            spi::Operation::Write(&ca.to_array()),
            spi::Operation::Write(data),
        ])
    }
}
