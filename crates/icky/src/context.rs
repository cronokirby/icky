pub struct Context<'s> {
    src: &'s str,
}

impl<'s> Context<'s> {
    pub fn new(source: &'s str) -> Self {
        Self { src: source }
    }

    pub fn span(&self, start: usize, len: usize) -> &str {
        &self.src[start..start + len]
    }
}
