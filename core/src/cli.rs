use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "PascalCase")]
pub enum Resource {
    SetView,
    ApiView
}

/// cli to create apis quickly in django
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Resource type to generate
    #[arg(value_enum)]
    pub resource: Resource,

    /// Name of the api app
    #[arg(short, long)]
    pub name: String,

    /// Path to output the app
    #[arg(short, long, default_value = "./")]
    pub path: std::path::PathBuf,
}


