use embedded_hal::serial::{Read, Write};
pub(crate) struct SerialForwarder<T: Read<u8>, U: Write<u8>> {
    src: T,
    dest: U,
}

impl<T: Read<u8>, U: Write<u8>> SerialForwarder<T, U> {
    pub fn new(src: T, dest: U) -> Self {
        return Self { src, dest };
    }

    pub fn poll_and_forward(&mut self) {
        while let Ok(data) = self.src.read() {
            // write may return Err if it would block because output fifo is full, just tight poll until write is complete
            while let Err(_) = self.dest.write(data) {}
        }
    }
}
