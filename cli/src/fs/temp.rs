use crate::generators::compose::{
    generate_compose_backend, generate_compose_backend_sa, generate_compose_frontend,
};
use serde_json::{from_reader, json, Value};
use std::fs::OpenOptions;
use std::fs::{create_dir, read_to_string, remove_file, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Manage file paths and contents for system
pub(crate) struct TempFs {
    /// temp directory for a11ywatch
    tmp_dir: String,
    /// temp app directory for a11ywatch
    app_dir: String,
    /// backend infra compose file
    pub backend_compose: String,
    /// frontend compose file
    pub frontend_compose: String,
    /// results of scan file location
    pub results_file: String,
    /// results of github html file location
    pub results_github_file: String,
    /// infra config file
    pub config_file: String,
}

/// merge two values together from left-right.
fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

/// standard file system handling methods
pub(crate) trait Fs {
    fn new() -> Self;
    fn set_token(&self) {}
    fn set_cv_url(&self) {}
    fn set_cv_token(&self) {}
    fn ensure_temp_dir() {}
    fn create_compose_backend_file(&self, standalone: &bool);
    fn create_compose_frontend_file(&self);
    fn sync();
    fn build(&self);
}

/// temporary file system
impl TempFs {
    pub fn new() -> Self {
        let tmp_dir = std::env::temp_dir().display().to_string();
        let app_dir = format!("{}/a11ywatch", &tmp_dir);
        let config_file = format!("{}/config.json", &app_dir);
        let results_file = format!("{}/results.json", &app_dir);
        let results_github_file = format!("{}/results_github.json", &app_dir);

        Self {
            backend_compose: format!("{}/compose.yml", app_dir),
            frontend_compose: format!("{}/compose.frontend.yml", app_dir),
            results_file,
            config_file,
            app_dir,
            tmp_dir,
            results_github_file,
        }
    }

    /// get the apps temp directory location
    pub fn get_temp_dir(&mut self) -> &String {
        &self.app_dir
    }

    // build project directory and make sure it always exist
    pub fn build(&self) {
        TempFs::ensure_temp_dir(&self.tmp_dir, &self.app_dir).unwrap(); // create project dir
        TempFs::sync(&self.app_dir, &self.config_file).unwrap(); // sync versions
    }

    /// create compose backend file is does not exist
    pub fn create_compose_backend_file(&mut self, standalone: &bool) -> std::io::Result<()> {
        self.build();
        let sa = standalone == &true;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.backend_compose)
            .unwrap();

        let gfile = if sa {
            generate_compose_backend_sa()
        } else {
            generate_compose_backend()
        };
        file.write_all(&gfile.as_bytes())?;

        Ok(())
    }

    /// create compose frontend file is does not exist
    pub fn create_compose_frontend_file(&mut self) -> std::io::Result<()> {
        self.build();
        if !Path::new(&self.frontend_compose).exists() {
            let mut file = File::create(&self.frontend_compose)?;
            file.write_all(&generate_compose_frontend().as_bytes())?;
        }
        Ok(())
    }

    /// read results from scan to string
    pub fn read_results(&self) -> String {
        let mut data = String::new();

        if Path::new(&self.results_file).exists() {
            let mut file = File::open(&self.results_file).unwrap();
            file.read_to_string(&mut data).unwrap();
        }

        data
    }

    /// read results from scan to string
    pub fn read_results_github(&self) -> String {
        let mut data = String::new();

        if Path::new(&self.results_github_file).exists() {
            let mut file = File::open(&self.results_github_file).unwrap();
            file.read_to_string(&mut data).unwrap();
        }

        data
    }

    /// get the api token to use for request
    pub fn get_token(&self) -> String {
        if Path::new(&self.config_file).exists() {
            let file = File::open(&self.config_file).unwrap();
            let json: Value = from_reader(&file).unwrap();

            json["token"].as_str().unwrap_or_default().into()
        } else {
            "".into()
        }
    }

    /// set the api token to use for request
    pub fn set_token(&self, token: &String) -> std::io::Result<()> {
        self.build();
        let file = File::open(&self.config_file)?;
        let mut prev_json: Value = from_reader(&file)?;

        let json = json!({ "token": &token });

        merge(&mut prev_json, &json);

        let mut file = File::create(&self.config_file)?;

        file.write_all(&prev_json.to_string().as_bytes())?;

        Ok(())
    }

    /// set the Computer Vision api token to use for request
    pub fn set_cv_token(&self, token: &String) -> std::io::Result<()> {
        self.build();
        let file = File::open(&self.config_file)?;
        let mut prev_json: Value = from_reader(&file)?;

        let json = json!({ "cv_token": &token });

        merge(&mut prev_json, &json);

        let mut file = File::create(&self.config_file)?;

        file.write_all(&prev_json.to_string().as_bytes())?;

        self.create_env_file().unwrap();

        Ok(())
    }

    /// set the Computer Vision url to use for request
    pub fn set_cv_url(&self, u: &String) -> std::io::Result<()> {
        self.build();
        let file = File::open(&self.config_file)?;
        let mut prev_json: Value = from_reader(&file)?;

        let json = json!({ "cv_url": &u });

        merge(&mut prev_json, &json);

        let mut file = File::create(&self.config_file)?;

        file.write_all(&prev_json.to_string().as_bytes())?;

        self.create_env_file().unwrap();

        Ok(())
    }

    /// create an env file from the config
    pub fn create_env_file(&self) -> std::io::Result<()> {
        self.build();
        let file = File::open(&self.config_file)?;
        let prev_json: Value = from_reader(&file)?;
        let env_path = format!("{}/.env", &self.app_dir);
        let env_path_tmp = format!("{}/env.txt", &self.app_dir);

        if !Path::new(&env_path).exists() {
            File::create(&env_path)?;
        };
        if !Path::new(&env_path_tmp).exists() {
            File::create(&env_path_tmp)?;
        };

        let file = File::open(&env_path)?;
        let file_tmp = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&env_path_tmp)
            .unwrap();

        let cv_token = prev_json["cv_token"].as_str().unwrap_or_default();
        let cv_url = prev_json["cv_url"].as_str().unwrap_or_default();

        // custom API keys
        let c_v_s_k = "COMPUTER_VISION_SUBSCRIPTION_KEY";
        let c_v_e = "COMPUTER_VISION_ENDPOINT";

        // keep track of keys already wrote to file.
        let mut wrote_c_v_s_k = false;
        let mut wrote_c_v_e = false;
        let mut wrote_crawler = false;

        let mut writer: LineWriter<File> = LineWriter::new(file_tmp);
        let reader = BufReader::new(&file);

        // map allowed env keys to file
        for line in reader.lines() {
            if let Ok(item) = line {
                // crawler exist in env path ignore re-write
                if item.starts_with(&"CRAWLER_IMAGE=") {
                    wrote_crawler = true;
                };
                if !cv_token.is_empty() && item.contains(&c_v_s_k) {
                    writer.write_all(format!("{c_v_s_k}={}\n", cv_token).to_string().as_bytes())?;
                    wrote_c_v_s_k = true;
                } else if !cv_url.is_empty() && item.contains(&c_v_e) {
                    writer.write_all(format!("{c_v_e}={}\n", cv_url).to_string().as_bytes())?;
                    wrote_c_v_e = true;
                } else {
                    writer.write_all(format!("{}\n", item).to_string().as_bytes())?;
                };
            }
        }

        if !cv_token.is_empty() && !wrote_c_v_s_k {
            writer.write_all(format!("{c_v_s_k}={}\n", cv_token).to_string().as_bytes())?;
        };

        if !cv_url.is_empty() && !wrote_c_v_e {
            writer.write_all(
                format!("COMPUTER_VISION_ENDPOINT={}\n", cv_url)
                    .to_string()
                    .as_bytes(),
            )?;
        };

        if !wrote_crawler {
            // m1 max
            if cfg!(all(
                target_os = "macos",
                target_arch = "aarch64",
                target_pointer_width = "64"
            )) {
                writer.write_all("CRAWLER_IMAGE=darwin-arm64\n".to_string().as_bytes())?;
                // stand alone image if used
                writer.write_all("A11YWATCH_IMAGE=darwin\n".to_string().as_bytes())?;
            } else if cfg!(target_os = "linux") {
                let info = os_info::get();
                let os_type = info.os_type();

                let c_os = if os_type == os_info::Type::Ubuntu {
                    "ubuntu"
                } else if os_type == os_info::Type::Alpine {
                    "alpine"
                } else {
                    "debian"
                };

                writer.write_all(format!("CRAWLER_IMAGE={c_os}\n").to_string().as_bytes())?;
            };
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(false)
            .open(&env_path)?;

        writer.flush()?;
        file.write_all(
            read_to_string(Path::new(&env_path_tmp))
                .unwrap_or_default()
                .to_string()
                .as_bytes(),
        )?;
        remove_file(Path::new(&env_path_tmp))?;

        Ok(())
    }

    /// write to a file json results
    pub fn save_results(&self, json: &serde_json::Value) -> std::io::Result<()> {
        self.build();
        let mut file = File::create(&self.results_file)?;
        file.write_all(&json.to_string().as_bytes())?;

        Ok(())
    }

    /// write to a cwd csv results
    pub fn save_csv_results(&self, buf: &[u8], file_name: &String) -> std::io::Result<()> {
        self.build();
        let mut file = File::create(file_name)?;

        file.write_all(buf)?;

        Ok(())
    }

    /// write to the github upload file
    pub fn save_github_results(&self, json: &serde_json::Value) -> std::io::Result<()> {
        self.build();
        let mut file = File::create(&self.results_github_file)?;
        file.write_all(&json.to_string().as_bytes())?;

        Ok(())
    }

    /// make sure the tmp directory is created for the app
    fn ensure_temp_dir(tmp_dir: &str, app_dir: &str) -> std::io::Result<()> {
        if !Path::new(tmp_dir).exists() {
            match create_dir(tmp_dir) {
                _ => ()
            }
        }
        if !Path::new(&app_dir).exists() {
            match create_dir(app_dir) {
                _ => ()
            }
        }
        Ok(())
    }

    /// determine whether the temp dir needs to re-init from a new version change
    fn sync(app_dir: &str, config_file: &str) -> std::io::Result<()> {
        let version: &'static str = env!("CARGO_PKG_VERSION");

        if Path::new(&config_file).exists() {
            let file = File::open(&config_file)?;
            let mut prev_json: Value = from_reader(&file).expect("file should be proper JSON");
            let current_version = prev_json
                .get("version")
                .expect("file should have version key");

            if version != current_version {
                if !app_dir.is_empty() {
                    // TODO: handle directory cleanup
                }

                let json = json!({ "version": version });

                merge(&mut prev_json, &json);

                let mut file = File::create(&config_file)?;

                file.write_all(&prev_json.to_string().as_bytes())?;
            }
        } else {
            let mut file = File::create(&config_file)?;
            let json = json!({ "version": version });

            file.write_all(&json.to_string().as_bytes())?;
        }

        Ok(())
    }
}

/// generic file system handling for application.
impl Fs for TempFs {
    fn new() -> Self {
        let tmp_dir = std::env::temp_dir().display().to_string();
        let app_dir = format!("{}/a11ywatch", &tmp_dir);
        let results_file = format!("{}/results.json", &app_dir);
        let results_github_file = format!("{}/results_github.json", &app_dir);

        Self {
            backend_compose: format!("{}/compose.yml", app_dir),
            frontend_compose: format!("{}/compose.frontend.yml", app_dir),
            config_file: format!("{}/compose.json", app_dir),
            results_file,
            app_dir,
            tmp_dir,
            results_github_file,
        }
    }
    fn ensure_temp_dir() {}
    fn create_compose_backend_file(&self, _standalone: &bool) {}
    fn create_compose_frontend_file(&self) {}
    fn set_token(&self) {}
    fn set_cv_url(&self) {}
    fn set_cv_token(&self) {}
    fn sync() {}
    fn build(&self) {}
}
