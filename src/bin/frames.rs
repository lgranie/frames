use clap::Parser;
use log::{debug, info, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::io::BufReader;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on (debug, info, warn(default))
    #[clap(short, long)]
    debug: Option<String>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let cli = Cli::parse();

    // set log level
    match cli.debug.as_deref() {
        Some("debug") => SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap(),
        Some("info") => SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap(),
        _ => SimpleLogger::new()
            .with_level(LevelFilter::Warn)
            .init()
            .unwrap(),
    }

    // load config file
    let config_path: PathBuf;
    match cli.config.as_deref() {
        Some(cp) => config_path = cp.to_path_buf(),
        _ => {
            let xdg_dirs = BaseDirectories::with_prefix("frames")
                .expect("cannot create config directory");
            config_path = xdg_dirs
                .place_config_file("server.yml")
                .expect("cannot create config file");   
            info!("using default {}", config_path.display());
        }
    };
    let config_file = std::fs::File::open(&config_path).unwrap();
    let config_reader = BufReader::new(config_file);
    let config: frames::config::Config = 
        serde_yaml::from_reader(config_reader).expect("unable to load config");
    debug!("config is {:?}", config);

    // load downloaders
    let mut transmission : frames::downloader::Transmission = 
        frames::downloader::Transmission::new(
            "transmission".to_string(),
            config.downloaders.get("transmission").unwrap()
        );
    info!("downloader {}", transmission.name);
    info!("downloader {}", transmission.is_alive().await);  
}
