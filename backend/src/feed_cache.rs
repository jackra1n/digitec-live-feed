use std::collections::{HashSet, VecDeque};
use crate::types::FeedItem;
use chrono::{DateTime, Utc};


type FeedItemKey = (
    String,
    DateTime<Utc>,
    i32,
    String,
);

fn create_key(item: &FeedItem) -> FeedItemKey {
    (
        item.user_name.clone(),
        item.date_time,
        item.social_shopping_transaction_type_id,
        item.url.clone(),
    )
}

#[derive(Debug)]
pub struct FeedItemCache {
    seen_keys: HashSet<FeedItemKey>,
    order: VecDeque<FeedItemKey>,
    capacity: usize,
}

impl FeedItemCache {
    pub fn new(capacity: usize) -> Self {
        FeedItemCache {
            seen_keys: HashSet::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn process_and_filter_new(&mut self, items: Vec<FeedItem>) -> Vec<FeedItem> {
        let mut new_items = Vec::new();

        for item in items.into_iter() {
            let item_key = create_key(&item);

            if self.seen_keys.contains(&item_key) {
                continue;
            }

            if self.order.len() >= self.capacity {
                if let Some(oldest_key) = self.order.pop_front() {
                    self.seen_keys.remove(&oldest_key);
                }
            }

            self.seen_keys.insert(item_key.clone());
            self.order.push_back(item_key);

            new_items.push(item);
        }
        new_items
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.seen_keys.len()
    }

    #[allow(dead_code)]
    pub fn contains(&self, item: &FeedItem) -> bool {
        let item_key = create_key(item);
        self.seen_keys.contains(&item_key)
    }
}