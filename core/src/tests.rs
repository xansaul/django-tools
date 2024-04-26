
#[cfg(test)]
mod tests {

    use std::{fs, path::PathBuf};
    use crate::utils; 
 

    #[test]
    fn must_create_folder() {
        let folder_name: String = String::from("test-folder");
        let path: PathBuf = PathBuf::from("./");
        let complete_path = path.join(&folder_name);

        assert!(!complete_path.exists());
        let result = utils::create_folder(&path, &folder_name);

        assert!(path.exists());
        assert!(path.is_dir());

        assert_eq!(result.unwrap(), "App created successfully");

        fs::remove_dir(&complete_path).unwrap();

        assert!(!complete_path.exists());
    }

    #[test]
    fn must_return_error_if_folder_exists(){

        let folder_name: String = String::from("test-folder-exists");
        let path: PathBuf = PathBuf::from("./");
        let complete_path = path.join(&folder_name);


        assert!(!complete_path.exists());
        utils::create_folder(&path, &folder_name).unwrap();
        let result = utils::create_folder(&path, &folder_name);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Folder already exists.");

        fs::remove_dir(&complete_path).unwrap();

        assert!(!complete_path.exists());
    }

    #[test]
    fn must_create_file(){
        let file_name = "test-file.txt";

        let mut path = PathBuf::from("./");

        let result = utils::create_file(&path, file_name);

        assert!(result.is_ok());
        assert!(path.exists());
        
        path.push(file_name);

        fs::remove_file(&path).unwrap();
        

        assert!(!path.exists());
    }

    #[test]
    fn must_return_error_if_file_exists(){

        let file_name = "test-file-exists.txt";

        let mut path = PathBuf::from("./");
        let result = utils::create_file(&path, file_name);

        assert!(result.is_ok());

        
        let result = utils::create_file(&path, file_name);
        
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "File already exists.");
        
        path.push(file_name);
        
        fs::remove_file(&path).unwrap(); 
    }
}