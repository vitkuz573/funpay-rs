# Roadmap

## Phase 1: Core Client + Auth (MVP)
- Client struct with cookie jar
- golden_key authentication
- CSRF token extraction
- Basic error types
- Unit tests for auth flow

## Phase 2: Lot Parsing
- Parse /chips/{id}/ pages
- Parse /lots/{id}/ pages
- Extract all offer fields
- Integration tests with real pages

## Phase 3: Offer Details
- Parse individual offer pages
- Handle all field variants
- Price currency normalization

## Phase 4: User Profiles
- Parse user profile pages
- Extract rating, reviews, online status
- Registration date parsing

## Phase 5: Category Discovery
- Parse game catalog from homepage
- Build game→node mapping
- Support both chips and lots types

## Phase 6: Monitoring
- Implement polling loop
- Change detection (new/removed/price)
- Callback system
- Rate limiting respect

## Phase 7: Polish
- Documentation
- Examples
- CI/CD setup
- Publish to crates.io
