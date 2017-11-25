use info;
use hwmon;
use filesystem;
use options::AppOptions;
use std::error::Error;
use std::fmt::Write;
use std::thread::sleep;
use std::time::Duration;
use std::fmt;

const SEP: &str = "     |     ";

struct Line<'a> {
    options: &'a AppOptions,
    buffer_line: String,
    code_warn: String,
    code_alert: String,
}

impl<'a> Line<'a> {
    fn from_options(options: &'a AppOptions) -> Line<'a> {
        Line {
            options,
            buffer_line: String::new(),
            code_warn: parse_color(&options.color_warn),
            code_alert: parse_color(&options.color_alert),
        }
    }

    fn reset(&mut self) -> Result<(), fmt::Error> {
        self.buffer_line.clear();
        if let Some(scr) = self.options.screen {
            write!(&mut self.buffer_line, "%{{S{}}}", scr)?;
        }
        write!(&mut self.buffer_line, "%{{{}}}", self.options.align.code())?;
        self.buffer_line.push_str(SEP);
        Ok(())
    }

    fn print(&self) {
        println!("{}   ", self.buffer_line);
    }

    fn append<S: AsRef<str>>(&mut self, data: S, show_warn: bool, show_alert: bool) {
        if show_alert {
            self.buffer_line.push_str(&self.code_alert);
        } else if show_warn {
            self.buffer_line.push_str(&self.code_warn);
        }

        self.buffer_line.push_str("  ");
        self.buffer_line.push_str(data.as_ref());
        self.buffer_line.push_str("  ");

        if show_warn || show_alert {
            self.buffer_line.push_str("%{F-}%{B-}")
        }
        self.buffer_line.push_str(SEP);
    }
}

pub fn run(options: AppOptions) -> Result<(), Box<Error>> {

    let mut line = Line::from_options(&options);
    let interval = Duration::from_secs(options.update_interval);

    loop {

        line.reset()?;

        // Memory

        let sysinfo = info::load();
        let memfree = to_mib(sysinfo.memory_free);

        line.append(
            format!("RAM Free: {} M", memfree as u64),
            memfree < options.memory_free_warn_threshold,
            memfree < options.memory_free_alert_threshold,
        );

        // Load average

        line.append(
            format!("LoadAvg: {:.2}, {:.2}", sysinfo.load_1m, sysinfo.load_5m),
            sysinfo.load_1m > options.loadavg_warn_threshold ||
                sysinfo.load_5m > options.loadavg_warn_threshold,
            sysinfo.load_1m > options.loadavg_alert_threshold ||
                sysinfo.load_5m > options.loadavg_alert_threshold,
        );

        // CPU temperature

        let coretemp = hwmon::load_core_temp()?;
        let max_temp = coretemp.iter().map(|ct| ct.input as u64).max().unwrap_or(0);

        line.append(
            format!("Core Temp.: {:.0} ºC", max_temp),
            max_temp >= options.cpu_temp_warn_threshold as u64,
            max_temp >= options.cpu_temp_alert_threshold as u64,
        );

        // Filesystem usage

        for fs in filesystem::get_info(&options.mountpoints[..]) {
            let free = 100.0 * fs.block_free as f64 / fs.block_size as f64;
            line.append(
                format!("{} = {:.0}% free", fs.path, free),
                free < options.disk_usage_warn_threshold,
                free < options.disk_usage_alert_threshold,
            );
        }

        // Print line and wait for the next update

        line.print();
        sleep(interval);

    }
}

fn to_mib(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}

// If 'color' is "#xxx", generates code to set foreground
// If 'color' is "/#xxx", generates code to set background
// If 'color' is "#xxx/#yyy", generates code to set both
fn parse_color(color: &str) -> String {
    match color.find('/') {
        None => format!("%{{F{}}}", color),
        Some(0) => format!("%{{B{}}})", &color[1..]),
        Some(o) => {
            let (fg, bg) = color.split_at(o);
            format!("%{{F{}}}%{{B{}}}", fg, &bg[1..])
        }
    }
}
