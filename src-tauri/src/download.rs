use anyhow::Result;
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct JavaRelease {
    pub(crate) downloadUrl: String,
    pub(crate) featureVersion: u32,
    pub(crate) packageType: String,
    pub(crate) version: String,
    pub(crate) filename: String,
    pub(crate) size: u64,
}

pub fn get_platform_name() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";
    #[cfg(target_os = "linux")]
    return "linux";
    #[cfg(target_os = "macos")]
    return "macos";
}

pub fn get_package_type() -> &'static str {
    #[cfg(target_os = "windows")]
    return "zip";
    #[cfg(target_family = "unix")]
    return "tar.gz";
}

pub fn get_arch_name() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "amd64";
    #[cfg(target_arch = "aarch64")]
    return "aarch64";
}

pub fn get_file_size() -> u64 {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return 137580532;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return 140085493;
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return 120056376;
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return 124832259;
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    return 48584493;
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return 117540219;
}

pub fn fetch_latest_release() -> JavaRelease {
    let filename = format!(
        "bellsoft-jre25+37-{}-{}-full.{}", 
        get_platform_name(),
        get_arch_name(),
        get_package_type()
    );
    
    JavaRelease { 
        downloadUrl: format!("https://launch.quickfirecorp.ru/java/{}", filename),
        packageType: get_package_type().to_owned(),
        featureVersion: 25, 
        version: "25+37".to_owned(), 
        filename: filename, 
        size: get_file_size()
    }
}

pub type ProgressCallback = dyn Fn(u64, u64);

/// Download the file and report progress through the callback.
pub fn download_file(url: &str, dest: &PathBuf, total_size: u64, progress: &ProgressCallback) -> Result<()> {
    let mut response = reqwest::blocking::get(url)?.error_for_status()?;
    let mut file = File::create(dest)?;
    let mut buffer = [0; 8192];
    let mut downloaded: u64 = 0;
    loop {
        let n = response.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        progress(downloaded, total_size);
    }
    Ok(())
}
