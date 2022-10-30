use bevy::prelude::*;
use bevy::window::WindowResized;

//Component to tag a marker
#[derive(Component)]
struct Marker;

fn main() {
    App::new()
        //add a WindowDescriptor resource. This will be used to set initial
        //height / width of window, as well as keep track of information and
        //data about the window
        .insert_resource(
            WindowDescriptor {
                width:400.0,
                height:400.0,
                title:"Window Resize Example".to_string(),
                ..default()
            }
        )
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        //initial setup
        .add_startup_system(setup)
        //window resize event listener
        .add_system(on_window_resize)
        //update position of marker
        .add_system(update_marker)
        .run();
}

//help function that finds the bottom right coordinates of the window
fn find_bottom_right(window : &WindowDescriptor) -> Vec3 {
    Vec3::new(window.width / 2.0, window.height / -2.0, 0.0)
}

fn setup(mut commands:Commands, window:Res<WindowDescriptor>) {
    commands.spawn_bundle(Camera2dBundle::default());
   
    //Added a marker that sits in the bottom right of the window
    commands.spawn()
        .insert(Marker)
        .insert_bundle(SpriteBundle {
            sprite:Sprite {
                color: Color::MAROON,
                custom_size:Some(Vec2::new(40.0, 40.0)),

                //Since we are anchoring to bottom right, lets set anchor
                //point also to bottom right, so we dont have to offset the
                //coordinates
                anchor:bevy::sprite::Anchor::BottomRight,
                ..default()
            },
            transform:Transform { 
                translation: find_bottom_right(&window),
                ..default()
            },
            ..default()
        });
}

//system that updates the position of the marker
fn update_marker(
        mut query:Query<&mut Transform, With<Marker>>,
        window:Res<WindowDescriptor>
    ) {

    //If window size hasn't changed, then we can just exit out
    if !window.is_changed() {
        return;
    }


    //window resource has changed, so lets update the position
    for mut transform in &mut query {
        transform.translation = find_bottom_right(&window);
    }
}

//listener for window resize event
//we store the window info in the WindowDescriptor resource
fn on_window_resize(
    mut window: ResMut<WindowDescriptor>,
    mut resize_events: EventReader<WindowResized>) {

    for e in resize_events.iter() {
        window.height = e.height;
        window.width = e.width;
    } 
}

