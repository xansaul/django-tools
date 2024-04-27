use cli_logic::cli::{CliArgs, Resource};
use crate::{apiview::codegen::generate_app_apiview, setview::codegen::generate_app_setview};


pub struct Generator;


impl Generator {

    pub fn generate_resource(args: CliArgs)-> Result<(), Box<dyn std::error::Error>> {
        match args.resource {
            Resource::ApiView(args) => generate_app_apiview(args),
            Resource::SetView(args) => generate_app_setview(args),
            Resource::Project(_) => Ok(())
        }
    }
}


