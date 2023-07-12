use envfile::EnvFile;
use std::{path::Path, sync::Arc};
use url::Url;

use eyre::Result;
use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, FieldElement},
    core::utils::get_selector_from_name,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider,
    },
    signers::{LocalWallet, SigningKey},
};

pub async fn approve() -> Result<()> {
    let envfile = EnvFile::new(&Path::new(".env"))?;

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(envfile.get("STARKNET_RPC_URL").unwrap()).unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();

    let dai_address = FieldElement::from_hex_be(
        "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
    )
    .unwrap();

    let usdc_address = FieldElement::from_hex_be(
        "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
    )
    .unwrap();

    let sender_address = FieldElement::from_hex_be(
        "0x02058f6050454efcde895decc689bc9458571091f5314b45b9aa123a9f00eb4a",
    )
    .unwrap();

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(envfile.get("PRIVATE_KEY").unwrap()).unwrap(),
    ));
    let address = FieldElement::from_hex_be(envfile.get("ACCOUNT_ADDRESS").unwrap()).unwrap();

    // TODO: set testnet/mainnet based on provider
    let mut account = SingleOwnerAccount::new(provider, signer, address, chain_id);

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let account = Arc::new(account);

    let approve_dai_call = Call {
        to: dai_address,
        selector: get_selector_from_name("approve").unwrap(),
        calldata: vec![
            sender_address,
            FieldElement::from_hex_be("0xffffffffffffffffffffffffffffffff").unwrap(),
            FieldElement::from_hex_be("0xffffffffffffffffffffffffffffffff").unwrap(),
        ],
    };

    let approve_usdc_call = Call {
        to: usdc_address,
        selector: get_selector_from_name("approve").unwrap(),
        calldata: vec![
            sender_address,
            FieldElement::from_hex_be("0xffffffffffffffffffffffffffffffff").unwrap(),
            FieldElement::from_hex_be("0xffffffffffffffffffffffffffffffff").unwrap(),
        ],
    };

    let result = account
        .execute(vec![approve_dai_call, approve_usdc_call])
        .fee_estimate_multiplier(3.0)
        .send()
        .await
        .unwrap();

    println!("Approved in Tx: {:#064x}", result.transaction_hash);

    Ok(())
}
