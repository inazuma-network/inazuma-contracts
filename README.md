<h1 align="center">Inazuma Contracts</h1>

<p align="center">
  WebAssembly smart contracts for the <b>Inazuma</b> layer-1 chain —
  worked examples, the host ABI, and deploy scripts.
</p>

---

## How contracts work here

Inazuma runs **WASM**, not the EVM. A contract is a WASM module deployed to its own
address with its own key-value storage. Writes are transactions and cost gas in
INAZ; reads through `inaz_query` are free.

| Concept | Inazuma |
| --- | --- |
| Bytecode | WebAssembly |
| Languages | Rust (recommended), or hand-written `.wat`, or anything targeting `wasm32-unknown-unknown` |
| Storage | key → bytes, per contract |
| Read a contract | `inaz_query(address, method, args)` — free, no transaction |
| Write | signed transaction, metered by gas |
| Fungible tokens | built into the protocol — no contract needed |

## Examples

| Example | Language | What it shows |
| --- | --- | --- |
| [`examples/counter-wat`](examples/counter-wat) | hand-written `.wat` | The complete host ABI in ~100 readable lines: input, storage read/write, return, log |
| [`examples/counter-rust`](examples/counter-rust) | Rust | The same contract with a normal toolchain and a build script |

## Quick start (Rust)

```bash
rustup target add wasm32-unknown-unknown
cd examples/counter-rust
cargo build --release --target wasm32-unknown-unknown
# -> target/wasm32-unknown-unknown/release/counter.wasm
```

Deploy and call it:

```bash
INAZ_KEY=inazkey1... bun run scripts/deploy.ts examples/counter-rust/target/wasm32-unknown-unknown/release/counter.wasm
INAZ_KEY=inazkey1... bun run scripts/call.ts <contract-address> add:5
bun run scripts/query.ts <contract-address> get
```

## Host ABI

Contracts import these functions from `env`:

| Function | Signature | Purpose |
| --- | --- | --- |
| `inz_input_len` | `() -> i32` | Length of the call argument bytes |
| `inz_input` | `(ptr, len) -> i32` | Copy the call arguments into memory |
| `inz_read` | `(key_ptr, key_len, val_ptr, val_len) -> i32` | Read storage; returns bytes written, `-1` when absent |
| `inz_write` | `(key_ptr, key_len, val_ptr, val_len) -> i32` | Write storage |
| `inz_return` | `(ptr, len)` | Set the return value |
| `inz_log` | `(ptr, len)` | Emit a log line visible in receipts |

The module must export `memory` and an entry point. See
[docs/abi.md](docs/abi.md) for exact semantics and gas notes.

## Writing safe contracts

- Validate every input; a panic aborts the call and still consumes gas.
- Keep storage tiny — everyone who syncs pays for it forever.
- Update balances before external effects, never after.
- Dry-run with `inaz_simulateTransaction` before spending real fees.
- Do not invent your own fungible token when the protocol-level token does the job
  more cheaply (`inaz_tokens`, `inaz_tokenBalance`).

## Guides

- [Host ABI reference](docs/abi.md)
- [Deploying and calling](docs/deploying.md)
- [Network docs](https://github.com/inazuma-network/inazuma-docs)

## Ecosystem

| Repo | Purpose |
| --- | --- |
| [inazuma-core](https://github.com/inazuma-network/inazuma-core) | The Rust L1 node and WASM runtime |
| [inazuma-sdk](https://github.com/inazuma-network/inazuma-sdk) | TypeScript SDK used by the scripts here |
| [inazuma-wallet](https://github.com/inazuma-network/inazuma-wallet) | Wallet extension |
| [inazuma-docs](https://github.com/inazuma-network/inazuma-docs) | Network documentation |
| [inazuma-faucet](https://github.com/inazuma-network/inazuma-faucet) | Test-INAZ faucet |
| [inazuma-contracts](https://github.com/inazuma-network/inazuma-contracts) | This repo |

MIT licensed.

---

## Why Inazuma exists

Inazuma is a sovereign layer 1 — our own consensus, state machine, networking and VM, not
a rollup or a fork. The goal is narrow and deliberate: **be the home chain for memes,
NFTs, collectibles, games and communities.**

That use case is high volume and low value per transaction. A 500-piece mint, a game
writing a move a second, a community handing out collectibles — none of them can pay
dollars in fees or wait seconds for a confirmation. So the whole design is bent around
being fast and near-free:

| | |
| --- | --- |
| Block time | 400 ms, finalised in the same block |
| Transfer fee | ~0.000001 INAZ — fractions of a cent |
| Throughput | ~2,500 tx/s ingest; 20k-36k tx/s execution in bench |
| Tokens & NFTs | first-class chain records — no contract needed to mint |
| Contracts | gas-metered WASM |
| Accounts | Ed25519, base58 addresses, optional ML-DSA-65 co-signature |
| Light clients | sparse Merkle state proofs |

Getting to top-tier means three things, in this order: enough independent validators that
nobody can stop the chain, tooling good enough that a first-time builder ships in an
afternoon, and fees that stay boring even when a collection goes viral. Every repo below
is one part of that.

## The Inazuma repos

| Repo | What's in it |
| --- | --- |
| [inazuma-core](https://github.com/inazuma-network/inazuma-core) | The Rust L1: consensus, state, staking, P2P, JSON-RPC, WASM VM |
| [inazuma-validator](https://github.com/inazuma-network/inazuma-validator) | Node operators: one-command installer, systemd units, health checks, full guide |
| [inazuma-sdk](https://github.com/inazuma-network/inazuma-sdk) | TypeScript client: RPC, keys, signing, sign-in, state proofs |
| [inazuma-wallet](https://github.com/inazuma-network/inazuma-wallet) | Self-custody wallet: browser extension, web and Android |
| **inazuma-contracts** (here) | WASM contract examples, host ABI and deploy scripts |
| [inazuma-faucet](https://github.com/inazuma-network/inazuma-faucet) | Test-token faucet service |
| [inazuma-docs](https://github.com/inazuma-network/inazuma-docs) | All written guides, organised by role |
| [inazuma-improvement-proposals](https://github.com/inazuma-network/inazuma-improvement-proposals) | INAZIPs — how the chain changes |

## Getting started, whoever you are

| I want to… | Go to |
| --- | --- |
| Use a wallet and send INAZ | [inazuma-wallet](https://github.com/inazuma-network/inazuma-wallet) |
| Get test INAZ | [inazuma-faucet](https://github.com/inazuma-network/inazuma-faucet) |
| Build an app | [inazuma-sdk](https://github.com/inazuma-network/inazuma-sdk) · [inazuma-contracts](https://github.com/inazuma-network/inazuma-contracts) |
| Run a node or stake | [inazuma-validator](https://github.com/inazuma-network/inazuma-validator) |
| Understand the internals | [inazuma-core](https://github.com/inazuma-network/inazuma-core) |
| Propose a protocol change | [INAZIPs](https://github.com/inazuma-network/inazuma-improvement-proposals) |
