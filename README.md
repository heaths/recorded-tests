# Recorded tests

This is a prototype for playing back and recording tests. We use something similar in other Azure SDK languages.
The idea is to provide enough context to set up an HTTP transport that will record live tests against provisioned resources,
or play back tests using recorded data depending on the `AZURE_TEST_MODE` environment variable:

Value | Description
--- | ----
playback | Use recorded data to run tests. This is the default if `AZURE_TEST_MODE` is undefined.
record | Record sanitized data when running tests. This requires provisioned resources.
live | Run tests against provisioned resources without recording data.

## Writing tests

Use the `#[recorded]` attribute macro to declare synchronous and asynchronous tests. Asynchronous tests will use `tokio`.

```rust
use recorded_tests_macros::recorded;

#[recorded] // -> #[test]
fn test_sync() {
    todo!();
}

#[recorded] // -> #[tokio::test]
async fn test_async() {
    todo!();
}
```

### TestContext

You can also declare and use a `TestContext` parameter. The test function signature will be rewritten to set up the `TestContext`
and clear parameters for a valid test signature.

```rust
use recorded_tests_core::TestContext;
use recorded_tests_macros::recorded;

#[recorded]
async fn test_async(ctx: TestContext) {
    assert_eq!(ctx.test_name(), "test_async");
    todo!();
}
```

### Live-only tests

If you have tests that require special set up or provisioned resources, you can pass attribute the test with `#[recorded(live)]`:

```rust
use recorded_tests_macros::recorded;

#[recorded(live)] // -> #[test] #[ignore = "skipping live tests"]
fn test_live_only() {
    todo!();
}
```

This adds an `#[ignore]` attribute so that `cargo test` output still shows a test was discovered but skipped.

## Examples

Run tests in playback mode:

```bash
cargo test
```

Run tests in record mode:

```bash
AZURE_TEST_MODE=record cargo test
```

Run tests in live mode:

```bash
AZURE_TEST_MODE=live cargo test
```
