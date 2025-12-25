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

pub fn parse_args() -> Args {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Encrypt => {
            println!("encrypt mode");
            Args::new(cli.secret, None)
        }
        Mode::Decrypt => {
            assert_eq!(cli.nonce.len(), 12, "invalid nonce length");
            println!("decrypt mode");
            Args::new(cli.secret, Some(cli.nonce[0..12].try_into().unwrap()))
        }
    }
}

pub struct Args {
    pub secret: String,
    pub nonce: Option<[u8; 12]>,
}

impl Args {
    pub fn new(secret: String, nonce: Option<[u8; 12]>) -> Self {
        Self { secret, nonce }
    }
}
