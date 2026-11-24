# Async-compatible authorization invocation adapters

`AuthMatrix` intentionally accepts a synchronous closure because Soroban's in-process test environment is synchronous and the workspace has no async runtime dependency. This keeps tests deterministic and within the crate's scope.

When an async client is involved, adapt it at the test boundary:

1. Build the transaction or invocation asynchronously in the client test.
2. Await it before registering the matrix closure.
3. Pass a synchronous closure that invokes the prepared Soroban call.
4. Keep network, runtime, and wallet concerns outside `AuthMatrix`.

A future adapter can be added without changing the matrix API by introducing a feature-gated executor integration. It must not create a default runtime or require network access in tests.
