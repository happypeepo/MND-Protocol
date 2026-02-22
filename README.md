# LatticePress

LatticePress is a production-ready, zero-copy transaction compression protocol for the Monad blockchain.

## Problem
Monad achieves 10,000 TPS through parallel execution. At this speed, 500-byte transactions bloat the network's history by 16.76 GB/hour.

## Solution
LatticePress intercepts transactions off-chain, compresses the payload (replacing long signatures with 1-byte dictionary IDs + applying Zstd to the rest), and decompress them directly in EVM memory using Yul before hitting `delegatecall`. This shrinks hourly bloat to 4.19 GB (75% reduction).

## Architecture
- **Packer (Rust Sidecar):** High-speed off-chain compression daemon utilizing `zstd` and dictionary substitution.
- **Unpacker (EVM Yul Gateway):** On-chain decompressor contract that reads compressed payloads and executes them via `delegatecall` entirely in memory.

## Getting Started

### Prerequisites
- [Foundry](https://getfoundry.sh/)
- [Rust & Cargo](https://rustup.rs/)

### Running the Demo
```bash
make all
```

### Validating the Live Test
Because LatticePress uses a `delegatecall` proxy pattern to minimize storage reading, all execution state changes happen on the **LatticeGateway** contract itself. The `LogicToken` simply acts as the reference bytecode.

```bash
# Don't check the Logic address! Check the Gateway address:
cast call <GATEWAY_ADDR> "balanceOf(address)(uint256)" <RECEIVER_ADDR> --rpc-url https://testnet-rpc.monad.xyz
```
