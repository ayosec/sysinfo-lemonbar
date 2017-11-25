use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Read};

#[derive(Debug)]
pub struct CoreTemp {
    pub input: f64,
    pub label: String,
}

pub fn load_core_temp() -> Result<Vec<CoreTemp>, Box<Error>> {
    Ok(
        fs::read_dir("/sys/class/hwmon")?
            .filter_map(|p| p.ok())
            .filter(|monitor| is_coretemp(monitor.path()))
            .flat_map(|monitor| {
                (1..)
                    .map(move |n| load_temp(monitor.path(), n))
                    .take_while(|t| t.is_ok())
                    .filter_map(|t| t.ok())
            })
            .collect(),
    )
}

fn is_coretemp(mut path: PathBuf) -> bool {
    path.push("name");
    match read_file(path) {
        Ok(b) => b.trim() == "coretemp",
        Err(_) => false,
    }
}

fn read_file<T: AsRef<Path>>(path: T) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut body = String::new();
    file.read_to_string(&mut body)?;
    Ok(body)
}

fn load_temp<T: AsRef<Path>>(path: T, num: usize) -> Result<CoreTemp, Box<Error>> {
    let input = path.as_ref().join(format!("temp{}_input", num));
    let label = path.as_ref().join(format!("temp{}_label", num));
    Ok(CoreTemp {
        input: read_file(input)?.trim().parse::<f64>()? / 1000.0,
        label: read_file(label)?.trim().into(),
    })
}
