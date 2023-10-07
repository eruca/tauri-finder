#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use std::ptr;
#[cfg(windows)]
use winapi::um::{
    fileapi::{
        FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, GetVolumePathNamesForVolumeNameW,
    },
    winnt::WCHAR,
};

use failure::Error;
use tokio::sync::mpsc::{Receiver, Sender};

#[cfg(windows)]
async fn get_windows_hard_disk_drivers() -> Vec<PathBuf> {
    // 使用FindFirstVolumeW函数来查找第一个磁盘驱动器的卷名称
    let mut volume_name: [WCHAR; 1024] = [0; 1024];
    let volume_handle = unsafe { FindFirstVolumeW(volume_name.as_mut(), volume_name.len() as u32) };
}

async fn get_root_dir() -> impl Iterator<Item = PathBuf> {
    #[cfg(unix)]
    return Some(PathBuf::from("/")).into_iter();
    #[cfg(windows)]
    return get_windows_hard_disk_drivers().await;
}

async fn recursive_walk_dirs(
    mut rec: Receiver<PathBuf>,
    sender: Sender<PathBuf>,
    sender_index: Sender<PathBuf>,
) -> Result<(), Error> {
    while let Some(path) = rec.recv().await {
        let mut dir = tokio::fs::read_dir(&path).await?;
        while let Some(ent) = dir.next_entry().await? {
            if let Ok(ft) = ent.file_type().await {
                if !ft.is_symlink() {
                    sender_index.send(ent.path()).await?;

                    if ft.is_dir() {
                        sender.send(ent.path()).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
