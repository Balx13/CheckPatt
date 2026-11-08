use std::{fs, io, path::PathBuf};

pub struct AppPaths {
    pub config: PathBuf,    // felhasználói beállítások
    pub userdata: PathBuf,  // felhasználói adatok
    pub datas: PathBuf,     // közös adatok (themes.ron, ...)
    pub databases: PathBuf, // sakk-adatbázisok: datas/db
    pub engines: PathBuf,   // sakkmotorok: datas/engines
    pub cache: PathBuf,
    pub logs: PathBuf,
}

impl AppPaths {
    pub fn installed() -> Self {
        let dirs = directories::ProjectDirs::from("com", "potato", "CheckPatt")
            .expect("nem található a felhasználó home könyvtára");
        let datas = dirs.data_local_dir().join("datas");
        Self {
            config: dirs.config_dir().to_path_buf(),
            userdata: dirs.data_dir().to_path_buf(),
            databases: datas.join("db"),
            engines: datas.join("engines"),
            cache: dirs.cache_dir().to_path_buf(),
            logs: dirs.data_local_dir().join("logs"),
            datas,
        }
    }

    pub fn portable() -> Self {
        let datas = PathBuf::from("datas");
        Self {
            config: PathBuf::from("config"),
            userdata: PathBuf::from("userdata"),
            databases: datas.join("db"),
            engines: datas.join("engines"),
            cache: PathBuf::from("cache"),
            logs: PathBuf::from("logs"),
            datas,
        }
    }

    pub fn init(&self) -> io::Result<()> {
        for dir in [
            &self.config,
            &self.userdata,
            &self.datas,
            &self.databases,
            &self.engines,
            &self.cache,
            &self.logs,
        ] {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}