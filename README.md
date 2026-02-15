# Async Linkchecker

Takes a markdown file containing a list of URLs, fetches each concurrently (32 at a time), extracts the inner content inside `<title>` tag, and generates a new markdown file with links in proper format.

## Usage

```bash
make run
# or
cargo run -- input.md output.md
```

### Input format

```md
- https://www.google.com
- https://github.com
```

### Output format

```md
- [Google](https://www.google.com)
- [GitHub](https://github.com)
```

If a request fails, the error is used as the label:

```md
- [404 Not Found](https://example.com/missing)
- [Connection Error](https://fake-domain.com)
```

## Build

```bash
make build
```

## Test

```bash
make test
```

## Lint

```bash
make lint
```