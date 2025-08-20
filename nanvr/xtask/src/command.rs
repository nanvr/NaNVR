use std::path::Path;
use xshell::{Shell, cmd};

pub fn zip(sh: &Shell, source: &Path) -> Result<(), xshell::Error> {
    let _push_guard = sh.push_dir(source);
    cmd!(sh, "zip -r9X {source} .").run()
}

pub fn unzip(sh: &Shell, source: &Path, destination: &Path) -> Result<(), xshell::Error> {
    cmd!(sh, "unzip {source} -d {destination}").run()
}

pub fn targz(sh: &Shell, source: &Path) -> Result<(), xshell::Error> {
    let parent_dir = source.parent().unwrap();
    let file_name = source.file_name().unwrap();

    cmd!(sh, "tar -czvf {source}.tar.gz -C {parent_dir} {file_name}").run()
}

pub fn download(sh: &Shell, url: &str, destination: &Path) -> Result<(), xshell::Error> {
    cmd!(sh, "curl -L -o {destination} --url {url}").run()
}

pub fn download_and_extract_zip(url: &str, destination: &Path) -> Result<(), xshell::Error> {
    let sh = Shell::new().unwrap();
    let temp_dir_guard = sh.create_temp_dir()?;

    let zip_file = temp_dir_guard.path().join("temp_download.zip");
    download(&sh, url, &zip_file)?;

    unzip(&sh, &zip_file, destination)
}
