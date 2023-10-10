#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{
    borrow::Cow,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use failure::Error;
use jwalk::WalkDirGeneric;
use ormlite::{sqlite::Sqlite, Pool};
use tokio::sync::mpsc::{channel, Receiver, Sender};
#[cfg(windows)]
use winapi::um::{
    fileapi::{GetDriveTypeW, GetLogicalDriveStringsW},
    winbase::{DRIVE_FIXED, DRIVE_NO_ROOT_DIR},
};

use crate::{finder::Setting, models::paths::insert_paths};

// const CHANNEL_SIZE: usize = 256;

// pub async fn listen(pool: Pool<Sqlite>, mut rx: Receiver<PathBuf>) -> Result<(), Error> {
//     while let Some(s) = rx.recv().await {
//         println!("PathBuf: {:?}", &s);
//         insert_paths(&pool, s.as_path()).await?;
//     }
//     Ok(())
// }

pub fn walk_path(
    path: &Path,
    setting: Arc<Setting>,
    exclude_paths: Vec<String>,
) -> WalkDirGeneric<((), ())> {
    let mut ex = setting.excludes.clone();
    ex.extend(exclude_paths);

    jwalk::WalkDir::new(path).process_read_dir(move |_, _, _, children| {
        children.iter_mut().for_each(|dir_entry_result| {
            if let Ok(dir_entry) = dir_entry_result {
                let pb = normalize_path(dir_entry.path());
                let curr_path = pb.to_str().unwrap_or("");

                if ex
                    .iter()
                    .any(|r| curr_path == "" || curr_path.contains(r.as_str()))
                {
                    dir_entry.read_children_path = None;
                }
            }
        })
    })
}

pub async fn walk(pool: Pool<Sqlite>, setting: Arc<Setting>) -> Result<(), Error> {
    match dirs::home_dir() {
        Some(home) => {
            for entry in walk_path(&home, setting, vec![])
                .into_iter()
                .filter_map(|e| e.ok())
            {
                insert_paths(&pool, entry.path().as_path()).await?;
            }
        }
        None => {}
    }
    // for path in get_root_dir().await {
    //     println!("Path: {:?}", path);
    //     for entry in walkdir::WalkDir::new(path.as_path())
    //         .into_iter()
    //         .filter_map(|e| match e {
    //             Ok(de) => match de.path().to_str() {
    //                 Some(p) => {
    //                     if setting.excludes.iter().any(|s| p.contains(s)) {
    //                         None
    //                     } else {
    //                         Some(de)
    //                     }
    //                 }
    //                 None => None,
    //             },
    //             _ => None,
    //         })
    //     {
    //         let path = entry.path().to_str().expect("entry.path to_str failed");
    //         if !setting.excludes.iter().any(|s| path.contains(s)) {
    //             if !entry.file_type().is_symlink() {
    //                 sx.send(entry.into_path()).await?;
    //             }
    //         }
    //     }
    // }
    Ok(())
}

// pub async fn walk(sender2: Sender<Arc<PathBuf>>) -> Result<(), Error> {
//     let (sender, receiver) = channel(CHANNEL_SIZE);

//     tokio::spawn(recursive_walk_dirs(receiver, sender.clone(), sender2));

//     println!("fininsh walk");
//     Ok(())
// }

#[cfg(windows)]
async fn get_windows_hard_disk_drivers() -> Vec<PathBuf> {
    // 使用FindFirstVolumeW函数来查找第一个磁盘驱动器的卷名称
    let mut buffer = [0u16; 256];
    let buffer_size = buffer.len() as u32;

    if unsafe { GetLogicalDriveStringsW(buffer_size, buffer.as_mut_ptr()) } > 0 {
        let driver_strings = OsString::from_wide(&buffer);
        return driver_strings
            .into_string()
            .unwrap()
            .split('\0')
            .filter(|driver| !driver.is_empty())
            .filter(|driver| {
                let driver_type = unsafe {
                    GetDriveTypeW(
                        OsString::from(*driver)
                            .encode_wide()
                            .collect::<Vec<u16>>()
                            .as_ptr(),
                    )
                };
                println!("Fixed hard driver: {} - {}", driver, driver_type);
                driver_type == DRIVE_FIXED || driver_type == DRIVE_NO_ROOT_DIR
            })
            .map(|driver| normalize_path(driver.into()))
            .collect();
    }
    vec![]
}

async fn get_root_dir() -> impl Iterator<Item = PathBuf> {
    #[cfg(unix)]
    return Some(PathBuf::from("/")).into_iter();
    #[cfg(windows)]
    return get_windows_hard_disk_drivers().await.into_iter();
}

// async fn recursive_walk_dirs(
//     mut rec: Receiver<Arc<PathBuf>>,
//     sender_index: Sender<Arc<PathBuf>>,
// ) -> Result<(), Error> {
//     while let Some(de) = rec.recv().await {
//         // println!("recursive: {:?} {:?}", de.path(), de.file_name());
//         if let Ok(mut dir) = tokio::fs::read_dir(de.as_path()).await {
//             while let Some(ent) = dir.next_entry().await? {
//                 if let Ok(ft) = ent.file_type().await {
//                     if !ft.is_symlink() {
//                         let arc_ent = Arc::new(ent.path());
//                         sender_index.send(arc_ent.clone()).await?;
//                     }
//                 }
//             }
//         } else {
//             println!("not {:?}", de.as_path());
//         }
//     }

//     println!("finish recursive walk dirs");
//     Ok(())
// }

#[cfg(windows)]
pub fn normalize_path(path: PathBuf) -> PathBuf {
    let path2 = path.to_string_lossy().to_string();
    if path2.contains("\\") {
        path2.replace("\\", "/").into()
    } else {
        path
    }
}

#[cfg(unix)]
pub fn normalize_path(path: PathBuf) -> PathBuf {
    path
}

mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use super::get_root_dir;
    use crate::{filesys::walk_path, finder::Setting};

    #[test]
    fn it_works() {
        let mut iter = dirs::home_dir()
            .into_iter()
            .chain(Some("/".into()).into_iter());

        assert_eq!(iter.next(), Some("/Users/nick".into()));
        assert_eq!(iter.next(), Some("/".into()));
    }

    #[test]
    fn dir_walk() {
        let dir = Path::new("/Users/nick/Downloads");
        let start = std::time::SystemTime::now();
        for path in walk_path(dir, Arc::new(Setting::new()), vec![]) {
            println!("{:?}", path.unwrap().path());
        }
        println!(
            "{}ms",
            std::time::SystemTime::now()
                .duration_since(start)
                .unwrap()
                .as_millis()
        );
    }
}
