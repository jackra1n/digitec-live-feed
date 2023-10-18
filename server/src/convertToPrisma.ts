import { DigitecSocialShoppingItem, DigitecDisplayPrice } from "./types/SocialShoppingTypes";

export const convertSocialShoppingItems = (items: DigitecSocialShoppingItem[]) => {
  return items.map((item: DigitecSocialShoppingItem) => {
    const {
      id,
      displayPrice,
      ...socialShoppingItem
    } = item;

    const newItem = {
      id: parseInt(id),
      ...socialShoppingItem,
      dateTime: new Date(item.dateTime),
    };

    return newItem;
  });
}

export const convertDisplayPrices = (items: DigitecSocialShoppingItem[]) => {
  return items
    .filter((item: DigitecSocialShoppingItem) => item.displayPrice !== null)
    .map((item: DigitecSocialShoppingItem) => {
      const displayPrice = item.displayPrice as DigitecDisplayPrice;
      return {
        amountInclusive: displayPrice.amountInclusive,
        amountExclusive: displayPrice.amountExclusive,
        currency: displayPrice.currency,
        socialShoppingItemId: parseInt(item.id),
      };
    });
}