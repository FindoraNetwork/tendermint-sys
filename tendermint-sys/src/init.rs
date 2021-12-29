use crate::{
    raw::{init_config, ByteBufferReturn},
    Error, NodeEnum, Result,
};
use std::{fs, path::Path};

fn new_config(path: &str, node_type: NodeEnum) -> Result<()> {
    let mut config_str = String::from(path);
    let config_bytes = ByteBufferReturn {
        len: config_str.len(),
        data: config_str.as_mut_ptr(),
    };

    let code = unsafe { init_config(config_bytes, node_type.code()) };
    if code == 0 {
        Ok(())
    } else {
        Err(Error::from_new_config_error(code))
    }
}

pub fn init_home(path: &str, node_type: NodeEnum) -> Result<()> {
    let home_path = Path::new(path);
    let config_dir_path = home_path.join("config");
    let data_dir_path = home_path.join("data");

    fs::create_dir_all(data_dir_path)?;
    fs::create_dir_all(config_dir_path.clone())?;

    // let config_file_path = config_dir_path.join("config.toml");
    new_config(home_path.to_str().unwrap(), node_type)?;
    Ok(())
}
