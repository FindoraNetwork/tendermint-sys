use crate::{Error, Result, raw::init_config};
use ffi_support::ByteBuffer;
use std::{fs, path::Path};

fn new_config(path: &str) -> Result<()> {
    let config_str = String::from(path);
    let config_bytes = ByteBuffer::from_vec(config_str.into_bytes());

    let code = unsafe { init_config(config_bytes) };
    if code == 0 {
        Ok(())
    } else {
        Err(Error::from_new_config_error(code))
    }
}

pub fn init_home(path: &str) -> Result<()> {
    let home_path = Path::new(path);
    let config_dir_path = home_path.join("config");
    let data_dir_path = home_path.join("data");
    
    fs::create_dir_all(data_dir_path)?;
    fs::create_dir_all(config_dir_path.clone())?;

    let config_file_path = config_dir_path.join("config.toml");
    new_config(config_file_path.to_str().unwrap())?;
    Ok(())
}

