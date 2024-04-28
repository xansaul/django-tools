use std::{path::PathBuf, process::Command};
use cli_logic::{cli::ProjectAction, utils::create_folder};


pub fn create_project(args: &ProjectAction)-> Result<&'static str, Box<dyn std::error::Error>> {

    let complete_path_project = args.path.join(&args.name);

    if let Err(error) = create_folder(&args.path, &args.name) {
        return Err(error);
    }

    if let Err(error) = get_python_cmd() {
        return  Err(error);
    }

    if let Err(error) = create_venv(get_python_cmd().unwrap(), &complete_path_project){
        return Err(error);
    }

    if let Err(error) = create_django_project(&complete_path_project, &args.name) {
        return  Err(error);
    }

    Ok("Project created.")
}


fn create_venv(python_cmd: String, path: &PathBuf)-> Result<(), Box<dyn std::error::Error>> {
    println!("Creating virtual env...");

    let venv_creation_output = Command::new(python_cmd)
    .arg("-m")
    .arg("venv")
    .arg(path.join("venv"))
    .output();

    if  let Err(error) =  venv_creation_output {
        return Err(Box::new(error));
    }
    
    Ok(())
}


fn get_python_cmd() -> Result<String, Box<dyn std::error::Error>> {
    // Intenta ejecutar 'python --version'
    let python_version_output = Command::new("python")
        .arg("--version")
        .output();

    // Verifica si 'python --version' fue exitoso
    if let Ok(output) = python_version_output {
        if output.status.success() {
            return Ok("python".to_string());
        }
    }

    // Intenta ejecutar 'python3 --version' si 'python --version' falla
    let python3_version_output = Command::new("python3")
        .arg("--version")
        .output();

    // Verifica si 'python3 --version' fue exitoso
    if let Ok(output) = python3_version_output {
        if output.status.success() {
            return Ok("python3".to_string());
        }
    }

    Err("No se pudo encontrar Python".into())
}

fn create_django_project(complete_path_project: &PathBuf, project_name: &String)-> Result<(), Box<dyn std::error::Error>>{
    
    println!("Creating project...");
    let path_unix= complete_path_project.strip_prefix("./").unwrap();

    let path_venv_win = format!("{}\\venv\\Scripts\\activate", complete_path_project.display());
    let path_venv_unix = format!(". {}/venv/bin/activate", path_unix.to_str().unwrap());

    let python_cmd: String = format!(
        r#"{} && pip install django && django-admin startproject {} {}"#,
        if cfg!(windows) { path_venv_win.replace("/", "\\") } else { path_venv_unix },
        project_name,
        complete_path_project.to_str().unwrap()
    );

    let create_project_result = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", &python_cmd])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(python_cmd)
            .output()
    };

    if let Err(error) = create_project_result {
        return Err(Box::new(error));
    }

    Ok(())
}