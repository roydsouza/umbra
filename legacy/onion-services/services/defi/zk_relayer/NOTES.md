# ZK Relayer

Privacy Pool / Railgun relayer accepting ZK-SNARK proofs via `.onion`.

## Concept

A relayer that broadcasts shielded transactions to the public mempool without seeing the network origin of the request.

## Implementation

- Accept ZK proofs via hidden service endpoint
- Validate proofs locally
- Broadcast to public mempool via Tor exit or direct connection
- Charge relay fee in shielded tokens

## Value

- Relayer never sees requester's IP during unshielding
- Resistant to correlation attacks
- Enables truly private DeFi withdrawals

## Key Location

`~/antigravity/umbra/keys/defi/zk_relayer.key`
