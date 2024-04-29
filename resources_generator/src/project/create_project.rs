use std::{fs, path::PathBuf, process::Command};
use cli_logic::{cli::ProjectAction, utils::create_folder};
use super::utils::{exect_commands, get_python_command};

pub fn create_project(args: &ProjectAction)-> Result<&'static str, Box<dyn std::error::Error>> {

    let complete_path_project = args.path.join(&args.name);

    if let Err(error) = create_folder(&args.path, &args.name) {
        return Err(error);
    }

    if let Err(error) = get_python_command() {
        return  Err(error);
    }

    if let Err(error) = create_venv(get_python_command().unwrap(), &complete_path_project){
        return Err(error);
    }

    if let Err(error) = create_django_project(&complete_path_project, &args.name) {
        return  Err(error);
    }


    if args.rest_framework {

        if let Err(error) = add_rest_framework(&complete_path_project, &args.name) {
            return Err(error);
        }
        
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


fn create_django_project(complete_path_project: &PathBuf, project_name: &String)-> Result<(), Box<dyn std::error::Error>>{
    
    println!("Creating project...");

    let python_cmd = build_python_command_for_create_project(&complete_path_project, &project_name);

    if let Err(error) = exect_commands(python_cmd) {
        return Err(error);
    }

    Ok(())
}


fn build_command(complete_path_project: &PathBuf, extra_command: String) -> String{

    let venv_activate = get_command_to_activate_venv(&complete_path_project);

    let command = format!(
        r#"{} && {}"#,
        venv_activate,
        extra_command
    );

    command
}

fn build_command_install_rest_framework(complete_path_project: &PathBuf)->String {

    let command = format!(
        r#"pip install djangorestframework"#,
    );

    build_command(complete_path_project, command)
}


fn build_python_command_for_create_project(complete_path_project: &PathBuf, project_name: &String)->String{

    let command = format!(
        r#"pip install django && django-admin startproject {} {}"#,
        project_name,
        complete_path_project.to_str().unwrap()
    );

    build_command(complete_path_project, command)
}


fn get_command_to_activate_venv(complete_path_project: &PathBuf)-> String{

    let processed_unix_path = complete_path_project.strip_prefix("./").unwrap();
    let path_venv_unix = format!(". {}/venv/bin/activate", processed_unix_path.to_str().unwrap());

    let path_venv_win = format!("{}\\venv\\Scripts\\activate", complete_path_project.display());

    let command = if cfg!(windows) { path_venv_win.replace("/", "\\") } else { path_venv_unix };

    command
}

fn add_rest_framework(complete_path_project: &PathBuf, project_name: &String)->Result<(), Box<dyn std::error::Error>>{
    println!("Installing REST framework...");
        
    if let Err(error) = exect_commands(
        build_command_install_rest_framework(&complete_path_project)
    ) {
        return Err(error);
    }
    
    let resutl = add_rest_framework_to_installed_apps(&complete_path_project, project_name);
    
    match resutl {
        Ok(_) => println!("'rest_framework' added to INSTALLED_APPS in settings.py."),
        Err(error) => {
            println!("Add 'rest_framework' manually.");
            return Err(error);
        }
    }
    

    Ok(())
}

fn add_rest_framework_to_installed_apps(complete_path_project: &PathBuf, project_name: &String) -> Result<(), Box<dyn std::error::Error>> {

    let settings_path = complete_path_project.join(project_name).join("settings.py");
    let settings_content = fs::read_to_string(&settings_path)?;

    let installed_apps_pos = settings_content.find("INSTALLED_APPS");
    let installed_apps_pos = installed_apps_pos.ok_or("INSTALLED_APPS not found in settings.py file.")?;


    let bracket_pos = settings_content[installed_apps_pos..]
        .find(']')
        .map(|pos| installed_apps_pos + pos)
        .ok_or("Closing bracket not found for INSTALLED_APPS.")?;


    let mut modified_content = String::with_capacity(settings_content.len() + 20);
    modified_content.push_str(&settings_content[..bracket_pos]);
    modified_content.push_str("    'rest_framework',\n");
    modified_content.push_str(&settings_content[bracket_pos..]);

    fs::write(&settings_path, modified_content)?;

    Ok(())
}