use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::Path;
use ssh2::Session;
mod args;
mod config;
mod titan;
static mut DISABLE:Option<String>=None;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let option = args::Option::new();
    unsafe { DISABLE = Some(option.disable.clone()) };

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

    println!("认证成功!");
    
    // 打开 SFTP 会话
    let mut sftp = sess.sftp()?;
    
    if !option.config.is_empty() {
        config::Config::init(sftp);
        return Ok(());
    }

    let local_path = titan::fmt_path(&option.local_path)?;

    upload(&mut sess, &mut sftp, &local_path, &option.remote_dir)?;

    let remote_dir = titan::fmt_server_path(sftp, &option.remote_dir)?;

    let cdc = format!("cd {}", remote_dir.to_str().unwrap());
    let commands: Vec<&str> = vec![&cdc,option.command.as_str()];

    let mut channel = sess.channel_session().unwrap();
    channel.shell().unwrap();
    println!("------Commads------");
    for i in &commands {
        println!("{}", i);
        channel.write(format!("{}\n", i).as_bytes()).unwrap();
    }
    channel.send_eof().unwrap();

    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    channel.wait_close().unwrap();
    Ok(())
}

fn upload(session: &mut Session, sftp: &mut ssh2::Sftp, local_dir: &Path, remote_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    if local_dir.is_file(){
        let remote_path = Path::new(remote_dir);
    
        let mut file = File::open(&local_dir)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        println!("upload file: {} -> {}",local_dir.display(),remote_path.display());
        match sftp.stat(remote_path) {
            Ok(attr) => {
                if attr.is_dir() {
                    let remote_path_str = format!("{}/{}", remote_dir, local_dir.file_name().unwrap().to_str().unwrap());
                    let remote_path = Path::new(&remote_path_str);
                    let mut remote_file = sftp.create(&remote_path)?;
                    remote_file.write_all(&buffer)?;
                    println!("{} 上传成功！",local_dir.display());
                    return Ok(());
                }
                if attr.is_file() {
                    let mut remote_file = sftp.create(&remote_path)?;
                    remote_file.write_all(&buffer)?;
                    println!("{} 上传成功！",local_dir.display());
                    return Ok(());
                }
            },
            Err(e) => {
                if remote_path.file_name().is_none(){
                    let remote_path_str = format!("{}/{}", remote_dir, local_dir.file_name().unwrap().to_str().unwrap());
                    let remote_path = Path::new(&remote_path_str);
                    let mut remote_file = sftp.create(&remote_path)?;
                    remote_file.write_all(&buffer)?;
                    println!("{} 上传成功！",local_dir.display());
                    return Ok(());
                }else{
                    let mut remote_file = sftp.create(&remote_path)?;
                    remote_file.write_all(&buffer)?;
                    println!("{} 上传成功！",local_dir.display());
                    return Ok(());
                }
            }
        };

        let mut remote_file = if remote_path.is_file(){
            sftp.create(&remote_path)?
        }else{
            let remote_path_str = format!("{}/{}", remote_dir, local_dir.file_name().unwrap().to_str().unwrap());
            let remote_path = Path::new(&remote_path_str);
            sftp.create(&remote_path)?
        };
        remote_file.write_all(&buffer)?;
        println!("{} 上传成功！",local_dir.display());
        return Ok(());
    }
    
    // 创建远程文件夹
    let remote_dir_path = Path::new(remote_dir);
    if sftp.opendir(remote_dir_path).is_err() {
        println!("create remote dir : {}", remote_dir_path.display());
        sftp.mkdir(remote_dir_path, 0o755)?;
    }else{
        println!("remote dir : {} exist", remote_dir_path.display());
    }
    // 遍历本地文件夹中的所有文件和子文件夹
    for entry in fs::read_dir(local_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let remote_path_str = format!("{}/{}", remote_dir, file_name);
        let remote_path = Path::new(&remote_path_str);
        if path.is_dir() {
            // 如果是子文件夹，则递归上传
            println!("uploading dir: {} -> {}",path.display(),remote_path.display());
            upload(session, sftp, &path, &remote_path_str)?;
        } else {
            // 如果是文件，则上传文件内容
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            println!("uploading file: {} -> {}",path.display(),remote_path.display());

            let mut remote_file = sftp.create(&remote_path)?;
            remote_file.write_all(&buffer)?;
        }
    }

    Ok(())
}

#[test]
fn shell()-> Result<(), Box<dyn std::error::Error>> {


    // 建立 SSH 会话
    let tcp = std::net::TcpStream::connect("pingfanh.top:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // 使用用户名和密码进行认证
    sess.userauth_password("root","Ljq071023251")?;

    // 检查认证结果
    if !sess.authenticated() {
        println!("认证失败！");
        return Ok(());
    }

    println!("认证成功!");

    // 打开 SFTP 会话
    let sftp = sess.sftp()?;

    let remote_dir = titan::fmt_server_path(sftp, "/home/ubuntu/sc")?;

    let cdc = format!("cd {}", remote_dir.to_str().unwrap());
    let commands: Vec<&str> = vec![&cdc,];

    let mut channel = sess.channel_session().unwrap();
    channel.shell().unwrap();
    println!("------Commads------");
    for i in &commands {
        println!("{}", i);
        channel.write(format!("{}\n", i).as_bytes()).unwrap();
    }
    channel.send_eof().unwrap();

    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    channel.wait_close().unwrap();
    Ok(())
}