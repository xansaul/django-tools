use cli_logic::cli::CliArgs;
use clap::Parser;
use resources_generator::generator::Generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CliArgs = CliArgs::parse();

    let result = Generator::generate_resource(args);

    match result {
        Ok(message) => println!("{message}"),
        Err(error) => println!("{error}")  
    }

    Ok(())
}


