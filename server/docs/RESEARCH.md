digitec live feed website code is dumped in `resources/digitec-live-feed_website.js`

Based on that we know the different types of live feed activities that are defined with `socialShoppingTransactionTypeId`:

| Key                         | ID | Description                                                                        |
|-----------------------------|----|------------------------------------------------------------------------------------|
| RATING                      | 1  | A user rates a product.                                                            |
| USERCREATION                | 2  | A user creates an account.                                                         |
| ORDEREDPRODUCT              | 3  | A user orders a product.                                                           |
| PICKEDUPPRODUCT             | 4  | A user picks up a product in-store.                                                |
| SHIPPEDPRODUCT              | 5  | A product is shipped to a user.                                                    |
| WATCHEDPRODUCT              | 6  | A user views or "watches" a product.                                               |
| SEARCH                      | 7  | A user performs a search query.                                                    |
| OAUTHUSERCONNECTED          | 8  | A user connects their account using an OAuth provider (e.g., Google, Facebook).    |
| USERVOTESONRATING           | 9  | A user votes on another user's product rating (e.g., "helpful" or "not helpful").  |
| USERCOMMENTSONRATING        | 10 | A user comments on another user's product rating.                                  |
| USERASKSQUESTION            | 11 | A user asks a question (e.g., about a product or a topic).                         |
| USERANSWERSQUESTION         | 12 | A user answers a question.                                                         |
| USERVOTESONQUESTION         | 13 | A user votes on another user's question.                                           |
| USERVOTESONANSWER           | 14 | A user votes on another user's answer.                                             |
| USERCOMMENTSONANSWER        | 15 | A user comments on another user's answer.                                          |
| USERSTARTSDISCUSSION        | 16 | A user starts a discussion (e.g., about a product or topic).                       |
| USERCONTRIBUTESTODISCUSSION | 17 | A user contributes to an ongoing discussion (e.g., replying to posts or comments). |
