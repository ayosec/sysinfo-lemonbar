use std::env::args;
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct AppOptions {
    #[options(help = "print help message")]
    pub help: bool,

    #[options(help = "screen where data is shown")]
    pub screen: Option<usize>,

    #[options(help = "interval to compute updates, in seconds", meta = "SECS")]
    pub update_interval: u64,

    #[options(help = "mountpoint to show disk space usage", meta = "PATH")]
    pub mountpoints: Vec<String>,

    #[options(help = "threshold (in %) to show the disk usage in WARN state", meta = "VALUE")]
    pub disk_usage_warn_threshold: f64,

    #[options(help = "threshold (in %) to show the disk usage in ALERT state", meta = "VALUE")]
    pub disk_usage_alert_threshold: f64,

    #[options(help = "threshold (in ºC) to show the CPU temperature in WARN state",
              meta = "VALUE")]
    pub cpu_temp_warn_threshold: f64,

    #[options(help = "threshold (in ºC) to show the CPU temperature in ALERT state",
              meta = "VALUE")]
    pub cpu_temp_alert_threshold: f64,

    #[options(help = "threshold (in Mib) to show the free memory in WARN state", meta = "VALUE")]
    pub memory_free_warn_threshold: f64,

    #[options(help = "threshold (in Mib) to show the free memory in ALERT state", meta = "VALUE")]
    pub memory_free_alert_threshold: f64,

    #[options(help = "threshold to show the load average in WARN state", meta = "VALUE")]
    pub loadavg_warn_threshold: f64,

    #[options(help = "threshold to show the load average in ALERT state", meta = "VALUE")]
    pub loadavg_alert_threshold: f64,

    #[options(help = "color for the text in WARN state", meta = "FG/BG")]
    pub color_warn: String,

    #[options(help = "color for the text in ALERT state", meta = "FG/BG")]
    pub color_alert: String,
}

impl Default for AppOptions {
    fn default() -> AppOptions {
        AppOptions {
            help: false,
            screen: None,
            update_interval: 5,
            mountpoints: vec![],
            disk_usage_warn_threshold: 10.0,
            disk_usage_alert_threshold: 5.0,
            cpu_temp_warn_threshold: 60.0,
            cpu_temp_alert_threshold: 70.0,
            memory_free_warn_threshold: 512.0,
            memory_free_alert_threshold: 128.0,
            loadavg_warn_threshold: 1.0,
            loadavg_alert_threshold: 2.0,
            color_warn: String::from("#000/#fa7"),
            color_alert: String::from("#fff/#700"),
        }
    }
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
