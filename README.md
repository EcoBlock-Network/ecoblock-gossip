# ecoblock-gossip

Implements the gossip protocol used to broadcast and replicate Tangle data between mobile nodes in a decentralized mesh.

Purpose
-------
- Handle P2P diffusion of Tangle blocks and metadata across a mesh of peers.
- Provide peer discovery, connection management and message relaying with simple anti-spam/backoff.
- Validate incoming messages (canonical payload, signature) using `ecoblock-core` and `ecoblock-crypto`.

What lives here
---------------
- `src/` — gossip protocol implementation, peer and connection management, and basic routing.
- `src/protocol` — wire formats and canonical encodings for gossip messages.
- `src/peer` — peer discovery, addressing and connection helpers.
- `src/mesh` — mesh maintenance, relay logic and heuristics (fanout, TTL, backoff).

Stability contract
------------------
The network wire format and the canonical encoding used for message IDs and signed payloads must remain stable between releases unless a clear migration and compatibility plan is provided. Changes to the gossip wire format or canonical payloads must be accompanied by:

- a documented migration strategy, and
- regression vectors and tests that demonstrate interop between old and new formats.

Quick example
-------------
Conceptual example showing how a node might publish a block to the mesh (API names are illustrative):

```rust
// Conceptual example — adapt to actual crate APIs
// let node = GossipNode::bind("0.0.0.0:9000")?;
// node.connect("/ip4/1.2.3.4/tcp/9000")?;
// node.gossip(block)?; // publish a signed Tangle block to peers
```

Running tests
-------------
Run the crate tests locally:

```bash
cd libs/ecoblock-gossip
cargo test
```

Note: integration tests that exercise real P2P behavior may require networking (multiple processes or loopback addresses); keep unit tests isolated where possible.

Contributing
------------
- Keep wire formats and canonical encodings stable. If you need to change them, include a migration plan and regression vectors.
- Add unit tests for any change to the protocol encoding or validation logic.
- Add integration tests that demonstrate message propagation across a small simulated mesh (2–4 nodes) when adding behavioral changes.

License
-------
MIT
