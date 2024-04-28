use cli_logic::cli::{CliArgs, Resource};
use crate::{apiview::codegen::generate_app_apiview, project::create_project::create_project, viewset::codegen::generate_app_viewset};

pub struct Generator;


impl Generator {

    pub fn generate_resource(args: CliArgs)-> Result<&'static str, Box<dyn std::error::Error>> {
        match args.resource {
            Resource::ApiView(args) => generate_app_apiview(&args),
            Resource::ViewSet(args) => generate_app_viewset(&args),
            Resource::Project(args) => create_project(&args)
        }
    }
}


