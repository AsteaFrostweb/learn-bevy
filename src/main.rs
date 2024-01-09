use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup_system.system())
        .run();
}

fn setup_system(commands: &mut Commands, asset_server: Res<AssetServer>, mut shaders: ResMut<Assets<Shader>>) {
    let vertex_shader = asset_server.load::<Shader, _>("path_to_vertex_shader.vert");
    let fragment_shader = asset_server.load::<Shader, _>("path_to_fragment_shader.frag");

    commands.insert_resource(ShaderStages {
        vertex: vertex_shader,
        fragment: Some(fragment_shader),
    });
}