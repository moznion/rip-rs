#[derive(Debug)]
pub(crate) struct Parsed<T> {
    value: T,
    cursor: usize,
}

impl<T> Parsed<T> {
    pub(crate) fn new(value: T, cursor: usize) -> Self {
        Parsed { value, cursor }
    }

    pub(crate) fn get_value(&self) -> &T {
        &self.value
    }

    pub(crate) fn get_cursor(&self) -> usize {
        self.cursor
    }
}
