# Deploying and calling contracts

## 1. Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Keep the artifact small: `opt-level = "z"`, `lto = true`, `panic = "abort"`, and
strip debug info. Smaller bytecode is cheaper to deploy and faster to load.

## 2. Deploy

```bash
INAZ_KEY=inazkey1... bun run scripts/deploy.ts path/to/contract.wasm
```

The script signs locally, submits a deploy transaction and prints the contract
address. Fund the deploying wallet first ([faucet](https://github.com/inazuma-network/inazuma-faucet)).

## 3. Call (write)

```bash
INAZ_KEY=inazkey1... bun run scripts/call.ts <address> "add:5"
```

Writes are transactions: they take one block (~400 ms) and cost gas. Simulate
first to catch a revert for free.

## 4. Query (read)

```bash
bun run scripts/query.ts <address> get
```

Reads never enter a block and cost nothing.

## 5. Inspect

| Call | Shows |
| --- | --- |
| `inaz_getContract(address)` | Code hash, deployer, creation height |
| `inaz_contractStorage(address, key)` | A raw storage value |
| `inaz_contracts()` | Deployed contracts on the chain |
| `inaz_getReceipt(hash)` | Gas used, logs, return value |

## Troubleshooting

| Symptom | Cause | Fix |
| --- | --- | --- |
| `out of gas` | Loop or oversized storage write | Reduce work per call, or split across calls |
| `trap` / `unreachable` | A panic or out-of-bounds access | Validate inputs; check pointer lengths |
| `invalid module` | Missing `memory` export or unsupported feature | Build for `wasm32-unknown-unknown`, no threads/SIMD |
| Deploy accepted, calls do nothing | Entry point not exported | Export the entry function the runtime expects |
