use process_path::get_executable_path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture,TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::rc::Rc;
use util::Hasher;
use tiled::{parse_file, Map};

pub struct Assets<'a> {
    textures: HashMap<Asset, Rc<Texture<'a>>, Hasher>,
    maps: HashMap<Asset, Rc<Map>, Hasher>,
    map_textures: HashMap<Asset, HashMap<String, Rc<Texture<'a>>, Hasher>, Hasher>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Assets<'a> {
        Assets {
            textures: HashMap::<_, _, _>::default(),
            maps: HashMap::<_, _, _>::default(),
            map_textures: HashMap::<_, _, _>::default(),
        }
    }

    pub fn load_all(&mut self, creator: &'a mut TextureCreator<WindowContext>) {
        let asset_files = map!{
            Asset::None => "NONE",
            Asset::Empty => "empty.png",
            Asset::Test => "test.png",
            Asset::TestMap => "testmap/testmap.tmx"
        };

        let mut assets_dir_pathbuf = PathBuf::from(get_executable_path().unwrap().parent().unwrap().parent().unwrap().parent().unwrap());
        assets_dir_pathbuf.push("assets");

        for (asset, asset_file) in asset_files.iter() {
            if asset == &Asset::None {
                continue;
            }
            let mut asset_pathbuf = assets_dir_pathbuf.clone();
            asset_pathbuf.push(asset_file);
            let asset_path = asset_pathbuf.as_path();

            if !asset_path.exists() {
                panic!("Asset {} doesn't exist.", asset_file);
            } else if asset_path.is_file() {
                match asset_path.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("") {
                    "png" | "jpg" => {
                        println!("Loading texture {}", asset_file);
                        self.textures.insert(asset.clone(), Rc::new(creator.load_texture(asset_path).unwrap()));
                    },
                    "tmx" => {
                        println!("Loading map {}", asset_file);
                        let map = parse_file(asset_path).unwrap();
                        let mut textures = HashMap::<String, Rc<Texture<'a>>, Hasher>::default();
                        for tileset in map.tilesets.iter() {
                            for image in tileset.images.iter() {
                                let mut texture_pathbuf = PathBuf::from(asset_path.parent().unwrap());
                                texture_pathbuf.push(image.source.clone());
                                println!("Loading tileset image: {}", image.source);
                                textures.insert(image.source.clone(), Rc::new(creator.load_texture(texture_pathbuf.as_path()).unwrap()));
                            }
                        }
                        self.map_textures.insert(asset.clone(), textures);
                        self.maps.insert(asset.clone(), Rc::new(map));
                    }
                    _ => {
                        panic!("Unrecognised extension for {}.", asset_file);
                    },
                }
            } else {
                panic!("Asset {} is a directory.", asset_file);
            }
        }
    }

    pub fn get_texture(&self, asset: &Asset) -> &Rc<Texture<'a>> {
        self.textures.get(asset).unwrap()
    }

    pub fn get_map(&self, asset: &Asset) -> &Rc<Map> {
        self.maps.get(asset).unwrap()
    }

    pub fn get_map_texture(&self, asset: &Asset, source: &String) -> &Rc<Texture<'a>> {
        self.map_textures.get(asset).unwrap().get(source).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Asset {
    None,
    Test,
    Empty,
    TestMap
}
