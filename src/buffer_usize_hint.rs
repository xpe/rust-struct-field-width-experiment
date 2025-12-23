const CAPACITY: usize = 13;

pub struct InputBufferUsizeHint {
    data: [char; CAPACITY],
    len: usize,
}

impl InputBufferUsizeHint {
    pub fn new() -> Self {
        Self {
            data: ['\0'; CAPACITY],
            len: 0,
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if self.len < CAPACITY {
            // SAFETY: We just checked that len < CAPACITY (13)
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
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
            // SAFETY: len was > 0, now len < 13
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
            Some(self.data[self.len])
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        if index < self.len {
            // SAFETY: We maintain len <= 13
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Default for InputBufferUsizeHint {
    fn default() -> Self {
        Self::new()
    }
}
