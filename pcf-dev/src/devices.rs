use embedded_hal::blocking::i2c::{Read, Write};
use std::collections::HashMap;
use std::vec::Vec;

use crate::{Error, SlaveAddr};

pub enum PcfDevice {
    Pcf8574,
    Pcf8574a,
}

pub struct Pcf<'a, B> {
    mask: &'a mut HashMap<&'a str, u8>,
    pub(crate) backup: u8,
    bus: B,
    addr: u8,
}

impl<'a, B, E> Pcf<'a, B>
where
    B: Write<Error = E> + Read<Error = E>,
{
    pub fn new(
        i2c: B,
        device: PcfDevice,
        slave_addr: SlaveAddr,
        write_mask_map: &'a mut HashMap<&'a str, u8>,
    ) -> Self {
        if write_mask_map.is_empty() {
            write_mask_map.insert("AllRead", 0x00);
        }
        Pcf {
            mask: write_mask_map,
            backup: 0,
            bus: i2c,
            addr: slave_addr.val(match device {
                PcfDevice::Pcf8574 => 0x20,
                PcfDevice::Pcf8574a => 0x38,
            }),
        }
    }

    // pub(crate) fn prev(&self, handler: &str) -> Option<&u8> {
    //     self.
    // }

    pub(crate) fn get_mask(&mut self, handler: &str) -> Result<&u8, Error<E>> {
        match self.mask.get(handler) {
            Some(m) => Ok(m),
            None => Err(Error::InvalidInputData),
        }
    }

    pub(crate) fn sign_handler(&mut self, set_mask: Vec<(&'a str, u8)>) {
        for (k, v) in set_mask {
            self.mask.entry(k).and_modify(|old| *old = v).or_insert(v);
        }
    }
}

impl<'a, B, E> Pcf<'a, B>
where
    B: Write<Error = E>,
{
    fn write(&mut self, buffer: &[u8]) -> Result<(), E> {
        match self.bus.write(self.addr, buffer) {
            Ok(()) => {
                self.backup = *buffer.last().unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) fn write_to_mask(&mut self, raw_data: &[u8], handler: &str) -> Result<(), Error<E>> {
        let mask = *match self.mask.get(handler) {
            Some(m) => m,
            None => return Err(Error::InvalidInputData),
        };

        let bckup = self.backup & (!mask);
        //coppy another group to rawdata
        let data: Vec<u8> = raw_data.iter().map(|d| (d & mask) | bckup).collect();
        self.write(&data).map_err(Error::I2C)
    }

    pub(crate) fn write_b_to_mask(&mut self, raw_data: u8, handler: &str) -> Result<(), Error<E>> {
        let mask = *match self.mask.get(handler) {
            Some(m) => m,
            None => return Err(Error::InvalidInputData),
        };

        let bckup = self.backup & (!mask);
        //coppy another group to rawdata
        let data = (raw_data & mask) | bckup;
        self.write(&[data]).map_err(Error::I2C)
    }
}

impl<'a, B, E> Pcf<'a, B>
where
    B: Read<Error = E>,
{
    fn read(&mut self, buffer: &mut [u8]) -> Result<(), E> {
        self.bus.read(self.addr, buffer)
    }

    pub(crate) fn read_from_mask(
        &mut self,
        buffer: &mut [u8],
        handler: &str,
    ) -> Result<(), Error<E>> {
        let mask = *match self.mask.get(handler) {
            Some(m) => m,
            None => return Err(Error::InvalidInputData),
        };

        //backup unrelated bits
        let bckup = self.backup & (!mask);

        match self.read(buffer).map_err(Error::I2C) {
            Ok(()) => {
                for i in &mut *buffer {
                    *i &= mask;
                }
                //update backup data
                self.backup = buffer.last().unwrap() | bckup;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) fn read_b_from_mask(&mut self, handler: &str) -> Result<u8, Error<E>> {
        let mut res = 0u8;

        let mask = *match self.mask.get(handler) {
            Some(m) => m,
            None => return Err(Error::InvalidInputData),
        };

        //backup unrelated bits
        let bckup = self.backup & (!mask);

        match self.read(&mut [res]).map_err(Error::I2C) {
            Ok(()) => {
                res &= mask;
                //update backup data
                self.backup = res | bckup;
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }
}
