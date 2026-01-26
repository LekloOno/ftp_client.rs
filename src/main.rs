use ftp_client::{
    FtpClient,
    log,
    basic::{client::TcpFtpClient, command::FtpBaseCommands},
    response::success
};

pub fn main() -> ftp_client::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let hostname = args.get(1).map_or("ftp.free.fr", |s| s.as_str());
    let mut client = TcpFtpClient::new(hostname, None)?;
    println!("Connecting to {hostname}");
    
    let res = client.read_response();

    let Ok(ok_res) = res else {
        let err = res.unwrap_err();
        println!("Couldn't establish connection: {err}");
        return Err(err);
    };
    
    println!("{ok_res}");

    log::send_and_read(&FtpBaseCommands::User { username: "anonymous".to_string() }, &mut client);
    log::send_and_read(&FtpBaseCommands::Pass { password: "pswd".to_string() }, &mut client);
    log::send_and_read(&FtpBaseCommands::Pwd, &mut client);

    log::send_command(&FtpBaseCommands::Pasv, &mut client);
    
    let res = client.read_response();

    let Ok(ok_res) = res else {
        println!("Error on pasv {}", res.unwrap_err());
        return Ok(());
    };

    let ftp_client::FtpSuccessRes::EnteringPassiveMode {msg} = ok_res else {
        println!("Unexpected response {}", ok_res);
        return Ok(());
    };

    let (ip, port) = success::parse_pasv_reply(&msg).expect("msg");

    println!("{}:{}", ip, port);

    client.open_data(&hostname, &port.to_string()).expect("Error while opening data port.");

    log::send_command(&FtpBaseCommands::List { pathname: None }, &mut client);

    std::io::copy(client.data_stream().unwrap(), &mut std::io::stdout())?;

    println!("Completed");
    
    Ok(())
}