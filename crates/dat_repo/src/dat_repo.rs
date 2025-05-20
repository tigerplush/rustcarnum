use std::path::Path;

use bevy::{asset::io::memory::Dir, platform::collections::HashMap, prelude::*};
use bevy_dat::Dat;
use bevy_mes::Mes;

#[derive(Debug)]
pub enum DatRepoLoadingError {
    FileNotFound(String),
    PatternMatch,
    ArtTypeNotFound(MesFileType),
    EntryNotFound(u32),
    InvalidArtType,
}

#[derive(Default, Resource)]
pub struct DatRepo {
    pub dir: Dir,
    dats: Vec<Dat>,
    dat_handles: Vec<Handle<Dat>>,
    pub mes_handles: Vec<(MesFileType, Handle<Mes>)>,
    mes_filenames: Vec<(MesFileType, String)>,
    mes: HashMap<MesFileType, Mes>,
}

impl DatRepo {
    pub fn add_dat(&mut self, handle: Handle<Dat>) {
        self.dat_handles.push(handle);
    }

    pub fn loaded_dat(&self, asset_server: &AssetServer) -> bool {
        self.dat_handles
            .iter()
            .all(|handle| asset_server.is_loaded(handle))
    }

    pub fn fill(&mut self, dats: &Assets<Dat>) {
        for handle in &self.dat_handles {
            let dat = dats.get(handle).unwrap();
            self.dats.push(dat.clone());
        }
        self.dat_handles.clear();
    }

    /// Loads a file from the .dat file table entry and adds it to the
    /// memory storage.
    pub fn load_file_directly(&self, pattern: &str) -> Result<String, DatRepoLoadingError> {
        for dat in self.dats.iter().rev() {
            if let Some(entry) = dat.get(pattern) {
                self.dir
                    .insert_asset(Path::new(&entry.filename), dat.bytes(entry));
                return Ok(format!("memory://{}", entry.filename));
            }
        }
        Err(DatRepoLoadingError::FileNotFound(pattern.to_string()))
    }

    pub fn load_file_match(
        &self,
        dats: &Assets<Dat>,
        pattern: impl Fn(&String) -> bool,
    ) -> Result<String, DatRepoLoadingError> {
        for handle in self.dat_handles.iter().rev() {
            let Some(dat) = dats.get(handle) else {
                continue;
            };
            if let Some(entry) = dat.get_fn(&pattern) {
                self.dir
                    .insert_asset(Path::new(&entry.filename), dat.bytes(entry));
                return Ok(format!("memory://{}", entry.filename));
            }
        }
        Err(DatRepoLoadingError::PatternMatch)
    }

    pub fn add_mes_file_to_load(&mut self, key: MesFileType, filename: &str) {
        self.mes_filenames.push((key, filename.to_string()));
    }

    pub fn load_next(&mut self, asset_server: &AssetServer) -> bool {
        if let Some((key, filename)) = self.mes_filenames.pop() {
            match self.load_file_directly(&filename) {
                Ok(path) => {
                    self.mes_handles.push((key, asset_server.load(path)));
                }
                Err(err) => warn!("{:?}", err),
            }
            return true;
        }
        false
    }

    pub fn loaded_all(&self, asset_server: &AssetServer) -> bool {
        self.mes_handles
            .iter()
            .all(|(_, handle)| asset_server.is_loaded(handle))
    }

    pub fn insert_mes(&mut self, key: MesFileType, mes: &Mes) {
        self.mes.insert(key, mes.clone());
    }

    pub fn load_file_by_num(&self, num: u32) -> Result<String, DatRepoLoadingError> {
        let art_type = match num {
            _ => MesFileType::Name(Name::Interface),
        };
        let Some(mes) = self.mes.get(&art_type) else {
            return Err(DatRepoLoadingError::ArtTypeNotFound(art_type));
        };
        let Some((_, file_name)) = mes.contents.get(&num) else {
            return Err(DatRepoLoadingError::EntryNotFound(num));
        };
        let path: &str = art_type.try_into()?;
        let file = format!("{}{}", path, file_name);
        self.load_file_directly(&file)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MesFileType {
    Description,
    ItemEffect,
    Critter(MesCritterType),
    Name(Name),
    Portrait(Portrait),
}

impl TryInto<&str> for MesFileType {
    type Error = DatRepoLoadingError;
    fn try_into(self) -> std::result::Result<&'static str, Self::Error> {
        let path = match self {
            MesFileType::Name(Name::Scenery) => "art\\scenery\\",
            MesFileType::Name(Name::Interface) => "art\\interface\\",
            _ => return Err(DatRepoLoadingError::InvalidArtType),
        };
        Ok(path)
    }
    // fn try_into(value: &MesFileType) -> std::result::Result<Self, Self::Error> {

    //     let mes_type = match value {
    //         "art\\scenery\\" => MesFileType::Name(Name::Scenery),
    //         "art\\interface\\" => MesFileType::Name(Name::Interface),
    //         "art\\unique_npc\\" => MesFileType::Name(Name::UniqueNpc),
    //         "art\\monster\\" => MesFileType::Name(Name::Monster),
    //         "art\\eye_candy\\" => MesFileType::Name(Name::EyeCandy),
    //         _ => return Err(DatRepoLoadingError::InvalidArtType)
    //     };
    //     Ok(mes_type)
    // }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MesCritterType {
    Base,
    Xp,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Name {
    Scenery,
    Interface,
    UniqueNpc,
    Monster,
    EyeCandy,
    Container,
    Light,
    Tile,
    Roof,
    Wall,
    WallProto,
    Structure,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Portrait {
    Game,
    User,
}
