use process_path::get_executable_path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture,TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::rc::Rc;
use util::{Hasher,rect_part};
use tiled::{parse_file, Map};

pub struct Assets<'a> {
    textures: HashMap<Asset, Rc<Texture<'a>>, Hasher>,
    maps: HashMap<Asset, Rc<Map>, Hasher>,
    map_gids: HashMap<Asset, HashMap<u32, (Rc<Texture<'a>>, Rect), Hasher>, Hasher>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Assets<'a> {
        Assets {
            textures: HashMap::<_, _, _>::default(),
            maps: HashMap::<_, _, _>::default(),
            map_gids: HashMap::<_, _, _>::default(),
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
                        let mut max_gid = 0;
                        for layer in map.layers.iter() {
                            for tile_row in layer.tiles.iter() {
                                for gid in tile_row.iter() {
                                    if *gid > max_gid {
                                        max_gid = *gid;
                                    }
                                }
                            }
                        }
                        let tile_width = map.tilesets[0].tile_width;
                        let tile_height = map.tilesets[0].tile_height;
                        let mut map_gids = HashMap::<u32, (Rc<Texture<'a>>, Rect), Hasher>::default();
                        for i in 0..(max_gid + 1) {
                            match map.get_tileset_by_gid(i) {
                                Some(tileset) => {
                                    let relative_gid = i - tileset.first_gid;
                                    let texture = textures.get(&tileset.images[0].source).unwrap().clone();
                                    let tile_rect = rect_part(relative_gid, tile_width, tile_height, texture.query().width);
                                    map_gids.insert(i, (texture, tile_rect));
                                },
                                None => { },
                            }
                        }

                        self.maps.insert(asset.clone(), Rc::new(map));
                        self.map_gids.insert(asset.clone(), map_gids);
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

    pub fn get_map_gid(&self, asset: &Asset, gid: u32) -> &(Rc<Texture<'a>>, Rect) {
        self.map_gids.get(asset).unwrap().get(&gid).unwrap()
    }

    //pub fn get_map_gid(&self, asset: &Asset, gid: u64) -> (
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Asset {
    None,
    Test,
    Empty,
    TestMap
}
