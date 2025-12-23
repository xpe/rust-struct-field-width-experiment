const CAPACITY: usize = 13;

// =============================================================================
// InputBufferU8 — uses len: u8
// =============================================================================

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

// =============================================================================
// InputBufferUsize — uses len: usize
// =============================================================================

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

// =============================================================================
// InputBufferU8Hint — uses len: u8 with assert_unchecked before indexing
// =============================================================================

pub struct InputBufferU8Hint {
    data: [char; CAPACITY],
    len: u8,
}

impl InputBufferU8Hint {
    pub fn new() -> Self {
        Self {
            data: ['\0'; CAPACITY],
            len: 0,
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        if (self.len as usize) < CAPACITY {
            // SAFETY: We just checked that len < CAPACITY (13)
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
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
            // SAFETY: len was > 0, now len < 13
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
            Some(self.data[self.len as usize])
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        if index < self.len as usize {
            // SAFETY: We maintain len <= 13
            unsafe { std::hint::assert_unchecked(self.len <= 13) };
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}

impl Default for InputBufferU8Hint {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// InputBufferUsizeHint — uses len: usize with assert_unchecked (control case)
// =============================================================================

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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_sizes() {
        println!("InputBufferU8:        size={}, align={}",
            std::mem::size_of::<InputBufferU8>(),
            std::mem::align_of::<InputBufferU8>());
        println!("InputBufferUsize:     size={}, align={}",
            std::mem::size_of::<InputBufferUsize>(),
            std::mem::align_of::<InputBufferUsize>());
        println!("InputBufferU8Hint:    size={}, align={}",
            std::mem::size_of::<InputBufferU8Hint>(),
            std::mem::align_of::<InputBufferU8Hint>());
        println!("InputBufferUsizeHint: size={}, align={}",
            std::mem::size_of::<InputBufferUsizeHint>(),
            std::mem::align_of::<InputBufferUsizeHint>());
    }

    #[test]
    fn test_basic_operations() {
        let mut buf = InputBufferU8::new();
        assert_eq!(buf.len(), 0);
        assert!(buf.push('a').is_ok());
        assert_eq!(buf.len(), 1);
        assert_eq!(buf.get(0), Some(&'a'));
        assert_eq!(buf.pop(), Some('a'));
        assert_eq!(buf.len(), 0);
    }
}
