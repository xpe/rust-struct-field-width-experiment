mod buffer_u8;
mod buffer_u8_hint;
mod buffer_usize;
mod buffer_usize_hint;

pub use buffer_u8::InputBufferU8;
pub use buffer_u8_hint::InputBufferU8Hint;
pub use buffer_usize::InputBufferUsize;
pub use buffer_usize_hint::InputBufferUsizeHint;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_sizes() {
        println!(
            "InputBufferU8:        size={}, align={}",
            std::mem::size_of::<InputBufferU8>(),
            std::mem::align_of::<InputBufferU8>()
        );
        println!(
            "InputBufferUsize:     size={}, align={}",
            std::mem::size_of::<InputBufferUsize>(),
            std::mem::align_of::<InputBufferUsize>()
        );
        println!(
            "InputBufferU8Hint:    size={}, align={}",
            std::mem::size_of::<InputBufferU8Hint>(),
            std::mem::align_of::<InputBufferU8Hint>()
        );
        println!(
            "InputBufferUsizeHint: size={}, align={}",
            std::mem::size_of::<InputBufferUsizeHint>(),
            std::mem::align_of::<InputBufferUsizeHint>()
        );
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
