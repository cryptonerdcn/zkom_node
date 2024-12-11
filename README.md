# ZKOM node(POC)

This is a demo node of ZKOM project, it will run ZK KYC computation.

## Installation

First, install `scarb` (we suggest using `2.8.2`) using the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh -s -- -v 2.8.2
```

## Usage

### Build guest_rs

In this demo you don't need to build `guest_rs` because it's already built in the `src/guest_rs_bytecode.cairo` file.
But if you want to build it, you can use the following command:

```sh
scarb riscv build
```

### Run `cairo_server`

To run the `cairo_server`, use the following command:

```sh
cd cairo_server
cargo run
```

This project is using:
- `scarb-riscv` and `riscairo` (<https://github.com/massalabs/riscairo>).
- `WASM-Cairo` (<https://github.com/cryptonerdcn/wasm-cairo>).

## TASKS:

- [x] Using Cairo to compute the dot product for the KYC.
- [ ] Using WASM-Cairo to run Cairo code in the Browser.
- [ ] Generate the proof.
- [ ] Verifier.
