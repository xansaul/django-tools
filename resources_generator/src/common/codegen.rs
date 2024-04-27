use std::path::PathBuf;
use super::files_to_generate::CommonFiles;
use cli_logic::cli::ApiType;
use cli_logic::utils;


pub struct GenerateCommonFiles;


impl GenerateCommonFiles {
    
    pub fn generate_common_files(args: &ApiType) -> Result<PathBuf, Box<dyn std::error::Error>> {

        let result = utils::create_folder(&args.path, &args.name);
        if let Err(error) = result {
            return Err(error.into());
        }
        
        let complete_path_app = args.path.join(&args.name);
        utils::create_folder(&complete_path_app, &"migrations".to_string())?;
    
        let base_files_result = utils::create_files_and_write_content(
            &complete_path_app,
            CommonFiles::get_files(),
        );
        

        if let Err(error) = base_files_result {
            return Err(error.into());
        }
    

        Ok(complete_path_app)
    }

}
