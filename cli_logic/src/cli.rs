use clap::{Parser, Args,Subcommand};

#[derive(Subcommand, Debug)]
#[clap(rename_all = "PascalCase")]
pub enum Resource {
    /// Generate ViewSet App
    ViewSet(ApiType),
    /// Generate ApiView App
    ApiView(ApiType),
    /// Generate new project
    Project(ProjectAction)
}

/// Cli to create APIs quickly in django
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Resource type to generate
    #[command(subcommand)]
    pub resource: Resource,

}

#[derive(Args,Debug)]
pub struct ApiType {

    /// Name of the api app
    #[arg(short, long)]
    pub name: String,

    /// Path to output the app
    #[arg(short, long, default_value = "./")]
    pub path: std::path::PathBuf,
}

#[derive(Args,Debug)]
pub struct ProjectAction {

    /// Name of the Project
    #[arg(short, long)]
    pub name: String,
    
    /// Path to output the project
    #[arg(short, long, default_value = "./")]
    pub path: std::path::PathBuf,
    
    /// Install Django REST framework
    #[arg(short, long,  default_value="false")]
    pub rest_framework: bool,

}

