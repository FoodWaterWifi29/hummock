pub trait HasDiscardFunc<T> {
    // Discards any elements from self that match the predicate.
    // Does not guarantee preservation of order.
    fn discard<F>(&mut self, pred: F) where F: FnMut(&T) -> bool;
}

impl<T> HasDiscardFunc<T> for Vec<T> {
    fn discard<F>(&mut self, mut pred: F) 
        where F: FnMut(&T) -> bool
    {
        let mut j = 0;
        for i in 0..self.len() {
            if pred(&self[i]) {
                self.swap(i, j);
                j += 1;
            }
        }

        self.truncate(j);
    }
}
