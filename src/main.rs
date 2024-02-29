use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::Path;
use ssh2::Session;
mod args;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let option = args::Option::new();
    // let option=args::Option{
    //     host:"49.232.237.42:22".to_owned(),
    //     username:"ubuntu".to_owned(),
    //     password:"Ljq071023251".to_owned(),
    //     local_path:"./Cargo.toml".to_owned(),
    //     remote_dir:"/home/ubuntu/Cargo.toml".to_owned(),
    // };
    // 建立 SSH 会话
    let tcp = std::net::TcpStream::connect(option.host)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // 使用用户名和密码进行认证
    sess.userauth_password(&option.username, &option.password)?;

    // 检查认证结果
    if !sess.authenticated() {
        println!("认证失败！");
        return Ok(());
    }

    println!("认证成功！");

    // 打开 SFTP 会话
    let mut sftp = sess.sftp()?;
    let local_path=Path::new(&option.local_path);
    upload(&mut sess, &mut sftp,local_path , &option.remote_dir)?;

    println!("文件上传成功！");

    Ok(())
}

fn upload(session: &mut Session, sftp: &mut ssh2::Sftp, local_dir: &Path, remote_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    if local_dir.is_file(){
        let remote_path = Path::new(remote_dir);
    
        let mut file = File::open(&local_dir)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let mut remote_file = sftp.create(&remote_path)?;
        remote_file.write_all(&buffer)?;
        println!("{} 上传成功！",local_dir.display());
        return Ok(());
    }
    
    // 创建远程文件夹
    let remote_dir_path = Path::new(remote_dir);
    if sftp.opendir(remote_dir_path).is_err() {
        sftp.mkdir(remote_dir_path, 0o755)?;
    }
    // 遍历本地文件夹中的所有文件和子文件夹
    for entry in fs::read_dir(local_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let remote_path_str = format!("{}/{}", remote_dir, file_name);
        let remote_path = Path::new(&remote_path_str);
        println!("{remote_path_str}");
        if path.is_dir() {
            // 如果是子文件夹，则递归上传
            upload(session, sftp, &path, &remote_path_str)?;
        } else {
            // 如果是文件，则上传文件内容
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let mut remote_file = sftp.create(&remote_path)?;
            remote_file.write_all(&buffer)?;
        }
    }

    Ok(())
}