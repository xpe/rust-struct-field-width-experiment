const CAPACITY: usize = 13;

pub struct InputBufferU8 {
    data: [char; CAPACITY],
    len: u8,
}

impl InputBufferU8 {
    pub fn new() -> Self {
        Self {
            data: ['\0'; CAPACITY],
            len: 0,
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if (self.len as usize) < CAPACITY {
            self.data[self.len as usize] = c;
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len as usize])
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        if index < self.len as usize {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}

impl Default for InputBufferU8 {
    fn default() -> Self {
        Self::new()
    }
}
