use std::io::{Read, Result, Write};

pub trait BinaryReader: Read {
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)?;
        Ok(())
    }

    fn read_packet<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let len = self.read_u32()? as usize;
        let buf = &mut buf[..len];
        self.read_exact(buf)?;
        Ok(buf)
    }
}

pub trait BinaryWriter: Write {
    fn write_u32(&mut self, n: u32) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<()> {
        self.write_all(buf)
    }

    fn write_packet(&mut self, buf: &[u8]) -> Result<()> {
        self.write_u32(buf.len() as u32)?;
        self.write_all(buf)
    }
}

impl BinaryReader for socket2::Socket {}
impl BinaryWriter for socket2::Socket {}
