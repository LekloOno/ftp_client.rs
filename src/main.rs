use ftp_client::{FtpClient, basic::{client::TcpFtpClient, command::FtpBaseCommands}, command::FtpCommand};

pub fn main() -> ftp_client::Result<()> {
    let mut client = TcpFtpClient::new("ftp.ubuntu.com", None)?;
    println!("Connected !");
    
    FtpBaseCommands::Pwd.send(&mut client)?;
    println!("pwd command sent !");

    let res = client.read_response();
    match res {
        Ok(ok_res) => println!("{ok_res}"),
        Err(err_res) => println!("{err_res}"),
    }
    Ok(())
}