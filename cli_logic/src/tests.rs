
#[cfg(test)]
mod tests {

    use std::{collections::HashMap, fs, io::Read, path::PathBuf};
    use tempfile::tempdir;

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
        
        let file_name: &str = "test-file-exists.txt";

        let mut path = PathBuf::from("./");
        let result = utils::create_file(&path, file_name);

        assert!(result.is_ok());

        
        let result = utils::create_file(&path, file_name);
        
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "File already exists.");
        
        path.push(file_name);
        
        fs::remove_file(&path).unwrap(); 
    }

    #[test]
    fn test_capitalize_first_letter() {
        let input = String::from("hello");
        let expected_output = "Hello".to_string();
        assert_eq!(utils::capitalize_first_letter(&input), expected_output);
    }

    #[test]
    fn test_remove_last_character() {
        let input = String::from("hello");
        let expected_output = "hell".to_string();
        assert_eq!(utils::remove_last_character(&input), expected_output);
    }

    #[test]
    fn test_create_files_and_write_content() {
        
        let temp_dir = tempdir().expect("failed to create temporary directory");
        let path = temp_dir.path().to_path_buf();
        
        let mut files = HashMap::new();
        files.insert("file1.txt", "Content of file1.txt".to_string());
        files.insert("file2.txt", "Content of file2.txt".to_string());

        let result = utils::create_files_and_write_content(&path, files);

        
        assert!(result.is_ok());

        for file_name in &["file1.txt", "file2.txt"] {
            let file_path = path.join(file_name);
            let mut file_content = String::new();
            let mut file = std::fs::File::open(&file_path).expect("failed to open file");
            file.read_to_string(&mut file_content).expect("failed to read file content");
            assert_eq!(file_content, format!("Content of {}", file_name));
        }
    }
}