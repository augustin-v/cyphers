use clap::Parser;
#[derive(Parser)]
struct Cli {
    secret: String,
    #[arg(short, long)]
    mode: Mode,
    /// required on decrypt mode only
    #[arg(required_if_eq("mode", "Decrypt"), value_delimiter(','))]
    nonce: Vec<u8>,
}
#[derive(Clone, clap::ValueEnum)]
enum Mode {
    /// Encrypt Mode
    Encrypt,
    /// Decrypt mode
    Decrypt,
}

pub fn parse_args() {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Encrypt => {
            println!("encrypt mode")
        }
        Mode::Decrypt => {
            assert_eq!(cli.nonce.len(), 12, "invalid nonce length");
            println!("decrypt mode")
        }
    }
}
