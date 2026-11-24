# Authorization matrix cookbook

## Multisig-like roles

For a multisig-style contract, model every signer as a peer and keep the outsider in the matrix's known actor set:

~~~rust
let peers = PeerActors::new(&env, 3);
let allowed = peers.peers().to_vec();
let all = peers.all();
let signer_set = peers.peers().to_vec();

AuthMatrix::new(&env)
    .entry_point("approve", &all, move |caller| {
        if signer_set.contains(caller) { Ok(()) } else { Err(auth_error()) }
    })
    .assert_enforced();
~~~

The `all` list intentionally seeds the outsider so unauthorized access is checked. For threshold multisig, keep threshold/approval assertions in the contract-specific closure and use the matrix to verify caller authorization.

## Different roles per entry point

Use `AdminUserActors` or `RbacActors` when the same contract exposes administrative and user operations. Register the narrowest allowed list for each entry point; the matrix cross-products every known actor against every entry point.

## Exact error expectations

Use `entry_point_with_expected_errors` when a denied caller must return a specific contract error. The expected text is matched against the Soroban error's debug representation.
