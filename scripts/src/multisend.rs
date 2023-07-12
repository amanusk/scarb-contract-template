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

pub async fn multisend() -> Result<()> {
    let envfile = EnvFile::new(&Path::new(".env"))?;

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(envfile.get("STARKNET_RPC_URL").unwrap()).unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();

    let dai_address = FieldElement::from_hex_be(
        "0x00da114221cb83fa859dbdb4c44beeaa0bb37c7537ad5ae66fe5e0efd20e6eb3",
    )
    .unwrap();

    // let usdc_address = FieldElement::from_hex_be(
    //     "0x005a643907b9a4bc6a55e9069c4fd5fd1f5c79a22470690f75556c4736e34426",
    // )
    // .unwrap();

    let token_sender_address = FieldElement::from_hex_be(
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

    let multisend_call = Call {
        to: token_sender_address,
        selector: get_selector_from_name("multisend").unwrap(),
        calldata: vec![
            FieldElement::from_dec_str("2").unwrap(),
            dai_address,
            FieldElement::from_dec_str("1").unwrap(),
            FieldElement::ZERO,
        ],
    };

    let result = account.execute(vec![multisend_call]).send().await;
    // .unwrap();
    println!("Result {:#?}", result);

    // println!("Removing in Tx: {:#064x}", result.transaction_hash);

    Ok(())
}
