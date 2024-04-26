use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use crate::files_to_generate::{ApiViewFiles, CommonFiles, SetViewFiles};
use crate::cli::{Args, Resource};
use crate::utils;


pub struct GenerateResource;


impl GenerateResource {
    

    pub fn generate_app(args: Args)-> Result<(), Box<dyn std::error::Error>> {
        match args.resource {
            Resource::ApiView => GenerateResource::case_api_view(args),
            Resource::SetView => GenerateResource::case_set_view(args)
        }
    }

    fn case_api_view(args: Args)-> Result<(), Box<dyn std::error::Error>> {

        let result = GenerateResource::generate_app_struct(&args);

        if let Err(error) = result {
            return Err(error);
        }

        let name = String::from(&args.name);
        let complete_path_app = result.unwrap();

        let create_files_result = GenerateResource::create_files(
            &complete_path_app,
            ApiViewFiles::files(&name),
        );
    
        if let Err(error) = create_files_result {
            return Err(error.into());
        }

        Ok(())
    }

    fn case_set_view(args: Args)-> Result<(), Box<dyn std::error::Error>> {
        
        let result = GenerateResource::generate_app_struct(&args);
        
        if let Err(error) = result {
            return Err(error);
        }

        let name = String::from(&args.name);
        let complete_path_app = result.unwrap();

        let create_files_result = GenerateResource::create_files(
            &complete_path_app,
            SetViewFiles::files(&name),
        );
    
        if let Err(error) = create_files_result {
            return Err(error.into());
        }
    
        Ok(())
    }

    fn generate_app_struct(args: &Args) -> Result<PathBuf, Box<dyn std::error::Error>> {

        let result = utils::create_folder(&args.path, &args.name);
        if let Err(error) = result {
            return Err(error.into());
        }
        
        let complete_path_app = args.path.join(&args.name);
        utils::create_folder(&complete_path_app, &"migrations".to_string())?;
    
        let base_files_result = GenerateResource::create_files(
            &complete_path_app,
            CommonFiles::files_no_variables(),
        );
        

        if let Err(error) = base_files_result {
            return Err(error.into());
        }
    

        Ok(complete_path_app)
    }

    fn create_files(complete_path_app: &PathBuf, files:  HashMap<&str, String>)-> Result<(), Box<dyn std::error::Error>>{
        for (file, content) in files{
           
            let result = utils::create_file(&complete_path_app, file);
            
            if let Err(error) = result {
                return Err(error.into());
            }

            let mut file = result.unwrap();

            file.write(content.as_bytes())?;
        }
        Ok(())
    }
    


}
