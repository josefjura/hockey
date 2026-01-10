# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed
- CI builds no longer fail during E2E tests due to SESSION_SECRET being too short to meet the 32-character security requirement
- CI E2E tests now wait for the server to actually start instead of using a fixed delay, preventing test failures caused by server startup taking longer than expected
- Database queries now perform significantly faster with proper indexes on all foreign keys and common query patterns, preventing performance degradation as data grows (#177)
- Player height and weight fields now reject invalid values like negative numbers or biologically impossible measurements (#174)

### Security
- Session cookies are now cryptographically signed using HMAC-SHA256, preventing attackers from forging valid session tokens even if they can guess session IDs (#162)
- Production deployments now require a secure SESSION_SECRET to be explicitly set, preventing the use of insecure default values that could allow session forgery (#179)
- SESSION_SECRET must now be at least 32 characters long to ensure adequate cryptographic security (#179)

## [0.1.9] - 2026-01-05

### Added
- Full responsive design for mobile and tablet devices with collapsible sidebar navigation (#85)
- Touch-friendly form inputs with larger touch targets meeting accessibility standards (#85)
- Mobile-optimized table layouts with card-based display on small screens (#85)
- Horizontal scroll support for tables on tablet devices (#85)
- Bottom sheet style modals on mobile for better touch interaction (#85)
- Comprehensive test coverage for service layer operations including players, team participations, and dashboard statistics (#126)
- Career timeline for players showing property changes, trades, role changes, and career milestones with rich card display (#130)

### Changed
- Navigation sidebar now collapses into a hamburger menu on mobile devices (#85)
- Form buttons stack vertically on mobile for easier thumb access (#85)
- Page layouts now adapt to different screen sizes with optimized spacing (#85)
- Dashboard statistics now update automatically via HTMX events when creating entities from quick actions, improving performance and code organization (#135)
- Score event validation logic is now centralized for better code maintainability (#157)
- Match validation logic has been moved to a dedicated business layer for better separation of concerns and testability (#156)
- Player detail page data is now fetched through a dedicated business layer function for improved code organization (#154)
- Player scoring page data is now fetched through a dedicated business layer function for improved code organization (#155)
- Player create and update handlers now use dedicated form parsing and validation helpers for better code organization and maintainability (#153)

## [0.1.8] - 2025-12-31

### Added
- Maximum value validation for player event statistics - goals and assists now have a reasonable upper limit of 10,000 (#147)

### Changed
- Player event statistics cards now use CSS classes instead of inline styles for better maintainability (#143)
- Player event statistics UI now displays in both English and Czech languages (#139)

### Fixed
- Player event statistics are now saved reliably without leaving incomplete data if an error occurs (#146)

## [0.1.7] - 2025-12-31

_Initial changelog - previous versions not documented_

[unreleased]: https://github.com/josefjura/hockey/compare/v0.1.9...HEAD
[0.1.9]: https://github.com/josefjura/hockey/releases/tag/v0.1.9
[0.1.8]: https://github.com/josefjura/hockey/releases/tag/v0.1.8
[0.1.7]: https://github.com/josefjura/hockey/releases/tag/v0.1.7
