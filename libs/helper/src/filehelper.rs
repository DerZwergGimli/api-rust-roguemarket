use serde::de::DeserializeOwned;

use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn read_file<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let mut file = File::open(path).unwrap();
    let mut contents = std::string::String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = serde_json::from_str::<T>(contents.clone().as_str());

    return data.unwrap();
}

pub fn write_file<T: serde::Serialize>(path: &str, data: &T) {
    let mut file = File::create(path).unwrap();
    let _ = write!(file, "{}", serde_json::to_string_pretty(data).unwrap());
}
