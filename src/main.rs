mod coord;

use anyhow::{anyhow, Context, Result};
use cgdmat::Cipher;
use clap::{CommandFactory, ErrorKind, Parser};
use config::Config;
use coord::Coord;
use rpassword;
use std::str;

const MAX_ROWS: u32 = 8;
const MAX_COLS: u32 = 8;
const MAX_POS: u32 = 3;
const CONFIG_FILE: &str = "cgdmat.toml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(parse(try_from_str = Coord::from_str))]
    coordinates: Vec<Coord>,

    #[clap(long)]
    encode: Option<String>,
}

fn decode(coord: &Coord, password: &str, matrix: &str) -> Result<char> {
    let cipher = Cipher::new(password);
    let plaintext = cipher.decrypt(matrix);

    let chars: Vec<char> = str::from_utf8(plaintext.as_ref())
        .unwrap()
        .chars()
        .collect();
    let pos = coord.row * (MAX_COLS * MAX_POS) + coord.col * MAX_POS + coord.pos;
    return if pos < MAX_COLS * MAX_ROWS * MAX_POS {
        Ok(chars[pos as usize])
    } else {
        Err(anyhow!("Coordinate outside matrix"))
    };
}

fn enc(password: &str, message: &str) {
    let cipher = Cipher::new(password);
    let ciphertext = cipher.encrypt(message);
    println!("chiphertext: {}", ciphertext);
    let plaintext = cipher.decrypt(ciphertext.as_ref());
    assert_eq!(&plaintext, message);
}

fn get_config() -> Result<Config> {
    let xdg_dirs = xdg::BaseDirectories::new()?;
    let config_file = xdg_dirs
        .find_config_file(CONFIG_FILE)
        .with_context(|| format!("Can't find config file {}", CONFIG_FILE))?;
    Config::builder()
        .add_source(config::File::with_name(config_file.to_str().context("")?))
        .build()
        .context("Error retrieving config")
}

fn main() -> Result<()> {
    let config = get_config()?;
    let matrix = config.get_string("matrix")?;
    let cli = Cli::parse();

    if let Some(encode) = cli.encode.as_deref() {
        if cli.coordinates.len() > 0 {
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can't encode and decode in the same run",
            )
            .exit();
        }
        let password = rpassword::prompt_password("Password: ")?;
        enc(&password, encode);
    } else {
        if cli.coordinates.len() == 0 {
            let mut cmd = Cli::command();
            cmd.error(ErrorKind::ArgumentConflict, "No coordinates")
                .exit();
        }
        let password = rpassword::prompt_password("Password: ")?;
        let decoded = cli
            .coordinates
            .iter()
            .map(|c| decode(c, &password, &matrix).context("No value found for coordinate"))
            .collect::<Result<Vec<char>>>()?;
        decoded.iter().for_each(|d| println!("{}", d));
    };
    Ok(())
}
