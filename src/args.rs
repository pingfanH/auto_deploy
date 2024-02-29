
#[derive(Debug,Default)]
pub struct Option{
    pub host: String,
    pub username: String,
    pub password: String,
    pub local_path: String,
    pub remote_dir: String,
}
impl Option {
    pub fn new() -> Option {
        let args = std::env::args().collect::<Vec<String>>();
        println!("{:?}",args);
        let mut option = Option{
            ..Default::default()
        };
        for i in 0..args.len() {
                if args[i]=="--help"{
                   println!(r#"
                   usage : ./auto_deploy.exe -h ip:22 -u root -p passwd -l project -r /home/ubuntu/project
                   "#);
                    std::process::exit(1);
                }
                if args[i]=="-u"{
                    option.username = args[i+1].clone();
                    continue;
                }
                if args[i]=="-p"{
                    option.password = args[i+1].clone();
                    continue;
                }
                if args[i]=="-h"{
                    option.host = args[i+1].clone();
                    continue;
                }
                if args[i]=="-l"{
                    option.local_path = args[i+1].clone();
                    continue;
                }
                if args[i]=="-r"{
                    option.remote_dir = args[i+1].clone();
                    continue;
                }
        }
        if option.host.is_empty(){
            println!("host is empty!");
            std::process::exit(1);
        }
        if option.username.is_empty(){
            println!("username is empty!");
            std::process::exit(1);
        }
        if option.password.is_empty(){
            println!("password is empty!");
            std::process::exit(1);
        }
        if option.local_path.is_empty(){
            println!("local_dir is empty!");
            std::process::exit(1);
        }
        if option.remote_dir.is_empty(){
            println!("remote_dir is empty!");
            std::process::exit(1);
        }
        println!("{:?}",option);
        option
    }  
}