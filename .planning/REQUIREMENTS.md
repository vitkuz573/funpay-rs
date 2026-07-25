# Requirements

## R1: Authentication
- Accept golden_key cookie
- Store cookies in jar
- Handle CSRF tokens from data-app-data

## R2: Lot Parsing
- Fetch /chips/{id}/ and /lots/{id}/ pages
- Extract: offer_id, price, stock, seller info, description
- Parse both currency and item type lots

## R3: Offer Parsing
- Fetch individual /lots/offer?id={id}
- Extract full offer details
- Support all field types (type, time, method)

## R4: User Profiles
- Fetch /users/{id}/
- Extract: username, rating, reviews, registration date
- Get online status

## R5: Category Discovery
- Fetch homepage
- List all games with their category IDs
- Map game names to chips/lots URLs

## R6: Monitoring
- Poll for price changes at configurable interval
- Track seen offers to detect new/removed ones
- Callback or channel-based notification

## R7: Error Handling
- Custom FunPayError enum
- Network errors, parse errors, auth errors
- Rate limit detection (429)
