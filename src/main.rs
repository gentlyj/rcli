use clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_genpass, Base64SubCommand,
    Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            process_genpass(&opts)?;
        }
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => process_base64_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_base64_decode(&opts.input, opts.format)?,
        },
    }
    Ok(())
}
