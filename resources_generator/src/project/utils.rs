use std::process::Command;

pub fn exect_commands(python_cmd: String)-> Result<(), Box<dyn std::error::Error>>{

    let exect_command = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", &python_cmd])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(python_cmd)
            .output()
    };

    if let Err(error) = exect_command {
        return Err(Box::new(error));
    }

    Ok(())
}


pub fn get_python_command() -> Result<String, Box<dyn std::error::Error>> {
    
    let python_version_output = Command::new("python")
        .arg("--version")
        .output();

    
    if let Ok(output) = python_version_output {
        if output.status.success() {
            return Ok("python".to_string());
        }
    }

    
    let python3_version_output = Command::new("python3")
        .arg("--version")
        .output();

    if let Ok(output) = python3_version_output {
        if output.status.success() {
            return Ok("python3".to_string());
        }
    }

    Err("Python is not installed.".into())
}