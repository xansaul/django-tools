
use core::cli::Args;
use core::codegen::GenerateResource;
use clap::Parser;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let result = GenerateResource::generate_app(args);

    match result {
        Ok(()) => println!("App created."),
        Err(error) => println!("{error}")  
    }

    Ok(())
}


