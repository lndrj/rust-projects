use clap::Parser;

/// Command line arguments structure
#[derive(Parser)]
pub struct Args {
    /// URL to fetch
    #[arg(short, long)]
    pub url: String,

    /// Method to use (GET, POST, PUT, PATCH, DELETE)
    #[arg(long, default_value = "GET")]
    pub method: String,

    /// Data to send with the request
    #[arg(short, long, default_value = "")]
    pub data: String,

    #[arg(long)]
    pub header: Vec<String>,
}
