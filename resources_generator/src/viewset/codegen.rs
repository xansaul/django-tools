use cli_logic::{cli::ApiType, utils};
use crate::common::codegen::GenerateCommonFiles;

use super::files_to_generate::ViewSetFiles;



pub fn generate_app_viewset(args: &ApiType)-> Result<&'static str, Box<dyn std::error::Error>> {
        
    let result = GenerateCommonFiles::generate_common_files(&args);
    
    if let Err(error) = result {
        return Err(error);
    }

    let name = String::from(&args.name);
    let complete_path_app = result.unwrap();

    let create_files_result = utils::create_files_and_write_content(
        &complete_path_app,
        ViewSetFiles::files(&name),
    );

    if let Err(error) = create_files_result {
        return Err(error.into());
    }

    Ok("ViewSet app created.")
}
