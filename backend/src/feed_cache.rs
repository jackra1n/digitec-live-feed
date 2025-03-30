use std::collections::{HashSet, VecDeque};
use crate::types::FeedItem;

#[derive(Debug)]
pub struct FeedItemCache {
    seen_ids: HashSet<u32>,
    order: VecDeque<u32>,
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

    pub fn process_and_filter_new(&mut self, items: Vec<FeedItem>) -> Vec<FeedItem> {
        let mut new_items = Vec::with_capacity(items.len());

        for item in items.into_iter() {
            let item_id = item.id;

            if self.seen_ids.contains(&item_id) {
                continue;
            }

            if self.order.len() >= self.capacity {
                if let Some(oldest_id) = self.order.pop_front() {
                    self.seen_ids.remove(&oldest_id);
                }
            }
            self.seen_ids.insert(item_id);
            self.order.push_back(item_id);
            new_items.push(item);
        }
        new_items
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.seen_ids.len()
    }

    #[allow(dead_code)]
    pub fn contains(&self, item: &FeedItem) -> bool {
        self.seen_ids.contains(&item.id)
    }
}