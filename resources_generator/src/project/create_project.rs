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

    let python_version_output = Command::new("python")
    .arg("--version")
    .output();


    let python_cmd = match python_version_output {
        Ok(output) => {
            if output.status.success() {
                "python"
            } else {
                "python3"
            }
        }
        Err(_) => {
            "python3"
        }
    };

    println!("Creating virtual env...");

    let venv_creation_output = Command::new(python_cmd)
    .arg("-m")
    .arg("venv")
    .arg("venv")
    .output()
    .expect("Error creating virtual environment");

    println!("Creating project...");

    if !venv_creation_output.status.success() {
        return Err("Error creating virtual environment.".into());
    }

    let python_cmd = format!(
        r#"{} && pip install django && django-admin startproject {} ."#,
        if cfg!(windows) { "venv\\Scripts\\activate" } else { "source venv/bin/activate" },
        args.name
    );

    Command::new("cmd")
        .args(&["/C", &python_cmd])
        .output()
        .expect("Error executing Python commands");


    Ok(())
}