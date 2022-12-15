use lazy_static::lazy_static;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use structopt::StructOpt;
use config::{Config, File, FileFormat};
use log::*;

pub mod hexagonaltile;
pub mod soots;
pub mod factory;

///Small, a tactical RPG Game
#[derive(StructOpt, Debug)]
#[structopt(name = "Small RPG")]
pub struct Opt {
    /// Set the level that is loaded
    #[structopt(short, long, default_value = "treehouse")]
    pub lvl: String,
    /// Set whether board is loaded using generation or a map
    #[structopt(short, long, default_value = "image")]
    pub generate_method: String,
    /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , -vvvv or more: trace)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbosity: u8,
}

lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
    pub static ref CFG: Option<Config> = {
        let cfg = Config::builder().add_source(File::new("core/game_configuration.toml", FileFormat::Toml));

        match cfg.build() {
            Ok(config) => Some(config),
            Err(_) => {
                error!("couldn't load game configuration");
                None
                }
            }
        };

}

///Type alias for nannou color type
pub type Mrgb = Rgb<Srgb, u8>;

pub fn cfg_fetch(key: &str) -> String {
    CFG.as_ref().expect("Unable to generate configuration!")
       .get_string(key).unwrap_or_else(|_| panic!("Couldn't find requested configuration key: {}", key))
}

