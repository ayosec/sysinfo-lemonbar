use std::env::args;
use gumdrop::Options;

#[derive(Debug, Default, Options)]
pub struct AppOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "screen where data is shown")]
    screen: Option<usize>,

    #[options(help = "interval to compute updates, in seconds", meta = "SECS")]
    update_interval: Option<usize>,

    // A `Vec` field will accumulate all values received from the command line.
    #[options(help = "mountpoint to show disk space usage", meta = "PATH")]
    mountpoints: Vec<String>,
}

pub fn parse() -> Option<AppOptions> {
    let args: Vec<String> = args().collect();

    let opts = match AppOptions::parse_args_default(&args[1..]) {
        Ok(opts) => opts,
        Err(e) => {
            println!("{}: {}", args[0], e);
            return None;
        }
    };

    if opts.help {
        println!("Usage: info-bar-x11 [OPTIONS]\n\n{}", AppOptions::usage());
        return None;
    }

    Some(opts)
}
