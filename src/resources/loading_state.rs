use bevy::prelude::Resource;

#[derive(Resource)]
pub struct LoadingState {
    pub data_has_loaded: bool,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            data_has_loaded: false,
        }
    }
}
