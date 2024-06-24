pub mod command;
mod project_info;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::path_resolver::PathResolver;
use anyhow::Result;
use log::error;
use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use parking_lot::RwLock;
use project_info::ProjectInfos;
use tauri::{AppHandle, Manager};

pub type ProjectInfosWarpper = Arc<RwLock<ProjectInfos>>;
pub type ProjectManagerWarpper = Arc<RwLock<ProjectManager>>;

pub struct ProjectManager {
    watcher: RecommendedWatcher,
    project_infos: ProjectInfosWarpper,
    app: AppHandle,
}

impl ProjectManager {
    pub fn managed_by(app: &AppHandle) -> Result<()> {
        let data_path = Self::data_path(app.clone());
        if !data_path.exists() {
            Self::init_project_info(&data_path)?;
        }
        let project_infos = Arc::new(RwLock::new(Self::read_project_data(&data_path)?));

        let mut watcher = {
            let project_infos = project_infos.clone();
            let data_path = data_path.clone();
            let app = app.clone();
            recommended_watcher(move |event: Result<Event, _>| {
                if let Ok(event) = event {
                    let kind = event.kind;
                    if matches!(kind, EventKind::Create(_) | EventKind::Remove(_)) {
                        if let Ok(project_infos_new) = Self::read_project_data(data_path.clone()) {
                            *project_infos.write() = project_infos_new;
                            let _ = app.emit("project-list-update", ());
                        }
                    }
                }
            })?
        };

        watcher.watch(&data_path, RecursiveMode::NonRecursive)?;

        let manager = Self {
            watcher,
            project_infos,
            app: app.clone(),
        };
        let wrapper = Arc::new(RwLock::new(manager));
        app.manage(wrapper);

        Ok(())
    }

    pub fn update_project_infos<F: FnOnce(&mut ProjectInfos)>(&self, handler: F) -> Result<()> {
        handler(&mut self.project_infos.write());
        let toml = toml::to_string(&*self.project_infos.read());
        match toml {
            Ok(s) => fs::write(Self::data_path(self.app.clone()), s)?,
            Err(e) => error!("{e:?}"),
        }

        Ok(())
    }

    fn rewatch(&mut self) -> Result<()> {
        let data_path = Self::data_path(self.app.clone());
        let _ = self.watcher.unwatch(&data_path);
        self.watcher
            .watch(&data_path, RecursiveMode::NonRecursive)?;
        Ok(())
    }

    fn read_project_data<P: AsRef<Path>>(data_path: P) -> Result<ProjectInfos> {
        let data = fs::read_to_string(data_path.as_ref())?;
        let result = toml::from_str(&data)?;
        Ok(result)
    }

    fn init_project_info<P: AsRef<Path>>(data_path: P) -> Result<()> {
        fs::write(
            data_path.as_ref(),
            toml::to_string(&ProjectInfos::default()).unwrap(),
        )?;
        Ok(())
    }

    fn data_path(app: AppHandle) -> PathBuf {
        let path_resolver = PathResolver::new(app);
        path_resolver.project_dir().join(".projectlist.toml")
    }
}
