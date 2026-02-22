.PHONY: deploy-logic deploy-gateway run-packer live-test test-contracts all

# Monad Testnet RPC
RPC_URL = https://testnet-rpc.monad.xyz

# Include .env file if it exists
-include contracts/.env

all: deploy-logic deploy-gateway run-packer

deploy-logic:
	@echo "Deploying LogicToken to Monad Testnet..."
	cd contracts && ETH_RPC_URL=$(RPC_URL) forge create src/LogicToken.sol:LogicToken --private-key $(PRIVATE_KEY) --broadcast

deploy-gateway:
ifndef LOGIC_ADDR
	$(error LOGIC_ADDR is undefined. Usage: make deploy-gateway LOGIC_ADDR=0x...)
endif
	@echo "Deploying LatticeGateway to Monad Testnet targeting logic $(LOGIC_ADDR)..."
	cd contracts && ETH_RPC_URL=$(RPC_URL) forge create src/LatticeGateway.sol:LatticeGateway --private-key $(PRIVATE_KEY) --broadcast --legacy --constructor-args $(LOGIC_ADDR)
	
run-packer:
	@echo "Starting LatticePress Sidecar (Local Demo)..."
	cd sidecar && env CARGO_HOME=$(PWD)/.cargo TMPDIR=/tmp cargo build --release
	cd sidecar && ./target/release/sidecar --demo

live-test:
ifndef GATEWAY_ADDR
	$(error GATEWAY_ADDR is undefined. Usage: make live-test GATEWAY_ADDR=0x...)
endif
	@echo "Minting tokens to Wallet via Gateway..."
	cd contracts && ETH_RPC_URL=$(RPC_URL) cast send $(GATEWAY_ADDR) "mint(address,uint256)" 0xF8E9ED8c133659dfE1A509649454E22f5A905C99 500000000000000000000 --private-key $(PRIVATE_KEY)
	@echo "Sending LIVE compressed transaction to Monad Testnet..."
	cd sidecar && env CARGO_HOME=$(PWD)/.cargo TMPDIR=/tmp cargo build --release
	cd sidecar && ./target/release/sidecar --live $(GATEWAY_ADDR)

test-contracts:
	@echo "Running Yul Decompression tests..."
	cd contracts && forge test -vvv
