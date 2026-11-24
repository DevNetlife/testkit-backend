# AuthMatrix and TTL hardening

## AuthMatrix caller sets

Authorization tests often share the same operator, outsider, and admin actors across several contracts. A caller set should be registered once and included in every entry-point cross-product without placing the callers in an allow-list. This prevents an outsider from being accidentally omitted from a matrix.

Use the contract address as diagnostic context when a test covers multiple deployed contracts. Failure output should identify the contract, entry point, caller, expected outcome, and observed error.

## TTL ledger preservation

TTL helpers must advance only the ledger sequence and timestamp needed to cross an expiry boundary. All unrelated ledger fields—protocol version, base reserve, network id, and configured TTL limits—must survive the operation unchanged.

Instance storage has one TTL per contract instance, not one TTL per key. APIs that accept a key for consistency must ignore it for instance storage and must not report the key as a keyed entry. Persistent and temporary storage continue to use the supplied key.

Tests should snapshot the complete ledger info before and after expiry, assert preservation of unrelated fields, and exercise instance TTL operations with different dummy keys to prove they resolve to the same instance entry.
