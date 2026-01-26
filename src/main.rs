use ftp_client::{FtpClient, basic::{client::TcpFtpClient, command::FtpBaseCommands}, command::FtpCommand};

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

    
    FtpBaseCommands::Pwd.send(&mut client)?;
    println!("pwd command sent !");
    let res = client.read_response();

    match res {
        Ok(ok_res) => println!("{ok_res}"),
        Err(err_res) => println!("{err_res}"),
    };
    Ok(())
}