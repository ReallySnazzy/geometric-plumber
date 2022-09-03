use raylib::{RaylibHandle, texture::Texture2D, RaylibThread};

pub struct LoadedAssets {
    pub pipe_texture: Texture2D
}

pub fn load_assets(rl: &mut RaylibHandle, thread: &RaylibThread) -> LoadedAssets {
    LoadedAssets { 
        pipe_texture: rl.load_texture(&thread, "assets/pipe.png").unwrap() 
    }
}