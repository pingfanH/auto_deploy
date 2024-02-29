

pub struct Config{
    sftp:ssh2::Sftp,
}
impl Config {
    pub fn init(sftp:ssh2::Sftp)->Config{
        let config = Config{
            sftp,
        };
        config
    }
}