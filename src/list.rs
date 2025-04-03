use std::path::PathBuf;

use anyhow::Result;
use byte_unit::{Byte, UnitType};

use crate::{cli, utils};

pub(crate) fn list_files_in_dir(d: &str) -> Result<Vec<PathBuf>> {
    let mut res = vec![];
    for e in std::fs::read_dir(d)? {
        let Ok(e) = e else { continue };
        let Ok(ft) = e.file_type() else { continue };
        if ft.is_file() {
            res.push(e.path())
        }
    }
    Ok(res)
}

pub(crate) fn list_files_in_xdg() -> Result<Vec<PathBuf>> {
    let xdg = xdg::BaseDirectories::with_prefix(utils::XDG_PREFIX)?;
    Ok(xdg.list_data_files(""))
}

pub(crate) fn list_files(dir: Option<impl AsRef<str>>) -> Result<Vec<PathBuf>> {
    match dir {
        Some(d) => list_files_in_dir(d.as_ref()),
        None => list_files_in_xdg(),
    }
}

pub fn run(cli: cli::List) -> Result<()> {
    let mut files = list_files(cli.dir.as_ref())?;

    // deterministic order
    files.sort();

    for f in files {
        let Some(f_str) = f.as_path().to_str() else {
            continue;
        };
        print!("{}", f_str);
        if cli.long {
            if let Ok(info) = std::fs::metadata(f) {
                let b = Byte::from_u64(info.len());
                let b_approx = b.get_appropriate_unit(UnitType::Binary);
                print!(" {b_approx:#.2}");
            }
        }
        println!("");
    }

    Ok(())
}
