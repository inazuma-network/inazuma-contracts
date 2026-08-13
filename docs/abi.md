# Host ABI reference

A contract is a WASM module. The runtime supplies host functions in the `env`
module; the contract exports `memory` and its entry point.

## Imports

```wat
(import "env" "inz_input_len" (func $input_len (result i32)))
(import "env" "inz_input"     (func $input (param i32 i32) (result i32)))
(import "env" "inz_read"      (func $read  (param i32 i32 i32 i32) (result i32)))
(import "env" "inz_write"     (func $write (param i32 i32 i32 i32) (result i32)))
(import "env" "inz_return"    (func $ret   (param i32 i32)))
(import "env" "inz_log"       (func $log   (param i32 i32)))
```

| Function | Returns | Notes |
| --- | --- | --- |
| `inz_input_len()` | byte length | Call it before copying input |
| `inz_input(ptr, len)` | bytes copied | Truncates when `len` is smaller than the input |
| `inz_read(k_ptr, k_len, v_ptr, v_max)` | bytes written, or `-1` | `-1` means the key is unset — treat it as your zero value |
| `inz_write(k_ptr, k_len, v_ptr, v_len)` | `0` on success | Charged by key + value size |
| `inz_return(ptr, len)` | — | Last call wins; the bytes appear in the receipt and in `inaz_query` results |
| `inz_log(ptr, len)` | — | UTF-8 text, for debugging and indexing |

## Memory rules

- Grow memory yourself if you need more than the declared pages.
- The host never writes outside the pointer/length pairs you pass.
- Out-of-bounds pointers abort the call.

## Gas

Gas is charged for instructions executed plus storage bytes read and written.
Storage writes dominate; a loop over a large key set is the usual reason a call
runs out of gas. Reads via `inaz_query` are free because they never enter a block.

## Calling convention

Arguments are opaque bytes — a convention, not a schema. The examples use short
ASCII commands (`""`, `get`, `add:5`) so they stay readable. For real contracts,
pick a compact encoding and version it in the first byte.

## Determinism

No clocks, no randomness, no network, no floating-point reliance. Every node must
compute the same result, so anything non-deterministic is unavailable by design.
Need randomness? Use a commit-reveal scheme across two transactions.
