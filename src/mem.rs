#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PageAddress(u32);

impl PageAddress {
    pub fn new_power_2(bit: u8) -> Self {
        Self(1 << bit)
    }
    pub fn from_byte_address(ba: u64) -> Self {
        Self((ba >> 11) as u32)
    }
    fn b0(&self) -> u8 {
        self.0 as u8
    }
    fn b1(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    fn b2(&self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub fn to_array(&self) -> [u8; 3] {
        [self.b2(), self.b1(), self.b0()]
    }

    /// Increments this address to the next block, returns original
    pub fn increment_block(&mut self) -> PageAddress {
        let pa = *self;
        self.0 += 64;
        pa
    }

    /// Increment this address to the next page, returns the original
    pub fn increment_page(&mut self) -> PageAddress {
        let pa = *self;
        self.0 += 1;
        pa
    }

    /// Convert from page address to byte address
    pub fn to_byte_address(self) -> u64 {
        (self.0 as u64) << 11
    }
}

impl From<u32> for PageAddress {
    fn from(value: u32) -> Self {
        PageAddress(value)
    }
}

/// Iterates through block addresses
pub struct BlockAddressIterator {
    end: PageAddress,
    pa: PageAddress,
}

impl BlockAddressIterator {
    pub fn new(start: PageAddress, end: PageAddress) -> Self {
        Self {
            end,
            pa: Default::default(),
        }
    }
}

impl Iterator for BlockAddressIterator {
    type Item = PageAddress;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pa > self.end {
            None
        } else {
            Some(self.pa.increment_block())
        }
    }
}

pub struct ColumnAddress(u16);

impl ColumnAddress {
    pub fn new(addr: u16) -> Self {
        Self(addr)
    }

    fn b0(&self) -> u8 {
        self.0 as u8
    }
    fn b1(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub fn to_array(&self) -> [u8; 2] {
        [self.b1(), self.b0()]
    }
}

impl From<u16> for ColumnAddress {
    fn from(value: u16) -> Self {
        ColumnAddress(value)
    }
}
