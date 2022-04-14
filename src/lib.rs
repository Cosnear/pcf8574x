mod slave_addr;
pub use slave_addr::SlaveAddr;

mod devices;
pub use devices::{Pcf, PcfDevice};

pub mod pins;

#[derive(Debug)]
pub enum Error<E> {
    I2C(E),
    InvalidInputData,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
