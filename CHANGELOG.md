# Changelog

## [0.1.0] - 2026-07-25

### Added
- `FunPayClient` with builder pattern, retry, rate limiting, caching
- HTML parser for offers, games, users, orders, reviews, chats
- `SearchQuery` builder for complex searches
- `Monitor` for real-time price change detection
- Streaming search via `async-stream`
- JSON/CSV export
- `SellerProfile` model
- Type-safe IDs (OfferId, UserId, GameId, LotId, Server)
- Currency enum (RUB, USD, EUR)
- GameCategory enum (Chips, Lots)
- OnlineStatus enum (Online, Offline)
- Typed error system (ParseError, AuthError, FunPayError)
- LRU cache for game list
- 71 tests including wiremock mocks and edge cases
- Benchmarks (231µs for 100 games, 4.5ms for 200 offers)
