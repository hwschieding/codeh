use std::env;

mod cli;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    let raw_config = cli::RawArgs::from_args(args)?;

    println!("Cli started with conf {:?}", raw_config);

    // println!("Hello, world! {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
