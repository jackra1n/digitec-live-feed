package main

import "fmt"

type FeedItemCache struct {
	seenCompositeKeys map[string]struct{}
	order             []string
	capacity          int
}

func NewFeedItemCache(capacity int) *FeedItemCache {
	if capacity <= 0 {
		fmt.Println("Warning: FeedItemCache created with non-positive capacity, defaulting to 10")
		capacity = 50
	}
	return &FeedItemCache{
		seenCompositeKeys: make(map[string]struct{}, capacity),
		order:             make([]string, 0, capacity),
		capacity:          capacity,
	}
}

func generateCompositeKey(item FeedItem) string {
	return fmt.Sprintf("%s_%s_%d_%s",
		item.UserName,
		item.DateTime,
		item.SocialShoppingTransactionTypeID,
		item.URL,
	)
}

func (fic *FeedItemCache) ProcessAndFilterNew(items []FeedItem) []FeedItem {
	newItems := make([]FeedItem, 0, len(items))

	for _, item := range items {
		compositeKey := generateCompositeKey(item)

		if _, exists := fic.seenCompositeKeys[compositeKey]; exists {
			continue
		}

		if len(fic.order) >= fic.capacity {
			oldestID := fic.order[0]
			delete(fic.seenCompositeKeys, oldestID)
			fic.order = fic.order[1:]
		}

		fic.seenCompositeKeys[compositeKey] = struct{}{}
		fic.order = append(fic.order, compositeKey)
		newItems = append(newItems, item)
	}

	return newItems
}

func (fic *FeedItemCache) Len() int {
	return len(fic.seenCompositeKeys)
}

func (fic *FeedItemCache) Contains(item FeedItem) bool {
	compositeKey := generateCompositeKey(item)
	_, exists := fic.seenCompositeKeys[compositeKey]
	return exists
}
