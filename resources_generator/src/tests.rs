
#[cfg(test)]
mod tests {
    
    use std::{fs, path::PathBuf};
    
    use crate::apiview::codegen::generate_app_apiview;
    use crate::setview::codegen::generate_app_setview;
    use crate::project::create_project::create_project;
    use cli_logic::{cli::{ApiType, ProjectAction}, utils};

    #[test]
    #[ignore]
    fn must_create_project(){
        let args:ProjectAction = ProjectAction{
            name: "projectTest".to_string(),
            path: PathBuf::from("./")
        };

        let result = create_project(&args);
        assert!(result.is_ok());
        
        let complete_path_project = args.path.join(format!("./{}",args.name));
        
        fs::remove_dir_all(&complete_path_project).unwrap();

        assert!(!complete_path_project.exists());
    }

    #[test]
    fn must_return_error_create_project(){
        let args:ProjectAction = ProjectAction{
            name: "projectTest".to_string(),
            path: PathBuf::from("./")
        };
        utils::create_folder(&args.path,&args.name).unwrap();

        let result = create_project(&args);

        assert!(result.is_err());

        let complete_path_project = args.path.join(&args.name);

        fs::remove_dir(&complete_path_project).unwrap();
        assert!(!complete_path_project.exists());
    }

    #[test]
    fn must_create_apiview_app(){

        let args = ApiType {
            name: "appTestApiView".to_string(),
            path: PathBuf::from("./")
        };

        let complete_path = args.path.join(&args.name);

        let result = generate_app_apiview(&args);

        assert!(result.is_ok());

        fs::remove_dir_all(&complete_path).unwrap();

        assert!(!complete_path.exists());
    }

    #[test]
    fn must_return_error_apiview_app(){

        let args = ApiType {
            name: "appTest".to_string(),
            path: PathBuf::from("./this-path-doesn't-exists")
        };

        
        let result = generate_app_apiview(&args);
        assert!(result.is_err());
        
        let complete_path_app = args.path.join(&args.name);
        assert!(!complete_path_app.exists());
    }

    #[test]
    fn must_create_setview_app(){

        let args = ApiType {
            name: "appTestSetView".to_string(),
            path: PathBuf::from("./")
        };

        let complete_path = args.path.join(&args.name);

        let result = generate_app_setview(&args);

        assert!(result.is_ok());

        fs::remove_dir_all(&complete_path).unwrap();

        assert!(!complete_path.exists());
    }

    #[test]
    fn must_return_error_setview_app(){

        let args = ApiType {
            name: "appTest".to_string(),
            path: PathBuf::from("./this-path-doesn't-exists")
        };

        let result = generate_app_setview(&args);
        assert!(result.is_err());
        
        let complete_path_app = args.path.join(&args.name);
        assert!(!complete_path_app.exists());
    }

}