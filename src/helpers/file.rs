use std::{fs::{create_dir_all, File}, io::{Read, Write}, path::{Path, PathBuf}, thread, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use reqwest::blocking::ClientBuilder;



#[derive(Clone)]
pub struct DownloadInfo {
    pub url: String,
    pub dest: PathBuf,
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


//get download path dir
pub fn get_download_dir(app_name: &str) -> PathBuf {
    let mut path = dirs_next::home_dir().expect("Could not determine data directory");
    path.push(format!(".xupg/module/downloads/{}", app_name));
    path
}

pub fn get_download_path(app_name: &str, file_name: &str) -> PathBuf {
    let mut path = get_download_dir(app_name);
    // path.push(format!(".xupg/module/downloads/{}/{}", app_name, file_name));
    path.push(file_name);
    path
}

pub fn download_with_progress(url: &str, dest: &Path, pb: ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(dest.parent().unwrap())?;

    let host = reqwest::Url::parse(url)?.host_str().unwrap().to_string();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT_ENCODING, reqwest::header::HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("*/*"));
    headers.insert(reqwest::header::CONNECTION, reqwest::header::HeaderValue::from_static("keep-alive"));
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("PostmanRuntime/7.42.0"));
    headers.insert(reqwest::header::HOST, reqwest::header::HeaderValue::from_str(&host).unwrap());

    // Create a client with an extended timeout
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(300))  // 5 minutes timeout
        .connection_verbose(true)
        .default_headers(headers)
        .build()?;

    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    if total_size <= 0 {
        return Err(format!("Error Getting file infu").into());
    }

    //  get headers and print them
    
    let mut file = File::create(dest)?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192];

    pb.set_length(total_size);
    
    loop {
        match response.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(n) => {
                file.write_all(&buffer[..n])?;
                downloaded += n as u64;
                pb.set_position(downloaded);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    
    // download size in mb
    let downloaded = downloaded / 1024 / 1024;
    pb.finish_with_message(format!("Downloaded file of size: {:?} Mb to {:?}", downloaded, dest.display()));
    Ok(())
}

pub fn download_multiple_files(files: Vec<DownloadInfo>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut threads = vec![];
    // let mut successful: Vec<DownloadInfo> = vec![];
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
                // delete the file if download fails
                if download_info.dest.exists() {
                    std::fs::remove_file(download_info.dest).unwrap();
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
    Ok(true)
}

// list files in a directory
pub fn list_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in dir.read_dir().expect("Failed to read directory") {
            if let Ok(entry) = entry {
                files.push(entry.path());
            }
        }
    }
    files
}