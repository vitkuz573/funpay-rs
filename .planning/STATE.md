# State

## Current Phase: 1 - Core Client + Auth
## Status: Planning Complete

## Decisions
- Using reqwest with cookie feature for session management
- scraper crate for HTML parsing
- thiserror for error types
- Manual CSRF extraction from data-app-data JSON

## Blockers
- None

## Learnings
- FunPay uses golden_key cookie for auth
- CSRF tokens come from body's data-app-data attribute
- Two different CSRF tokens: form and header
