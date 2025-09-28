#[derive(Clone, Copy)]
pub struct Entity {
    pub(crate) index: usize,
}

impl Entity {
    pub fn new(index: usize) -> Self {
        Entity { index }
    }
}
