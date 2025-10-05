<img align="left" src="www/logo.svg" alt="Tenkyo" width="150">

# Tenkyo

## Build

*Prerequisites: Rust 1.70+ with wasm32-unknown-unknown target and wasm-pack*

```bash
./build.sh && cd www && python3 -m http.server 8080
```

## URL Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | string | Target redirect URL |
| `delay` | number | Delay in seconds before redirect |
| `message` | string | Custom message to display |
| `cancel` | boolean | Show cancel button (add `&cancel` to enable) |

## Examples

```
?url=https://github.com&delay=5
?url=https://example.com&delay=10&cancel
?url=https://example.com&message=Please%20wait...&delay=5
```

## License

MIT
