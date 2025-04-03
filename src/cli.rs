#[derive(Debug, clap::Parser)]
pub struct List {
    /// Storage directory
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
    /// List more info about each file
    #[arg(short = 'l')]
    pub long: bool,
}

#[derive(Debug, clap::Parser)]
pub struct Clear {
    /// Storage directory
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub struct Dir {
    /// Storage directory
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub struct Serve {
    /// Path to the unix socket to serve
    #[arg(long = "socket")]
    pub unix_socket: Option<String>,
    /// Storage directory
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
    /// Write all message to this single (.jsonl) file.
    /// This means all clients implictly participate in a single trace
    /// and should not send `OPEN`.
    #[arg(long = "into-file")]
    pub single_file: Option<String>,
    /// Daemonize on startup
    #[arg(long = "daemonize")]
    pub daemonize: bool,
}

#[derive(Debug, clap::Parser)]
pub struct GetTEF {
    /// The file to remove. Can be "latest".
    #[arg(index = 1, value_name = "FILE")]
    pub jsonl_file: String,
    /// Storage directory.
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
    /// Output file (.json file)
    #[arg(short = 'o', long = "out")]
    pub o: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// List log files
    List(List),
    /// Delete log files
    Clear(Clear),
    /// Serve as a daemon
    Serve(Serve),
    /// get a file as a TEF file
    GetTEF(GetTEF),
    /// Show directory
    Dir(Dir),
}
