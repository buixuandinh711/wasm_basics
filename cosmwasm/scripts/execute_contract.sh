wasmd tx wasm execute $1 "$2" --from alice --gas auto --gas-adjustment 1.15 --yes

address=$(wasmd query wasm list-contract-by-code $1 --output json | jq -r '.contracts[-1]')