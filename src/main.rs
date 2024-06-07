use clap::{arg, Command};
use core::fmt;
use std::{fs, path::Path};
use termion::color;
use termion::style;

#[derive(Debug)]
struct TxOpenConfig {
    tx_hash: String,
    chain: Chain,
}
#[derive(Debug)]
struct AddressOpenConfig {
    address: String,
    chain: Chain,
}
#[derive(Debug)]
struct BlockOpenConfig {
    block: String,
    chain: Chain,
}
#[derive(Debug)]
struct ShowHintsConfig {
    editor: String,
}
#[derive(Debug)]
struct SearchHintsConfig {
    pattern: String,
    clean: bool,
}
#[derive(Debug)]
enum Chain {
    Coston,
    Coston2,
    Flare,
    Songbird,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chain::Coston => write!(f, "https://coston-explorer.flare.network"),
            Chain::Coston2 => write!(f, "https://coston2-explorer.flare.network"),
            Chain::Flare => write!(f, "https://flare-explorer.flare.network"),
            Chain::Songbird => write!(f, "https://songbird-explorer.flare.network"),
        }
    }
}

/**
TODO: add address hints
TODO: add address hint search
TODO: add open explorer for all addresses that match a pattern of a hint
*/
fn main() {
    init_project_dir();
    let matches = Command::new("blsct")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Wrapper for cli for Bloscout")
        .arg(
            arg!(--chain <CHAIN>)
                .required(true)
                .value_parser(["coston", "coston2", "flare", "songbird"]),
        )
        .subcommand(
            Command::new("tx")
                .about("Open a transaction on web")
                .arg(arg!(--tx <TXHASH>).required(true)),
        )
        .subcommand(
            Command::new("address")
                .about("Open an address on web")
                .arg(arg!(--address <ADDRESS>).required(true)),
        )
        .subcommand(
            Command::new("block")
                .about("Open a block on web")
                .arg(arg!(--block <BLOCK>).required(true)),
        )
        .subcommand(
            Command::new("hint")
                .subcommand(Command::new("search").about(
                    "Search for address hints. Should return a list of addresses matching a pattern.",
                ).arg(arg!(--pattern <PATTERN>).required(true))
                 .arg(arg!(--clean).required(false)))
                .subcommand(Command::new("show").about("Opens a file with address hints.").arg(
                    arg!(--editor <EDITOR>)
                        .required(false)
                        .default_value("nano"),
                )),
        )
        .get_matches();

    let chain = match matches.get_one::<String>("chain").map(|s| s.as_str()) {
        Some("coston") => Chain::Coston,
        Some("coston2") => Chain::Coston2,
        Some("flare") => Chain::Flare,
        Some("songbird") => Chain::Songbird,
        _ => {
            println!("Invalid chain provided.");
            return;
        }
    };

    match matches.subcommand() {
        Some(("tx", sub_matches)) => {
            let tx_hash = sub_matches.get_one::<String>("tx").unwrap().clone();
            let config = TxOpenConfig { tx_hash, chain };
            run_open_tx(config);
        }
        Some(("address", sub_matches)) => {
            let address = sub_matches.get_one::<String>("address").unwrap().clone();
            let config = AddressOpenConfig { address, chain };
            run_open_address(config);
        }
        Some(("block", sub_matches)) => {
            let block = sub_matches.get_one::<String>("block").unwrap().clone();
            let config = BlockOpenConfig { block, chain };
            run_open_block(config);
        }
        Some(("hint", sub_matches)) => match sub_matches.subcommand() {
            Some(("search", sub_matches)) => {
                let pattern = sub_matches.get_one::<String>("pattern").unwrap().clone();
                let clean = sub_matches.get_one::<bool>("clean").unwrap();
                let config = SearchHintsConfig {
                    pattern,
                    clean: clean.clone(),
                };
                run_search_addresses(config);
            }
            Some(("show", sub_matches)) => {
                let editor = sub_matches.get_one::<String>("editor").unwrap().clone();
                let config = ShowHintsConfig { editor };
                run_show_addresses(config);
            }
            _ => {
                println!("No valid subcommand was provided.");
                println!("Use --help for more information.");
            }
        },
        _ => {
            println!("No valid subcommand was provided.");
            println!("Use --help for more information.");
        }
    }
}

fn run_open_tx(config: TxOpenConfig) {
    println!("Opening transaction with hash: {}", config.tx_hash);
    let url = format!("{}/tx/{}", config.chain, config.tx_hash);
    // open browser window at url
    match webbrowser::open(&url) {
        Ok(_) => println!("Successfully opened the URL."),
        Err(e) => eprintln!("Failed to open the URL: {}", e),
    }
}

fn run_open_address(config: AddressOpenConfig) {
    println!("Opening address with hash: {}", config.address);
    let url = format!("{}/address/{}", config.chain, config.address);
    // open browser window at url
    match webbrowser::open(&url) {
        Ok(_) => println!("Successfully opened the URL."),
        Err(e) => eprintln!("Failed to open the URL: {}", e),
    }
}

fn run_open_block(config: BlockOpenConfig) {
    println!("Opening block with hash: {}", config.block);
    let url = format!("{}/block/{}", config.chain, config.block);
    // open browser window at url
    match webbrowser::open(&url) {
        Ok(_) => println!("Successfully opened the URL."),
        Err(e) => eprintln!("Failed to open the URL: {}", e),
    }
}

fn run_show_addresses(config: ShowHintsConfig) {
    // open address hints file
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let project_dir = home_dir.join(".blsct");
    let address_hints_file = project_dir.join("address_hints.json");

    std::process::Command::new(config.editor)
        .arg(address_hints_file)
        .spawn()
        .expect("Failed to open address hints file");
}

fn run_search_addresses(config: SearchHintsConfig) {
    // search for address hints
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let project_dir = home_dir.join(".blsct");
    let address_hints_file = project_dir.join("address_hints.json");

    let hints = fs::read_to_string(&address_hints_file).expect("Failed to read address hints file");
    let hints: serde_json::Value = serde_json::from_str(&hints).expect("Failed to parse hints");

    let pattern = config.pattern;

    let matching_addresses = hints
        .as_object()
        .expect("Hints should be an object")
        .iter()
        .filter_map(|(key, value)| {
            if key.to_lowercase().contains(&pattern) {
                Some((key.clone(), value.as_str().unwrap_or_default().to_string()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if config.clean {
        for (_, address) in &matching_addresses {
            println!("{}", address);
        }
    } else {
        for (name, address) in &matching_addresses {
            println!("{}{}{}:", style::Bold, color::Fg(color::Blue), name);
            print!("{}", style::Reset);
            println!("{}", address);
        }
    }
}

fn init_project_dir() {
    // create a dir in ~/.blsct
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let project_dir = home_dir.join(".blsct");

    if !Path::new(&project_dir).exists() {
        fs::create_dir_all(&project_dir).expect("Failed to create project directory");
    }

    let address_hints_file = project_dir.join("address_hints.json");

    if !Path::new(&address_hints_file).exists() {
        fs::write(&address_hints_file, "{}").expect("Failed to create address hints file");
    }
}
