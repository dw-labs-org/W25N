pub struct ColumnAddress(u32);

impl ColumnAddress {
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

impl From<u32> for ColumnAddress {
    fn from(value: u32) -> Self {
        ColumnAddress(value)
    }
}
