use crate::command::FtpCommand;

pub enum FtpBaseCommands {
    User { username: String },
    Pass { password: String },
    Pasv,
    Cwd { pathname: String },
    Pwd,
    Cdup,
    List {pathname: Option<String> },
    Quit,
}

impl ToString for FtpBaseCommands {
    fn to_string(&self) -> String {
        match self {
            FtpBaseCommands::User {username} => format!("USER {username}"),
            FtpBaseCommands::Pass {password}=> format!("PASS {password}"),
            FtpBaseCommands::Pasv => format!("PASV"),
            FtpBaseCommands::Cwd {pathname}=> format!("CWD {pathname}"),
            FtpBaseCommands::Pwd => format!("PWD"),
            FtpBaseCommands::Cdup => format!("CDUP"),
            FtpBaseCommands::List {pathname} =>
                format!("LIST{}", pathname
                    .as_ref()
                    .map_or(
                        "".to_string(),
                        |s| format!(" {}", s))
                ),
            FtpBaseCommands::Quit => format!("QUIT"),
        }
    }
}

impl FtpCommand for FtpBaseCommands {}