# AuthMatrix focused execution and authorization trees

The authorization helpers support focused verification in addition to a full
matrix run:

- select one registered entry point when a test only needs one caller path;
- compare the recorded authorization tree with an expected tree, including
  nested invocations and their argument values;
- compose shared role fixtures before registering contract-specific entry
  points, so the same admin/operator/outsider set is exercised consistently.

TTL tests should distinguish a key that is absent from a valid contract from a
target that cannot be inspected. The checked TTL API reports these as separate
typed errors, while the panic-oriented helper preserves the same distinction in
its failure text.
