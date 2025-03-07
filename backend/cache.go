package main

func NewLiveFeedItemsCache(size int) *LiveFeedItemsCache {
	return &LiveFeedItemsCache{items: make([]LiveFeedEntry, 0, size), maxSize: size}
}

func (cache *LiveFeedItemsCache) Add(item LiveFeedEntry) {
	if len(cache.items) >= cache.maxSize {
		cache.items = cache.items[1:]
	}
	cache.items = append(cache.items, item)
}

func filterNewItems(items []LiveFeedEntry, existingItems []LiveFeedEntry) []LiveFeedEntry {
	existingIDs := make(map[string]bool)
	for _, item := range existingItems {
		existingIDs[item.ID] = true
	}

	var newItems []LiveFeedEntry
	for _, item := range items {
		if !existingIDs[item.ID] {
			newItems = append(newItems, item)
		}
	}

	return newItems
}

func (cache *LiveFeedItemsCache) GetItems() []LiveFeedEntry {
	return cache.items
}
