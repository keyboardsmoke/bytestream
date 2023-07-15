pub trait Writer
{
    fn write_u8(&mut self, data: u8);
    fn write_u16(&mut self, data: u16);
    fn write_u32(&mut self, data: u32);
    fn write_u64(&mut self, data: u64);

    fn write_f32(&mut self, data: f32);
    fn write_f64(&mut self, data: f64);

    fn write_utf8_string_fixed(&mut self, data: String);
    fn write_utf16_string_fixed(&mut self, data: String);

    fn write_utf8_string_terminated(&mut self, data: String);
    fn write_utf16_string_terminated(&mut self, data: String);

    fn get(&self) -> &Vec<u8>;
}

pub struct StreamWriter
{
    buffer: Vec<u8>,
}

impl Writer for StreamWriter
{
    fn write_u8(&mut self, data: u8) {
        self.buffer.push(data)
    }

    fn write_u16(&mut self, data: u16) {
        self.buffer.append(&mut data.to_be_bytes().to_vec());
    }

    fn write_u32(&mut self, data: u32) {
        self.buffer.append(&mut data.to_be_bytes().to_vec());
    }

    fn write_u64(&mut self, data: u64) {
        self.buffer.append(&mut data.to_be_bytes().to_vec());
    }

    fn write_f32(&mut self, data: f32) {
        self.buffer.append(&mut data.to_be_bytes().to_vec());
    }

    fn write_f64(&mut self, data: f64) {
        self.buffer.append(&mut data.to_be_bytes().to_vec());
    }

    fn write_utf8_string_fixed(&mut self, data: String) {
        let mut bytes: Vec<u8> = data.bytes().collect();
        
        while bytes.last().is_some() && *bytes.last().unwrap() == 0 {
            bytes.pop();
        }

        self.buffer.append(&mut bytes);
    }

    fn write_utf16_string_fixed(&mut self, data: String) {
        let mut elements: Vec<u16> = data.encode_utf16().collect();
        
        while elements.last().is_some() && *elements.last().unwrap() == 0 {
            elements.pop();
        }

        let bytes = unsafe { std::slice::from_raw_parts(elements.as_ptr() as *const u8, elements.len() * std::mem::size_of::<u16>()) };
        self.buffer.append(&mut bytes.to_vec());
    }

    fn write_utf8_string_terminated(&mut self, data: String) {
        self.buffer.append(&mut data.bytes().collect());
        if data.bytes().last().is_some() && data.bytes().last().unwrap() != 0 {
            self.buffer.push(0);
        }
    }

    fn write_utf16_string_terminated(&mut self, data: String) {
        let mut elements: Vec<u16> = data.encode_utf16().collect();
        if elements.last().is_some() && *elements.last().unwrap() != 0 {
            elements.push(0);
        }
        let bytes = unsafe { std::slice::from_raw_parts(elements.as_ptr() as *const u8, elements.len() * std::mem::size_of::<u16>()) };
        self.buffer.append(&mut bytes.to_vec());
    }

    fn get(&self) -> &Vec<u8>
    {
        return &self.buffer;
    }
}

pub fn create() -> StreamWriter
{
    StreamWriter { buffer: Vec::new() }
}