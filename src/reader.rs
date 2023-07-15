use crate::ByteStreamError;

pub trait Reader
{
    fn read_u8(&mut self) -> u8;
    fn read_u16(&mut self) -> u16;
    fn read_u32(&mut self) -> u32;
    fn read_u64(&mut self) -> u64;

    fn read_f32(&mut self) -> f32;
    fn read_f64(&mut self) -> f64;

    fn read_utf8_string_fixed(&mut self, num_elements: usize) -> Result<String, ByteStreamError>;
    fn read_utf16_string_fixed(&mut self, num_elements: usize) -> Result<String, ByteStreamError>;

    fn read_utf8_string_terminated(&mut self) -> Result<String, ByteStreamError>;
    fn read_utf16_string_terminated(&mut self) -> Result<String, ByteStreamError>;
}

pub struct StreamReader<'a>
{
    pub index: usize,
    pub data: &'a [u8],
}

impl<'a> Reader for StreamReader<'a>
{
    fn read_u8(&mut self) -> u8 {
        let res = self.data[self.index];
        self.index += std::mem::size_of::<u8>();
        return res;
    }

    fn read_u16(&mut self) -> u16 {
        let spl = self.data.split_at(self.index);
        let precise: [u8; std::mem::size_of::<u16>()] = [spl.1[0], spl.1[1]];
        return u16::from_be_bytes(precise);
    }

    fn read_u32(&mut self) -> u32 {
        let spl = self.data.split_at(self.index);
        let precise: [u8; std::mem::size_of::<u32>()] = [spl.1[0], spl.1[1], spl.1[2], spl.1[3]];
        return u32::from_be_bytes(precise);
    }

    fn read_u64(&mut self) -> u64 {
        let spl = self.data.split_at(self.index);
        let precise: [u8; std::mem::size_of::<u64>()] = [spl.1[0], spl.1[1], spl.1[2], spl.1[3], spl.1[4], spl.1[5], spl.1[6], spl.1[7]];
        return u64::from_be_bytes(precise);
    }

    fn read_f32(&mut self) -> f32 {
        let spl = self.data.split_at(self.index);
        let precise: [u8; std::mem::size_of::<f32>()] = [spl.1[0], spl.1[1], spl.1[2], spl.1[3]];
        return f32::from_be_bytes(precise);
    }

    fn read_f64(&mut self) -> f64 {
        let spl = self.data.split_at(self.index);
        let precise: [u8; std::mem::size_of::<f64>()] = [spl.1[0], spl.1[1], spl.1[2], spl.1[3], spl.1[4], spl.1[5], spl.1[6], spl.1[7]];
        return f64::from_be_bytes(precise);
    }

    fn read_utf8_string_fixed(&mut self, num_elements: usize) -> Result<String, ByteStreamError> {
        let spl = self.data.split_at(self.index);
        let spl = spl.1.split_at(num_elements);
        self.index += num_elements;
        return Ok(String::from_utf8(spl.0.to_vec())?);
    }

    fn read_utf16_string_fixed(&mut self, num_elements: usize) -> Result<String, ByteStreamError> {
        let spl0 = self.data.split_at(self.index);
        let split_start = spl0.1;
        let spl: &[u16] = unsafe { std::slice::from_raw_parts(split_start.as_ptr() as *const u16, num_elements) };
        self.index += num_elements * std::mem::size_of::<u16>();
        return Ok(String::from_utf16(spl)?);
    }

    fn read_utf8_string_terminated(&mut self) -> Result<String, ByteStreamError> {
        let spl = self.data.split_at(self.index);
        let null_terminator = spl.1.iter().find(|x| **x == 0);
        if null_terminator.is_none() {
            return Err(ByteStreamError::Message(format!("Unable to find null terminator for string.")));
        }
        let null_terminator = null_terminator.unwrap();
        let delta = unsafe { (null_terminator as *const u8).offset_from(spl.1.as_ptr()) } as usize;
        let res = self.read_utf8_string_fixed(delta);
        if res.is_ok() {
            self.index += std::mem::size_of::<u8>(); // null terminator
            return Ok(res.unwrap());
        }
        return Err(res.err().unwrap());
    }

    fn read_utf16_string_terminated(&mut self) -> Result<String, ByteStreamError> {
        let spl = self.data.split_at(self.index);
        let data: &[u16] = unsafe { std::mem::transmute(spl.1) };
        let null_terminator = data.iter().find(|x| **x == 0);
        if null_terminator.is_none() {
            return Err(ByteStreamError::Message(format!("Unable to find null terminator for string.")));
        }
        let null_terminator = null_terminator.unwrap();
        let delta = unsafe { (null_terminator as *const u16 as *const u8).offset_from(spl.1.as_ptr()) } as usize;
        if delta % std::mem::size_of::<u16>() != 0 {
            return Err(ByteStreamError::Message(format!("The determined string length must be a multiple of 2.")))
        }
        let final_delta = delta / std::mem::size_of::<u16>();
        let res = self.read_utf16_string_fixed(final_delta);
        if res.is_ok() {
            self.index += std::mem::size_of::<u16>(); // null terminator
            return Ok(res.unwrap());
        }
        return Err(res.err().unwrap());
    }
}

pub fn from_slice<'a>(data: &'a [u8]) -> StreamReader
{
    StreamReader { index: 0, data }
}

pub fn from_vec<'a>(data: &'a Vec<u8>) -> StreamReader
{
    StreamReader { index: 0, data: data.as_slice() }
}