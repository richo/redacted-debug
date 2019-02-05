A generic mechanism for redacting things in your Debug impls

There is a small example that shows how to use it, but generally speaking:

```rust
use redacted_debug::RedactedDebug;

#[derive(RedactedDebug)]
struct User {
    username: String,
    #[redacted]
    password: String,
    /* ... */
}
```

This will give you a Debug implementation, but when printed the `password` field will be `"..."`.
