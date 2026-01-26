use ftp_client::{FtpClient, basic::{client::TcpFtpClient, command::FtpBaseCommands}};

pub mod log;

pub fn main() -> ftp_client::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let hostname = args.get(1).map_or("ftp.ubuntu.com", |s| s.as_str());
    let mut client = TcpFtpClient::new(hostname, None)?;
    println!("Connecting to {hostname}");
    
    let res = client.read_response();

    let Ok(ok_res) = res else {
        let err = res.unwrap_err();
        println!("Couldn't establish connection: {err}");
        return Err(err);
    };
    
    println!("{ok_res}");

    log::send_command(&FtpBaseCommands::User { username: "anonymous".to_string() }, &mut client);
    log::send_command(&FtpBaseCommands::Pass { password: "pswd".to_string() }, &mut client);
    log::send_command(&FtpBaseCommands::Pwd, &mut client);
    
    Ok(())
}