use bevy::asset::LoadState;
/// let's use just load assets once, amigos
use bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetsState>() // necessary to tell if they've finished loading
            .init_resource::<SceneAssets>()
            // make sure this loads before the spaceship uses it - right now that is
            // handled by running this PreStartup and spaceship in Startup
            .add_systems(PreStartup, load_assets)
            .add_systems(
                Update,
                check_asset_loading.run_if(in_state(AssetsState::Loading)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AssetsState {
    #[default]
    Loading,
    Loaded,
}

// all the models are loaded via SceneBundle - the models
// can have multiple elements and scene makes all that possible
#[derive(Resource, Clone, Debug, Default)]
pub struct SceneAssets {
    pub missile:   Handle<Scene>,
    pub nateroid:  Handle<Scene>,
    pub spaceship: Handle<Scene>, // pub sphere: Handle<Scene>,
}

pub fn load_assets(
    //    mut commands: Commands,
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    *scene_assets = SceneAssets {
        missile:   asset_server.load("models/Bullets Pickup.glb#Scene0"),
        nateroid:  asset_server.load("models/donut.glb#Scene0"),
        spaceship: asset_server.load("models/Spaceship.glb#Scene0"),
    };
}

pub fn check_asset_loading(
    mut next_state: ResMut<NextState<AssetsState>>,
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneAssets>,
) {
    let missile_loaded =
        asset_server.get_load_state(scene_assets.missile.id()) == Some(LoadState::Loaded);
    let nateroid_loaded =
        asset_server.get_load_state(scene_assets.nateroid.id()) == Some(LoadState::Loaded);
    let spaceship_loaded =
        asset_server.get_load_state(scene_assets.spaceship.id()) == Some(LoadState::Loaded);

    if missile_loaded && nateroid_loaded && spaceship_loaded {
        // println!("All assets loaded, transitioning to Loaded state");
        next_state.set(AssetsState::Loaded);
    }
}
