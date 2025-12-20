# Asset Management

This document describes how static assets (CSS, JavaScript, images, flags) are managed and embedded in the Hockey Management System.

## Overview

The application uses **rust-embed** to embed static assets directly into the binary during compilation. This approach provides several benefits:

- **Single Binary Deployment**: All assets are included in the executable
- **No External Dependencies**: No need to deploy separate asset directories
- **Faster Startup**: No file system lookups on first request
- **Improved Security**: Assets cannot be modified after deployment
- **Automatic Gzip Compression**: Tower-http middleware compresses responses

## Architecture

### Embedded Assets

All static assets except user uploads are embedded in the binary:

- **JavaScript Components** (`static/js/components/`)
  - Lit web components compiled from TypeScript
  - Minified for production builds
  - Includes shared utilities and types

- **CSS Files** (`static/css/`)
  - Tailwind CSS and custom styles
  - Currently empty (Tailwind loaded via CDN)

- **Flags** (`static/flags/`)
  - Country flag images
  - Currently empty (flags loaded externally)

### User Uploads

User-uploaded files (player photos, etc.) are **NOT embedded** and are served from the filesystem:

- Located in `static/uploads/`
- Stored persistently on disk
- Served with different caching strategy
- Must be backed up separately

## Development vs Production

### Development Mode (`cargo run`)

With the `debug-embed` feature enabled:

- Assets are read from the filesystem at runtime
- Changes to assets are immediately reflected (hot-reload)
- No build step required for asset changes
- Easier debugging and development

### Production Mode (`cargo build --release`)

Without `debug-embed`:

- Assets are embedded at compile time
- Binary includes all static files
- No filesystem access for embedded assets
- Smaller deployment footprint

## Build Process

### Web Components Build

The TypeScript web components must be compiled to JavaScript before the Rust build:

```bash
cd web_components

# Development build (non-minified)
yarn build

# Production build (minified)
yarn build:prod
```

The production build:
1. Compiles TypeScript to JavaScript (`tsc`)
2. Minifies the output using esbuild (`yarn build:minify`)
3. Outputs to `static/js/components/`

### Full Production Build

Use the Makefile target to build everything:

```bash
make build-full
```

This runs:
1. `cd web_components && yarn install && yarn build:prod` - Build and minify web components
2. `cargo build --release` - Build Rust binary with embedded assets

### Docker Build

The multi-stage Dockerfile handles the complete build:

**Stage 1: Node Builder**
- Installs Node dependencies
- Compiles and minifies TypeScript web components
- Outputs to `static/js/components/`

**Stage 2: Rust Builder**
- Copies source code and migrations
- Copies minified web components from Stage 1
- Embeds all assets using rust-embed
- Compiles Rust binary with embedded assets

**Stage 3: Runtime**
- Uses distroless base image
- Copies only the final binary and migrations
- No static files copied (they're embedded)

## Asset Serving

### Handler Implementation

The `src/assets.rs` module provides the asset handler:

```rust
#[derive(RustEmbed)]
#[folder = "static/"]
#[exclude = "uploads/*"]
pub struct Assets;
```

### Route Configuration

In `src/main.rs`:

```rust
.route("/static/*path", get(static_asset_handler))
```

### Caching Strategy

**Embedded Assets** (CSS, JS, images):
- Cache-Control: `public, max-age=31536000, immutable`
- 1 year cache (content hash-based cache busting recommended)

**User Uploads** (photos):
- Cache-Control: `public, max-age=3600`
- 1 hour cache (content may change)

## Compression

Gzip compression is handled by Tower-http middleware:

```rust
.layer(CompressionLayer::new().gzip(true))
```

This compresses all responses including:
- HTML pages
- JSON API responses
- Static assets
- User uploads

## Minification

### JavaScript Minification

Web components are minified using esbuild:

```bash
yarn build:minify
```

This uses esbuild with:
- `--minify` flag for compression
- `--allow-overwrite` to replace files in-place
- Removes comments and whitespace
- Mangles variable names

### Future Enhancements

Potential improvements:
- CSS minification (if custom CSS is added)
- Image optimization (if images are added)
- Content hash-based cache busting
- Pre-compressed assets (brotli)

## File Structure

```
static/
├── js/
│   └── components/          # Built Lit web components
│       ├── badge.js
│       ├── client-data-table.js
│       ├── confirm-dialog.js
│       ├── countries-table.js
│       ├── country-selector.js
│       ├── flag-icon.js
│       ├── loading-spinner.js
│       ├── loading-state.js
│       ├── modal.js
│       ├── toast.js
│       ├── toggle-switch.js
│       └── shared/
│           ├── api-client.js
│           └── types.js
├── css/                     # CSS files (currently empty)
├── flags/                   # Flag images (currently empty)
└── uploads/                 # User uploads (NOT embedded)
    └── players/             # Player photos
```

## Best Practices

### For Developers

1. **Always build web components before Rust**: Assets must exist before embedding
2. **Use `yarn build:prod` for production**: Ensures minification
3. **Test with production build**: `cargo build --release` to verify embedding
4. **Commit built files**: Both source and output should be in git

### For Deployment

1. **Use Docker build**: Multi-stage build handles everything correctly
2. **Backup uploads directory**: Not included in binary, needs separate backup
3. **Volume mount uploads**: `docker-compose.yaml` should mount `static/uploads`
4. **Monitor binary size**: Embedded assets increase binary size

### For Asset Changes

1. Modify TypeScript source in `web_components/`
2. Run `yarn build:prod` to compile and minify
3. Test locally with `cargo run`
4. Commit both `.ts` and `.js` files
5. Production build will embed the new assets

## Troubleshooting

### Assets not loading in production

- Verify assets exist in `static/` before build
- Check `cargo build --release` output for errors
- Ensure `debug-embed` feature is not enabled for production
- Verify paths match exactly (case-sensitive)

### Large binary size

- Run `yarn build:prod` to ensure minification
- Consider removing unused web components
- Check for accidentally embedded uploads
- Use `cargo bloat` to analyze binary size

### Upload files not accessible

- Verify volume mount in Docker Compose
- Check file permissions in `static/uploads/`
- Ensure upload handler uses correct path
- Verify `#[exclude = "uploads/*"]` in RustEmbed

## References

- [rust-embed](https://github.com/pyrossh/rust-embed) - Asset embedding library
- [tower-http](https://docs.rs/tower-http/) - HTTP middleware including compression
- [esbuild](https://esbuild.github.io/) - JavaScript minification
- [Lit](https://lit.dev/) - Web components framework
