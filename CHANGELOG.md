# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Full responsive design for mobile and tablet devices with collapsible sidebar navigation (#85)
- Touch-friendly form inputs with larger touch targets meeting accessibility standards (#85)
- Mobile-optimized table layouts with card-based display on small screens (#85)
- Horizontal scroll support for tables on tablet devices (#85)
- Bottom sheet style modals on mobile for better touch interaction (#85)

### Changed
- Navigation sidebar now collapses into a hamburger menu on mobile devices (#85)
- Form buttons stack vertically on mobile for easier thumb access (#85)
- Page layouts now adapt to different screen sizes with optimized spacing (#85)
- Dashboard statistics now update automatically via HTMX events when creating entities from quick actions, improving performance and code organization (#135)

### Fixed

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

[unreleased]: https://github.com/josefjura/hockey/compare/v0.1.8...HEAD
[0.1.8]: https://github.com/josefjura/hockey/releases/tag/v0.1.8
[0.1.7]: https://github.com/josefjura/hockey/releases/tag/v0.1.7
