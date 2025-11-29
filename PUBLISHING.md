# Publishing to Crates.io

This guide explains how to publish the `lambda-router` crate to [crates.io](https://crates.io).

## Prerequisites

1. **Create a crates.io account**: Go to [crates.io](https://crates.io) and sign in with your GitHub account.

2. **Get your API token**: 
   - Go to [Account Settings](https://crates.io/settings/tokens)
   - Create a new API token
   - Save this token securely

3. **Login to cargo**:
   ```bash
   cargo login <your-api-token>
   ```

## Before Publishing

### 1. Verify Package Metadata

Ensure all metadata in `Cargo.toml` is correct:
- `name`: Package name (must be unique on crates.io)
- `version`: Semantic version
- `description`: Clear description
- `license`: MIT or other valid license
- `repository`: GitHub repository URL
- `keywords`: Up to 5 keywords
- `categories`: Valid crates.io categories

### 2. Check the Package

Run a dry-run to verify everything is correct:

```bash
cd packages/lambda-router
cargo publish --dry-run
```

This will:
- Build the package
- Verify all files are included
- Check for common issues

### 3. Build Documentation

Ensure documentation builds correctly:

```bash
cargo doc --no-deps --open
```

### 4. Run Tests

```bash
cargo test
```

## Publishing

### First-Time Publishing

If this is the first time publishing this crate:

```bash
cd packages/lambda-router
cargo publish
```

### Subsequent Releases

1. **Update version** in `Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md** with changes

3. **Commit changes**:
   ```bash
   git add .
   git commit -m "chore: bump version to 0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```

4. **Publish**:
   ```bash
   cargo publish
   ```

## Version Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

## Separating from Workspace

If you want to publish this as a standalone package outside the workspace:

1. **Create a new repository**:
   ```bash
   mkdir lambda-router
   cd lambda-router
   git init
   ```

2. **Copy the package files**:
   ```bash
   cp -r /path/to/chatbot-platform/packages/lambda-router/* .
   ```

3. **Remove workspace reference** (if any)

4. **Publish**:
   ```bash
   cargo publish
   ```

## Using the Published Crate

Once published, users can add it to their `Cargo.toml`:

```toml
[dependencies]
lambda-router = "0.1"
```

Or with specific features:

```toml
[dependencies]
lambda-router = { version = "0.1", features = ["full"] }
```

## Troubleshooting

### "crate name already exists"

Choose a different name or check if you own the existing crate.

### "failed to verify package"

Run `cargo publish --dry-run` to see specific errors.

### "version already exists"

Increment the version number in `Cargo.toml`.

## Useful Links

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Crates.io](https://crates.io)
- [Semantic Versioning](https://semver.org)
