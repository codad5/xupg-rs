use std::{fs::{create_dir_all, File}, io::{Read, Write}, path::{Path, PathBuf}, thread};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[derive(Clone)]
pub struct DownloadInfo {
    url: String,
    dest: PathBuf,
}

impl DownloadInfo {
    pub fn new(url: String, dest: PathBuf) -> Self {
        Self { url, dest }
    }

    pub fn download_with_progress(&self, pb: ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
        download_with_progress(&self.url, &self.dest, pb)
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

pub fn get_download_path(app_name: &str, file_name: &str) -> PathBuf {
    let mut path = dirs_next::home_dir().expect("Could not determine data directory");
    path.push(format!(".xupg/module/downloads/{}/{}", app_name, file_name));
    path
}

pub fn download_with_progress(url: &str, dest: &Path, pb: ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(dest.parent().unwrap())?;

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let mut file = File::create(dest)?;

    let mut downloaded: u64 = 0;
    let mut buffer = [0; 1024];

    pb.set_length(total_size);

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message(format!("Downloaded file of size: {} bytes to {:?}", downloaded, dest.display()));
    Ok(())
}

pub fn download_multiple_files(files: Vec<DownloadInfo>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut threads = vec![];
    let pb = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {msg}")
        .unwrap()
        .progress_chars("##-");

    for file in files {
        let pb = pb.add(ProgressBar::new(100));
        pb.set_style(style.clone());
        pb.set_message(format!("Downloading {}", file.dest.display()));

        let download_info = file.clone();
        let pb_clone = pb.clone();
        threads.push(thread::spawn(move || {
            if let Err(e) = download_info.download_with_progress(pb_clone) {
                println!("Failed to download {}: {}", download_info.url(), e);
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
    Ok(true)
}
