/// Tracks which Properties have changed and need to be queued for syncing with
/// the Client
pub trait EntityMutator {
    /// Given the index of the Property whose value has changed, queue that
    /// Property for transmission to the Client
    fn mutate(&mut self, property_index: u8);
}
