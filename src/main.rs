use ethers::{
    providers::{Provider, Http},
    utils::{format_units}, 
    contract::{abigen, Eip712, EthAbiType},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer},
    types::{U256, Address},
    core::{
        utils::hex,
    },
};


use std::{
    time::{SystemTime, UNIX_EPOCH},
    str::FromStr,
    convert::TryFrom,
    sync::Arc,
};

use serde::{Serialize, Deserialize};
use clap::Parser;
use reqwest;

#[derive(Eip712, EthAbiType, Clone)]
#[eip712(
    name = "Farcaster name verification",
    version = "1",
    chain_id = 1,
    verifying_contract = "0xe3be01d99baa8db9905b33a3ca391238234b79d1"
)]
struct UserNameProof {
    name: String,
    timestamp: U256,
    owner: Address,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransferRequestBody {
    from: u64,
    to: u64,
    fid: u64,
    name: String,
    timestamp: u64,
    owner: String,
    signature: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser)]
    name: String,
    #[clap(long, value_parser, default_value = "1")]
    storage: u64,
    #[clap(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    set_fname: bool,
    #[clap(long, value_parser)]
    fid: Option<u64>,    
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
        let args = Args::parse();

        let provider = Provider::<Http>::try_from("https://optimism.meowrpc.com")?;
        // not using dotenv here to easily switch private keys
        let priv_key = std::env::var("PKEY")
            .expect("Missing Private Key");
            
        let wallet = priv_key.parse::<LocalWallet>()?
            .with_chain_id(10u64);

        let client = Arc::new(provider);

        println!("Using Address: {:?}", wallet.address());

        if args.set_fname {
            if let Some(fid) = args.fid {
                println!("Setting name for FID: {:?}", fid);
                set_fname(&wallet, &args.name, U256::from(fid)).await?;
            } else {
                println!("Please provide a FID to set the name for");
            }
        } else {
            register_name(&wallet, &client, &args.name, args.storage).await?;
        }
        
        Ok(())
}

async fn register_fid(signer: &SignerMiddleware<Arc<Provider<Http>>, LocalWallet>, storage_size: U256) -> eyre::Result<U256> {
    abigen!(
        IDGATEWAY,
        r#"[
            function price(uint256 extraStorage) returns (uint256)
            function register(address recovery) returns (uint256, uint256)
        ]"#,
    );

    let id_gateway = Address::from_str("0x00000000Fc25870C6eD6b6c7E41Fb078b7656f69")?;
    let contract = IDGATEWAY::new(id_gateway, signer.clone().into());

    let price = contract.price(storage_size).call().await?;
    println!("Price: {:?}", format_units(price, "ether").unwrap().parse::<f64>().unwrap());

    let (fid, _timestamp) = contract.register(signer.address()).value(price).call().await?;

    let receipt = contract.register(signer.address()).value(price).send().await?.await?;
    if let Some(receipt) = receipt {
        println!("Transaction Hash: 0x{:x}", receipt.transaction_hash);
    } else {
        println!("Transaction Failed");
        std::process::exit(1);
    }
    Ok(fid)
}

async fn register_name(wallet: &LocalWallet, client: &Arc<Provider<Http>>, name: &String, storage_size: u64) -> eyre::Result<()> {
        let signer = 
            SignerMiddleware::new(client.clone(), wallet.clone());
            
        // register FID to address
        let fid = register_fid(&signer, U256::from(storage_size)).await?;
        println!("Registered FID: {:?}", fid);
        
        // get the current epoch time   
        let timestamp = get_timestamp()?;

        // sign the registration eip712 hash
        let signature = sign_register(wallet, name.to_string(), U256::from(timestamp)).await?;

        let signature = format!("0x{}", signature);
        let address = format!("{:?}", signer.address());
        
        // post the registration to the server claiming a "name" for the FID registered to our address
        let body = TransferRequestBody {
            from: 0,
            to: fid.as_u64(),
            fid: fid.as_u64(),
            name: name.to_string(),
            timestamp: timestamp,
            owner: address,
            signature: signature,
        };

        post_register(body).await?;
        Ok(())
}

async fn set_fname(wallet: &LocalWallet, name: &String, fid: U256) -> eyre::Result<()> {
    let timestamp = get_timestamp()?;
    let signature = sign_register(&wallet, name.to_string(), U256::from(timestamp)).await?;
    let signature = format!("0x{}", signature);
    let address = format!("{:?}", wallet.address());

    let body = TransferRequestBody {
        from: 0,
        to: fid.as_u64(),
        fid: fid.as_u64(),
        name: name.to_string(),
        timestamp: timestamp,
        owner: address,
        signature: signature,
    };

    post_register(body).await?;
    Ok(())
}

async fn sign_register(wallet: &LocalWallet, name: String, timestamp: U256) -> eyre::Result<String> {
    let register = UserNameProof {
        name: name,
        timestamp: timestamp,
        owner: wallet.address(),
    };

    let signature_full = wallet.sign_typed_data(&register).await?;
    let sig_bytes = signature_full.to_vec();
    
    Ok(hex::encode(sig_bytes))
}

async fn post_register(body: TransferRequestBody) -> eyre::Result<()> {
    let client = reqwest::Client::new();
    let res = client.post("https://fnames.farcaster.xyz/transfers")
        .json(&body)
        .send()
        .await?;

    println!("FNames Api Response: {:#?}", res.text().await?);
    Ok(())
}

fn get_timestamp() -> eyre::Result<u64> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    Ok(since_the_epoch)
}
