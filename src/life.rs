pub trait Life: Sized {
    fn neighborhood(&self) -> [Self; 8];
}

impl Life for (i32, i32) {
    fn neighborhood(&self) -> [Self; 8] {
        [
            (self.0 - 1, self.1 - 1),
            (self.0, self.1 - 1),
            (self.0 + 1, self.1 - 1),
            (self.0 - 1, self.1),
            (self.0 + 1, self.1),
            (self.0 - 1, self.1 + 1),
            (self.0, self.1 + 1),
            (self.0 + 1, self.1 + 1),
        ]
    }
}
