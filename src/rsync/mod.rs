use std::path::{Path, PathBuf};
use tokio::process::*;
use std::error::Error;
use std::fmt::Display;
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub struct RsyncError {
    pub msg: String,
    pub status: Option<i32>,
}

impl Display for RsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RsyncError with code {}\nMessage: {}", self.status.unwrap_or(-1), self.msg)
    }
}

impl Error for RsyncError {}

pub async fn rsync<P: AsRef<Path>>(
    src: P,
    host: &String,
    dst_path: P
) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("rsync");
    cmd.arg("-arvh");
    cmd.arg(src.as_ref());
    cmd.arg(format!("{}:{}", host, dst_path.as_ref().display()));
    let mut child = cmd.spawn()?;
    let status = child.wait().await?;
    match status.success() {
        true => Ok(()),
        false => {
            let child_stderr = child.stderr.take().unwrap();
            let mut reader = tokio::io::BufReader::new(child_stderr);
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            Err(Box::new(RsyncError { msg: line , status: status.code()}))
        }
    }
}