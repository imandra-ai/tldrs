use std::{
    fs,
    io::{self, stdout, BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Result;

use crate::{cli, utils};

fn get_file_in_dir(file: &str, d: &str) -> Result<String> {
    let mut file2 = PathBuf::from(&d);
    file2.push(file);

    if fs::exists(&file2).ok() == Some(true) {
        Ok(file2.to_string_lossy().to_string())
    } else {
        anyhow::bail!("Tried {file:?} and {file2:?}, neither of which exists");
    }
}

fn find_latest_file(d: Option<impl AsRef<str>>) -> Result<String> {
    let mut files = crate::list::list_files(d)?;
    files.sort();

    let f: &PathBuf = files
        .last()
        .ok_or_else(|| anyhow::anyhow!("No files in directory"))?;
    Ok(f.to_string_lossy().to_string())
}

pub fn run(cli: cli::GetTEF) -> Result<()> {
    let mut file = cli.jsonl_file;

    if fs::exists(&file).ok() != Some(true) {
        if file == "latest" {
            file = find_latest_file(cli.dir.as_ref())?;
        } else {
            match cli.dir {
                None => anyhow::bail!("File {file:?} does not exist"),
                Some(d) => {
                    file = get_file_in_dir(&file, &d)?;
                }
            }
        }
    }

    log::info!("reading TEF trace from file {file:?}");
    let file_in = fs::File::open(file)?;
    let mut reader = BufReader::new(file_in);

    let out: Box<dyn io::Write> = match cli.o {
        Some(f) => Box::new(fs::File::create(f)?),
        None => Box::new(stdout().lock()),
    };
    let mut writer = BufWriter::new(out);

    utils::emit_tef(&mut reader, &mut writer)?;

    Ok(())
}
