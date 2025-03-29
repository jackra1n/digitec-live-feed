use std::collections::{HashSet, VecDeque};
use crate::types::FeedItem;

#[derive(Debug)]
pub struct FeedItemCache {
    seen_ids: HashSet<String>,
    order: VecDeque<String>,
    capacity: usize,
}

impl FeedItemCache {
    pub fn new(capacity: usize) -> Self {
        FeedItemCache {
            seen_ids: HashSet::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_and_check(&mut self, item: &FeedItem) -> bool {
        if self.seen_ids.contains(&item.id) {
            false
        } else {
            if self.order.len() >= self.capacity {
                if let Some(oldest_id) = self.order.pop_front() {
                    self.seen_ids.remove(&oldest_id);
                }
            }

            let id_owned = item.id.clone();
            self.seen_ids.insert(id_owned.clone());
            self.order.push_back(id_owned);

            true
        }
    }

    pub fn len(&self) -> usize {
        self.seen_ids.len()
    }
}