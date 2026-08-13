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
