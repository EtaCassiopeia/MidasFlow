use clap::Parser;

pub const LOGO: &str = r"
    __  ____     __
   /  |/  (_)___/ /___ ______
  / /|_/ / / __  / __ `/ ___/
 / /  / / / /_/ / /_/ (__  )
/_/  /_/_/\__,_/\__,_/____/
";

pub const DESCRIPTION: &str =
    r#"With MidasFlow, your data transformation is as simple and powerful as the Midas touch."#;
pub const DEFAULT_CONFIG_PATH_PATTERNS: &[&str] = &["./midas-config.yaml"];

#[derive(Parser, Debug)]
#[command(author, version, name = "midas")]
#[command(
about = format!("{} \n {}", LOGO, DESCRIPTION),
long_about = None,
)]
pub struct Cli {
    #[arg(
    global = true,
    short = 'c',
    long = "config-path",
    default_values = DEFAULT_CONFIG_PATH_PATTERNS
    )]
    pub config_paths: Vec<String>,
}
