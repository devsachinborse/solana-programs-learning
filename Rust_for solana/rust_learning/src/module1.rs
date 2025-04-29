
fn primetive_data_types() {
    let token_supply: u128 = 1_000_000_000_000_000_000;
    let block_number:i64 = -1234567890;

    println!("Token Supply is: {}", token_supply);
    println!("Block number: {}", block_number);

    let token_price : f32 = 3.13;
    let trasactiion_fees: f64 = 0.00001;
    println!("Tx fee : {}", trasactiion_fees );
    println!("Token price: {}", token_price);


    let is_trasaction_valid :bool = true;
    println!("is Tx valid? {}", is_trasaction_valid);

    let token_symbol:char = 'S';
    println!("Token symbol: {}", token_symbol);

    let wallet_address: &str = "0x838fosifds889sdf8"; // this is slice of string
    let contract_name:String = String::from("descentralize exchange");

    println!("wallet address: {}", wallet_address);
    println!("Contract_name: {}", contract_name);
}


pub fn arithmatic_data_type() {
    let account_balance:i32 = 1000;
    let transaction_amount:i32 = 300;

    println!("Account Balance : {},\ntransaction Amount: {}", account_balance, transaction_amount);

    println!("New Account balance: {}",account_balance - transaction_amount);

    println!("Increment balance: {}", account_balance * 2);

    println!("Division for share discribution: {}", account_balance  / 4);

    println!("Reaimder when dividing trasaction fees: {}" , account_balance % 3 );


    let gas_price :f64 = 0.000000012;
    let gas_used : f64= 21000.0;
    println!("Gas Price = {} , Gas Used = {}" , gas_price, gas_used);
    println!("Toatal Gas fees = {:.8}", gas_price * gas_used)
}


pub fn logical_operations() {
    let is_staking :bool = true;
    let has_sufficent_blance: bool = false;

    println!("Is stacking = {}, has sufficient balance = {}", is_staking,has_sufficent_blance );
    println!("Can perfrom staking: {}", is_staking && has_sufficent_blance);
    println!("Can either perform stacking or withdraw: {}", is_staking  ||has_sufficent_blance);
    println!("Negative Staking status: is_statking = {}", !is_staking);
}

 
pub fn demo(){
    println!("\n");
    primetive_data_types();
    println!("******************************************");
    println!("\n");


    arithmatic_data_type();
    println!("******************************************");
    println!("\n");


    logical_operations();
    println!("******************************************");
}



