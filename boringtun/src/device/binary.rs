use std::io::{Read, Result, Write};

pub trait BinaryReader: Read {
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        println!("read u32: {}", hex::encode(buf));
        Ok(u32::from_be_bytes(buf))
    }

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)?;
        Ok(())
    }

    fn read_packet<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let len = self.read_u32()? as usize;
        let buf = &mut buf[..len];
        println!("read_buf");
        self.read_exact(buf).unwrap();
        println!("read buf: {}", hex::encode(&buf));
        Ok(buf)
    }
}

pub trait BinaryWriter: Write {
    fn write_u32(&mut self, n: u32) -> Result<()> {
        println!("write u32: {}", hex::encode(n.to_be_bytes()));
        self.write_all(&n.to_be_bytes())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<()> {
        self.write_all(buf)
    }

    fn write_packet(&mut self, buf: &[u8]) -> Result<()> {
        self.write_u32(buf.len() as u32)?;
        println!("write buf: {}", hex::encode(&buf));
        self.write_all(buf)
    }
}
impl BinaryReader for socket2::Socket {
    fn read_packet<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        self.set_nonblocking(false);
        let len = self.read_u32()? as usize;
        let buf = &mut buf[..len];
        self.read_exact(buf)?;
        println!("read buf: {}", hex::encode(&buf));
        self.set_nonblocking(true);
        Ok(buf)
    }
}
impl BinaryWriter for socket2::Socket {
    fn write_packet(&mut self, buf: &[u8]) -> Result<()> {
        self.set_nonblocking(false).unwrap();
        self.write_u32(buf.len() as u32)?;
        println!("write buf: {}", hex::encode(&buf));
        self.write_all(buf)?;
        self.set_nonblocking(true).unwrap();
        Ok(())
    }
}

