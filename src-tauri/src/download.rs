use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    fs::{File},
    io::{Read, Write},
    path::PathBuf,
};

// Adoptium API
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct AdoptiumResponse {
    binary: AdoptiumBinary,
    release_name: String,
    vendor: String,
    version: AdoptiumVersion,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct AdoptiumBinary {
    architecture: String,
    os: String,
    package: AdoptiumPackage,
    jvm_impl: String,
    image_type: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct AdoptiumPackage {
    link: String,
    name: String,
    size: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct AdoptiumVersion {
    major: u32,
    build: u32,
    openjdk_version: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct GitHubTagInfo {
    // tag: String
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
    return "x86";

    #[cfg(target_arch = "aarch64")]
    return "aarch64";
}

pub fn get_arch_name_v2() -> &'static str {

    #[cfg(target_arch = "x86_64")]
    return "amd64";

    #[cfg(target_arch = "aarch64")]
    return "aarch64";
}

pub fn get_arch_name_for_adoptium() -> &'static str {

    #[cfg(target_arch = "x86_64")]
    return "x64";

    #[cfg(target_arch = "aarch64")]
    return "aarch64";
}

pub fn fetch_latest_release() -> Result<JavaRelease> {
    fetch_latest_release_from_api().or_else(|_| Ok(fetch_emergency_release()))
}

fn fetch_latest_release_from_api() -> Result<JavaRelease> {
    let url = format!("https://api.adoptium.net/v3/assets/latest/25/hotspot?os={}&architecture={}&image_type=jdk&vendor=eclipse",
        get_platform_name(), get_arch_name_v2());
    let resp = reqwest::blocking::get(url)?.error_for_status()?;
    let releases: Vec<AdoptiumResponse> = serde_json::from_reader(resp)?;

    let release = releases.into_iter().next().ok_or_else(|| anyhow!("No releases found"))?;
    
    // Convert Adoptium response to JavaRelease format
    Ok(JavaRelease {
        downloadUrl: release.binary.package.link,
        featureVersion: release.version.major,
        packageType: get_package_type().to_owned(),
        version: release.release_name,
        filename: release.binary.package.name,
        size: release.binary.package.size,
    })
}

fn fetch_emergency_release() -> JavaRelease {
    JavaRelease { 
        downloadUrl: format!("https://github.com/adoptium/temurin25-binaries/releases/download/jdk-25%2B36/OpenJDK25U-jdk_{}_{}_hotspot_25_36.{}", 
            get_arch_name_for_adoptium(), get_platform_name(), get_package_type()),
        packageType: get_package_type().to_owned(),
        featureVersion: 25, 
        version: "jdk-25+36".to_owned(), 
        filename: format!("OpenJDK25U-jdk_{}_{}_hotspot_25_36.{}", get_arch_name_for_adoptium(), get_platform_name(), get_package_type()), 
        size: 58401507 
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
