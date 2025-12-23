const CAPACITY: usize = 13;

pub struct InputBufferUsize {
    data: [char; CAPACITY],
    len: usize,
}

impl InputBufferUsize {
    pub fn new() -> Self {
        Self {
            data: ['\0'; CAPACITY],
            len: 0,
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if self.len < CAPACITY {
            self.data[self.len] = c;
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len])
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Default for InputBufferUsize {
    fn default() -> Self {
        Self::new()
    }
}
