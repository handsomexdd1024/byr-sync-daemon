use std::path::{Path, PathBuf};
use tokio::process::*;

pub async fn rsync<P: AsRef<Path>>(src: P, dst: P) -> Result<(), Box<dyn std::error::Error>> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let mut cmd = Command::new("rsync");
    cmd.arg("-arvh").arg(src).arg(dst);
    let mut rclone_ps = cmd.spawn()?;
    rclone_ps.wait().await.unwrap();
    Ok(())
}
