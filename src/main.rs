use clap::Parser;
use ethers::prelude::*;
use ethers::{
    types::U256,
    utils::format_units,
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    address: String,
    contract_address: String,
}

const RPC_URL: &str = "https://eth.llamarpc.com";

// fn rust_file_generation() -> Result<()> {
//     let abi_source = "./contracts/IERC20.json";
//     let out_file = std::env::temp_dir().join("ierc20.rs");
//     if out_file.exists() {
//         std::fs::remove_file(&out_file)?;
//     }
//     Abigen::new("IERC20", abi_source)?.generate()?.write_to_file(out_file)?;
//     Ok(())
// }

// fn rust_inline_generation_from_abi() {
//     abigen!(IERC20, "./contracts/IERC20.json");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let provider = Provider::<Http>::try_from(RPC_URL)?;

    // use ethers to read user balance from a wallet string
    let wallet = args.address.parse::<Address>()?;
    let contract_address = args.contract_address.parse::<Address>()?;
    let balance: U256 = provider.get_balance(wallet, None).await?;
    let num: String = format_units(balance, "ether").unwrap();

    println!("Your Ether Balance: {}", num);

    abigen!(
        IERC20,
        r#"[
            function totalSupply() external view returns (uint256)
            function name() external view returns (string memory)
            function decimals() external view returns (uint32)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );

    let client = Arc::new(provider);
    let contract = IERC20::new(contract_address, client);

    let balance: U256 = contract.balance_of(wallet).call().await?;
    let token_name: String = contract.name().call().await?;
    let decimals: u32 = contract.decimals().call().await?;
    
    let formatted_balance: String = format_units(balance, decimals).unwrap();

    println!("Your {} Balance: {}",token_name, formatted_balance);

    Ok(())
}