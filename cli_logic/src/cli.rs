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
#[command(
    before_help ="\n",
    after_help="\nㅤ",
    name="django-tools", 
    version = "0.0.2", 
    about, 
    long_about = None
)]
pub struct CliArgs {
    /// Resource type to generate
    #[command(subcommand)]
    pub resource: Resource,

}

#[derive(Args,Debug)]
#[command(
    before_help ="\n",
    after_help="\nㅤ",
)]
pub struct ApiType {

    /// Name of the api app
    #[arg(short, long)]
    pub name: String,

    /// Path to output the app
    #[arg(short, long, default_value = "./")]
    pub path: std::path::PathBuf,
}

#[derive(Args,Debug)]
#[command(
    before_help ="\n",
    after_help="\nㅤ",
)]
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

