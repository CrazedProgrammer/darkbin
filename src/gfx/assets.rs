extern crate walkdir;
extern crate process_path;

use sdl2::render::Texture;
use std::collections::HashMap;
use sdl2::render::{Canvas};
use sdl2::video::{WindowContext};
use sdl2::render::TextureCreator;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;
use self::walkdir::WalkDir;
use self::process_path::get_executable_path;
use sdl2::image::LoadTexture;
use std::rc::Rc;

pub struct Assets<'a> {
    textures: HashMap<String, Rc<Texture<'a>>>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Assets<'a> {
        Assets {
            textures: HashMap::new(),
        }
    }

    pub fn load_all(&mut self, creator: &'a mut TextureCreator<WindowContext>) {
        let mut assets_dir_pathbuf = PathBuf::from(get_executable_path().unwrap().parent().unwrap().parent().unwrap().parent().unwrap());
        assets_dir_pathbuf.push("assets");
        let assets_dir = assets_dir_pathbuf.as_path();
        for entry in WalkDir::new(assets_dir) {
            let entry_if = entry.unwrap();
            let path = entry_if.path();
            // definitely clean this up someday
            let mut asset_str = "".to_string();
            {
                let path_str = path.to_str().unwrap().to_string();
                let asset_str_parts = path_str.split("assets").collect::<Vec<_>>();
                let asset_str_temp = asset_str_parts.last().unwrap_or(&"").to_string();
                if asset_str_temp == "" {
                    asset_str = "".to_string();
                } else {
                    asset_str = asset_str_temp.chars().rev().take(asset_str_temp.chars().count() - 1).collect::<String>().chars().rev().collect();
                }
                println!("{}: {}", path.to_str().unwrap(), asset_str);
            }

            if path.is_file() {
                match path.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("") {
                    "png" => {
                        self.textures.insert(asset_str, Rc::new(creator.load_texture(path).unwrap()));
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn get_texture(&self, path: &String) -> Rc<Texture<'a>> {
        self.textures.get(path).unwrap().clone()
    }
}
