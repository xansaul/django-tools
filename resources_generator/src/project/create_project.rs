use std::process::Command;
use cli_logic::{cli::ProjectAction, utils::create_folder};
use std::env;


pub fn create_project(args: ProjectAction)-> Result<(), Box<dyn std::error::Error>> {

    let complete_path_project = args.path.join(&args.name);

    if let Err(error) = create_folder(&args.path, &args.name) {
        return Err(error);
    }

    if let Err(error) = env::set_current_dir(&complete_path_project) {
        return Err(Box::new(error));
    }

    if let Err(error) = create_venv(get_python_cmd()){
        return Err(error);
    }

    if let Err(error) = create_django_project(args.name) {
        return  Err(error);
    }

    Ok(())
}


fn create_venv(python_cmd: String)-> Result<(), Box<dyn std::error::Error>> {
    println!("Creating virtual env...");

    let venv_creation_output = Command::new(python_cmd)
    .arg("-m")
    .arg("venv")
    .arg("venv")
    .output();

    if  let Err(error) =  venv_creation_output {
        return Err(Box::new(error));
    }
    
    Ok(())
}


fn get_python_cmd() -> String {
    let python_version_output = Command::new("python")
    .arg("--version")
    .output();


    let python_cmd = match python_version_output {
        Ok(_) => {
            String::from("python")
        }
        Err(_) => {
            String::from("python3")
        }
    };

    python_cmd
}

fn create_django_project(name:String)-> Result<(), Box<dyn std::error::Error>>{
    
    println!("Creating project...");

    let python_cmd = format!(
        r#"{} && pip install django && django-admin startproject {} ."#,
        if cfg!(windows) { "venv\\Scripts\\activate" } else { "source venv/bin/activate" },
        name
    );

    let create_project_result = Command::new("cmd")
        .args(&["/C", &python_cmd])
        .output();


    if let Err(error) = create_project_result {
        return Err(Box::new(error));
    }

    Ok(())
}