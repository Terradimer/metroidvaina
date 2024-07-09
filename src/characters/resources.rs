use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerWalk {
    #[asset(
        paths(
            "p_walk/1.png",
            "p_walk/2.png",
            "p_walk/3.png",
            "p_walk/4.png",
            "p_walk/5.png",
            "p_walk/6.png",
            "p_walk/7.png",
            "p_walk/8.png"
        ),
        collection(typed)
    )]
    walking: Vec<Handle<Image>>,
}
