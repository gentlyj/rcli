mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};
pub use process::{process_base64_decode, process_base64_encode, process_csv, process_genpass};
