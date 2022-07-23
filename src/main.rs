use anyhow::Result;

mod wallet_lib;

fn main() -> Result<()> {
    let keypair = wallet_lib::create_keypair();

    println!("keypair:{:?}", keypair);

    Ok(())
}
