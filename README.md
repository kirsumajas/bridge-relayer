# bridge-relayer (skeleton)

Rust async relayer that watches TON & Solana, builds Borsh attestations, signs, and submits.

## Quick start

```bash
# 1) Toolchain
rustup default stable
cargo --version

# 2) New project & files
# (paste files from the spec)

# 3) Env
export SOL_RPC_HTTP=https://api.devnet.solana.com
export SOL_RPC_WS=wss://api.devnet.solana.com/
export SOL_BRIDGE_PROGRAM=<YourProgramId11111111111111111111111111111>
export TON_API_BASE=https://testnet.toncenter.com/api/v3
export TON_BRIDGE_ADDR_B64=<YourTonBridgeAddressBase64>
export RELAYER_SK_BASE64=<Base64Ed25519Secret>
export CFG_HASH_HEX=0000000000000000000000000000000000000000000000000000000000000000

# 4) Run
cargo run
