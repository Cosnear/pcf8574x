use crate::Error;
use crate::Pcf;
use embedded_hal::blocking::i2c::{Read, Write};

pub trait PcfHandler<'a, B, E> {
    fn sign(&self, device: &mut Pcf<'a, B>, mask: u8);
    fn write(&self, device: &mut Pcf<'a, B>, buffer: &[u8]) -> Result<(), Error<E>>;
    fn read(&self, device: &mut Pcf<'a, B>, buffer: &mut [u8]) -> Result<(), Error<E>>;
    fn write_byte(&self, device: &mut Pcf<'a, B>, buffer: u8) -> Result<(), Error<E>>;
    fn read_byte(&self, device: &mut Pcf<'a, B>) -> Result<u8, Error<E>>;
    fn prev(&self, device: &Pcf<'a, B>) -> u8;
}

impl<'a, B, E> PcfHandler<'a, B, E> for &'a str
where
    B: Read<Error = E> + Write<Error = E>,
{
    fn sign(&self, device: &mut Pcf<'a, B>, mask: u8) {
        device.sign_handler(vec![(self, mask)]);
    }
    
    fn write(&self, device: &mut Pcf<'a, B>, buffer: &[u8]) -> Result<(), Error<E>> {
        device.write_to_mask(buffer, self)
    }
    fn read(&self, device: &mut Pcf<'a, B>, buffer: &mut [u8]) -> Result<(), Error<E>> {
        device.read_from_mask(buffer, self)
    }

    fn write_byte(&self, device: &mut Pcf<'a, B>, buffer: u8) -> Result<(), Error<E>> {
        device.write_b_to_mask(buffer, self)
    }
    fn read_byte(&self, device: &mut Pcf<'a, B>) -> Result<u8, Error<E>> {
        device.read_b_from_mask(self)
    }

    fn prev(&self, device: &Pcf<'a, B>) -> u8 {
        device.backup
    }
}
