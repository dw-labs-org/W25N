use embedded_hal::{
    i2c::Operation,
    spi::{self},
};

use crate::{
    commands::{
        BLOCK_ERASE, DEEP_POWER_DOWN, JEDEC, PAGE_DATA_READ, PROGRAM_DATA_LOAD, PROGRAM_EXECUTE,
        RANDOM_PROGRAM_DATA_LOAD, READ, READ_REG, RELEASE_POWER_DOWN, RESET, STATUS_REGISTER_1,
        STATUS_REGISTER_2, STATUS_REGISTER_3, WRITE_DISABLE, WRITE_ENABLE, WRITE_REG,
    },
    mem::{ColumnAddress, PageAddress},
    registers::{Jedec, Status1, Status2, Status3},
};

pub struct W25N<SPI> {
    spi: SPI,
}

#[derive(Debug, Clone, Copy)]
pub enum Error<SPI>
where
    SPI: spi::SpiDevice,
{
    SPI(SPI::Error),
    WriteEnable,
    WriteDisable,
    EraseFailure,
    ProgramFailure,
    BlockProtect(u8),
}

type WResult<T, SPI> = Result<T, Error<SPI>>;

impl<SPI> W25N<SPI>
where
    SPI: spi::SpiDevice,
{
    // Wrappers around SPI that map errors ===============>
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
    // <================

    /// Send the Reset Command
    pub fn reset(&mut self) -> WResult<(), SPI> {
        self.write(&[RESET])
    }

    /// Return the JEDEC id of the device
    pub fn jedec(&mut self) -> WResult<Jedec, SPI> {
        let mut result = [0; 3];
        let mut ops = [
            spi::Operation::Write(&[JEDEC, 0x00]),
            spi::Operation::Read(&mut result),
        ];

        self.transaction(&mut ops)?;
        Ok(result.into())
    }

    /// Send the write enable command, check it sets WE-L flag
    pub fn write_enable(&mut self) -> WResult<(), SPI> {
        self.write(&[WRITE_ENABLE])?;
        if self.read_status_3()?.wel() {
            Ok(())
        } else {
            Err(Error::WriteEnable)
        }
    }

    /// Send the write disable command, check it clears WE-L flag
    pub fn write_disable(&mut self) -> WResult<(), SPI> {
        self.write(&[WRITE_DISABLE])?;
        if !self.read_status_3()?.wel() {
            Ok(())
        } else {
            Err(Error::WriteDisable)
        }
    }

    /// Wait until the busy flag is cleared
    pub fn wait_for_operation(&mut self) -> WResult<(), SPI> {
        while self.read_status_3()?.busy() {}
        Ok(())
    }

    /// Read the protection register
    pub fn read_status_1(&mut self) -> WResult<Status1, SPI> {
        let mut data = [READ_REG, STATUS_REGISTER_1, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status1::from_bytes([data[2]]))
    }

    /// Read the configuration register
    pub fn read_status_2(&mut self) -> WResult<Status2, SPI> {
        let mut data = [READ_REG, STATUS_REGISTER_2, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status2::from_bytes([data[2]]))
    }

    /// Read the status register
    pub fn read_status_3(&mut self) -> WResult<Status3, SPI> {
        let mut data = [READ_REG, STATUS_REGISTER_3, 0x00];
        self.transfer_in_place(&mut data)?;
        Ok(Status3::from_bytes([data[2]]))
    }

    /// Write to the Protection register
    pub fn write_status_1(&mut self, status: Status1) -> WResult<(), SPI> {
        self.write(&[WRITE_REG, STATUS_REGISTER_1, status.into()])
    }

    /// Write to the configuration register
    pub fn write_status_2(&mut self, status: Status2) -> WResult<(), SPI> {
        self.write(&[WRITE_REG, STATUS_REGISTER_2, status.into()])
    }

    /// Remove all the block protection to allow erase and writes
    pub fn disable_block_protect(&mut self) -> WResult<(), SPI> {
        let status = self.read_status_1()?.with_bp(0);
        self.write_status_1(status);
        match self.read_status_1()?.bp() {
            0x0 => Ok(()),
            x => Err(Error::BlockProtect(x)),
        }
    }
    /// Erase the block at ca
    /// Returns error if e-fail flag is set
    pub fn block_erase(&mut self, pa: PageAddress) -> WResult<(), SPI> {
        self.write_enable()?;
        self.transaction(&mut [
            spi::Operation::Write(&[BLOCK_ERASE]),
            spi::Operation::Write(&pa.to_array()),
        ])?;
        self.wait_for_operation()?;
        if self.read_status_3()?.e_fail() {
            Err(Error::EraseFailure)
        } else {
            Ok(())
        }
    }

    /// Load data into buffer at ca, reset rest of buffer to 0
    pub fn load_program_data(&mut self, ca: ColumnAddress, data: &[u8]) -> Result<(), Error<SPI>> {
        self.write_enable()?;
        self.transaction(&mut [
            spi::Operation::Write(&[PROGRAM_DATA_LOAD]),
            spi::Operation::Write(&ca.to_array()),
            spi::Operation::Write(data),
        ])
    }

    /// Load data into buffer at ca, do not reset rest of buffer to 0
    pub fn random_load_program_data(
        &mut self,
        ca: ColumnAddress,
        data: &[u8],
    ) -> Result<(), Error<SPI>> {
        self.write_enable()?;
        self.transaction(&mut [
            spi::Operation::Write(&[RANDOM_PROGRAM_DATA_LOAD]),
            spi::Operation::Write(&ca.to_array()),
            spi::Operation::Write(data),
        ])
    }

    /// Write data from the buffer to the page at pa
    /// Returns error if p-fail flag is set
    pub fn program_execute(&mut self, pa: PageAddress) -> WResult<(), SPI> {
        self.transaction(&mut [
            spi::Operation::Write(&[PROGRAM_EXECUTE]),
            spi::Operation::Write(&pa.to_array()),
        ])?;
        self.wait_for_operation()?;
        if self.read_status_3()?.p_fail() {
            Err(Error::ProgramFailure)
        } else {
            Ok(())
        }
    }

    /// Read data from page at pa into the buffer
    pub fn page_data_read(&mut self, pa: PageAddress) -> WResult<(), SPI> {
        self.transaction(&mut [
            spi::Operation::Write(&[PAGE_DATA_READ]),
            spi::Operation::Write(&pa.to_array()),
        ])?;
        self.wait_for_operation()
    }

    pub fn read_data(&mut self, ca: ColumnAddress, buf: &mut [u8]) -> WResult<(), SPI> {
        self.transaction(&mut [
            spi::Operation::Write(&[READ]),
            spi::Operation::Write(&ca.to_array()),
            spi::Operation::Write(&[0x00]),
            spi::Operation::Read(buf),
        ])
    }

    /// Go to deep power down state
    pub fn deep_power_down(&mut self) -> WResult<(), SPI> {
        self.write(&[DEEP_POWER_DOWN])
    }

    /// Exit deep power down state
    pub fn release_power_down(&mut self) -> WResult<(), SPI> {
        self.write(&[RELEASE_POWER_DOWN])
    }
}
