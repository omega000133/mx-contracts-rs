WALLET="${PWD}/wallet.pem"
PROJECT="${PWD}"
PROXY=https://testnet-gateway.multiversx.com
CHAINID=D

DEPLOY_GAS="25000000"
SFT_IDENTIFIER=0x585354525245504149522d653162363733 #XSTRREPAIR-e1b673
deploy() {
    mxpy --verbose contract deploy \
          --bytecode="output/on-chain-claim.wasm" \
          --arguments ${SFT_IDENTIFIER} \
          --pem=${WALLET} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} \
          --chain=${CHAINID} \
          --recall-nonce \
          --send \
          --outfile="deploy-testnet.interaction.json" || return

    TRANSACTION=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-testnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
