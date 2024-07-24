pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0x000000; width * height];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = 0xFFFFFF;
        }
    }

    pub fn is_point_set(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] == 0xFFFFFF
        } else {
            false
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }
}
