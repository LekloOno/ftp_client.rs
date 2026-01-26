use crate::{FtpClient, command::FtpCommand};

pub fn send_command(command: &dyn FtpCommand, client: &mut dyn FtpClient) {
    match command.send(client) {
        Ok(_) => println!("`{}` successfully sent !", command.to_string()),
        Err(err) => print!("An error while sending command `{}` : {}", command.to_string(), err),
    }
}

pub fn send_and_read(command: &dyn FtpCommand, client: &mut dyn FtpClient) {
    send_command(command, client);
    let res = client.read_response();

    match res {
        Ok(ok_res) => println!("{ok_res}"),
        Err(err_res) => println!("{err_res}"),
    };
}
