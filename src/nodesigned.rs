//call in web 3 and declare transaction value type.
use web3::types::{TransactionRequest, U256};

// call in tokio to use async function
#[tokio::main]

pub async fn main() -> web3::Result<()> {
//Declare web3 port
    let transport = web3::transports::Http::new("http://localhost:8545")?;
//Create new transport from port already declared
    let web3 = web3::Web3::new(transport);
//Connect to ETH accounts
    let mut accounts = web3.eth().accounts().await?;

//Let initial account be 100ETH from account 1
    let balance_before = web3.eth().balance(accounts[1], None).await?;

//Make transaction from account 0 to account 1
    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        value: Some(U256::from(10000)),
        data: None,
        nonce: None,
        condition: None
    };

//Retrieve transaction hash
    let tx_hash = web3.eth().send_transaction(tx).await?;

//Final balance
    let balance_after = web3.eth().balance(accounts[1], None).await?;
   
//Print hash, initial balance and final balance
    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);
    
       Ok(())
}
