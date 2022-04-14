/// Slave address of ds3502
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SlaveAddr {
    Default,
    Extend(bool, bool, bool),
}

impl Default for SlaveAddr {
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    /// Return Slaveaddress in u8
    pub(crate) fn val(&self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Extend(a2, a1, a0) => {
                default
                    | (if *a2 { 0x04 } else { 0x00 })
                    | (if *a1 { 0x02 } else { 0x00 })
                    | (if *a0 { 0x01 } else { 0x00 })
            }
        }
    }
}
