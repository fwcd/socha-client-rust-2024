pub trait Perform<Item>: Clone {
    /// Performs the given item.
    fn perform(&mut self, item: Item);

    /// The child state after having performed the given item.
    fn child(&self, item: Item) -> Self {
        let mut next = self.clone();
        next.perform(item);
        next
    }
}
