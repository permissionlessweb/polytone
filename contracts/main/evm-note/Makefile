union_version := v0.24.0
union_cmd := docker run --rm --network host -v /tmp:/tmp -v $(PWD)/cosmwasm/build:/build -v $(PWD)/cosmwasm/.keyring:/keyring ghcr.io/unionlabs/uniond-release:${union_version}

rpc_url := https://union-testnet-rpc.polkachu.com:443
chain_id := union-testnet-8
account_name := $(shell xxd -l16 -ps /dev/urandom)
contract_label := $(shell xxd -l16 -ps /dev/urandom)
contract_salt := $(shell xxd -l16 -ps /dev/urandom)
admin := union18k2uq7srsr8lwrae6zr0qahpn29rsp7tjznqdt
# Sepolia addresses
counter := 0x785B548D3d7064F77A26e479AC7847DBCE0c1B46
string_holder := 0xF67F4E25067b5B1D36EC8269cC8CE955C820680A
sender_counter := 0x313ff8Bf109bcEF105C3183039beFf0527d2cc38


channel_state_init := 1
channel_order_unordered := 1
# Pre-created by Union
channel_connection := connection-5


.PHONY: build deploy

build:
	mkdir -p build && RUSTFLAGS='-C target-cpu=mvp -C opt-level=z' cargo build --target wasm32-unknown-unknown --no-default-features --lib --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort && wasm-opt ../../target/wasm32-unknown-unknown/release/evm_note.wasm -o ../../artifacts/evm_note.wasm -O3 --converge

build-not-nix:
	RUSTFLAGS='-C link-arg=-s' cargo wasm && \
	    cp ../../target/wasm32-unknown-unknown/release/evm_note.wasm ../../artifacts/evm_note.wasm

deploy: build
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm store /build/contract.wasm --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value' | \
		xargs -I{} ${union_cmd} tx wasm instantiate2 {} '{ "config": { "protocol_timeout": 36000000000000 }}' ${contract_salt} --label ${contract_label} --admin ${admin} --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value' | \
		xargs -I{} echo "Contract address {}" && \
		rm -rf /keyring/*

upload: build
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm store /build/contract.wasm --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value'

migrate:
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm migrate ${COSMWASM_CONTRACT_ADDRESS} ${CODE_ID} '{}' --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value'

interact:
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm execute ${COSMWASM_CONTRACT_ADDRESS} '{"execute":{"msg":{"call":{"to":"${counter}","data":"b49004e9"}},"callback":null,"timeout_seconds":50000000000000}}' --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} -b sync

update-string:
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm execute ${COSMWASM_CONTRACT_ADDRESS} '{"execute":{"msg":{"call":{"to":"${string_holder}","data":"4ed3885e0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002b7468697320697320612064656d6f206f662073657474696e67206120737472696e67206f76657220494243000000000000000000000000000000000000000000"}},"callback":null,"timeout_seconds":50000000000000}}' --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} -b sync

count:
	${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm execute ${COSMWASM_CONTRACT_ADDRESS} '{"execute":{"msg":{"call":{"to":"${sender_counter}","data":"5b34b966"}},"callback":null,"timeout_seconds":50000000000000}}' --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} -b sync

# channel-init:
#     ${union_cmd} keys import-hex ${account_name} ${COSMOS_KEY} --keyring-backend test --keyring-dir /keyring && \
#         ${union_cmd} tx ibc channel open-init ${channel_connection} ${contract_address} --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \

check-channel:
	${union_cmd} --node ${rpc_url} query ibc channel channels  --limit 200 --output json | jq -r '.channels[] | select(.counterparty.port_id == "${CONTRACT_ADDRESS}")'

list-channels:
	${union_cmd} --node ${rpc_url} query ibc channel channels --limit 200 --output json | jq -r '.channels[] | select(.version | startswith("ucs01"))'
