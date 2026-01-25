use crate::command::FtpCommand;

pub enum FtpBaseCommands {
    User,
    Pass,
    Pasv,
    Cwd,
    Pwd,
    Cdup,
    List,
    Quit,
}

impl FtpCommand for FtpBaseCommands {
    fn as_str(&self) -> &str {
        match self {
            FtpBaseCommands::User => todo!(),
            FtpBaseCommands::Pass => todo!(),
            FtpBaseCommands::Pasv => todo!(),
            FtpBaseCommands::Cwd => todo!(),
            FtpBaseCommands::Pwd => "PWD",
            FtpBaseCommands::Cdup => todo!(),
            FtpBaseCommands::List => todo!(),
            FtpBaseCommands::Quit => todo!(),
        }
    }
}