pub trait Perform<Item>: Clone {
    type Error;

    /// Performs the given item.
    fn perform(&mut self, item: Item) -> Result<(), Self::Error>;

    /// The child state after having performed the given item.
    fn child(&self, item: Item) -> Result<Self, Self::Error> {
        let mut next = self.clone();
        next.perform(item)?;
        Ok(next)
    }
}
