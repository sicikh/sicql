pub use crate::error::Error;
use clap::Parser;

pub type Result<T> = core::result::Result<T, Error>;

/// Server for SicQL DBMS
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Port on which server is listening
    #[arg(default_value_t = 9117)]
    pub port: u16,
    // FIXME: more detailed description
    /// Size of memory for VM in bytes
    #[arg(default_value_t = 4096)]
    pub memory_size: usize,
}
