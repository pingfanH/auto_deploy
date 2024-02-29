use std::path::{Path, PathBuf};

use ssh2::Sftp; // Import PathBuf

pub fn fmt_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> { // Change type to PathBuf
    let local_dir = std::env::current_dir()?;
    let local_path = Path::new(path);
    let local_path = if local_path.is_relative() {
        let local_path_str = format!("{}\\{}", local_dir.to_str().unwrap(), path);
        let new_local_path = Path::new(&local_path_str).to_path_buf();
        new_local_path
    } else {
        local_path.to_path_buf()
    };
    
    Ok(local_path) // Add return statement
}
pub fn fmt_server_path(sftp:ssh2::Sftp,path: &str) -> Result<&Path, Box<dyn std::error::Error>> { // Change type to PathBuf
    let remote_path = Path::new(path);
    match sftp.stat(remote_path) {
        Ok(attr) => {
            if attr.is_dir() {
               
                return Ok(remote_path);
            }
            if attr.is_file() {
               
                return Ok(remote_path.parent().unwrap());
            }
        },
        Err(e) => {
            
        }
    };
    
    Ok(remote_path) // Add return statement
}
#[test]
fn test_fmt_path() {
    let path = "auto_deploy\\ZIP.exe";
    let result = fmt_path(path).unwrap();
    println!("{:?}", result);
}