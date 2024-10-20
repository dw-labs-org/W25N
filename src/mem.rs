pub struct PageAddress(u32);

impl PageAddress {
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
}

impl From<u32> for PageAddress {
    fn from(value: u32) -> Self {
        PageAddress(value)
    }
}

pub struct ColumnAddress(u16);

impl ColumnAddress {
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
