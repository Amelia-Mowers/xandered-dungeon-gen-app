mod display;
mod graph;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::graph::grid_transform::GridTransform;
use crate::graph::node::NodeBundle;
use crate::graph::connection::Connection;
use crate::graph::xander::*;
use crate::display::node::*;
use crate::display::connection::*;

fn main() {
    App::new()
    .add_plugins(( 
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        XanderPlugin,
        NodeDisplayPlugin,
        ConnectionDisplayPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, input)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    let init_node_positions = vec![
        (0, 0),
        (0, 1),
        // (1, 0),
        // (1, 1),
    ];

    for (x, y) in init_node_positions {
        commands.spawn(NodeBundle {
            position: GridTransform { x, y },
            ..Default::default()
        });
    }

    let init_connections = vec![
        ((0, 0), (0, 1)),
        // ((1, 0), (1, 1)),
        // ((0, 0), (1, 0)),
        // ((0, 0), (1, 1)),
    ];

    for ((xa, ya), (xb, yb)) in init_connections {
        commands.spawn(Connection {
            a: GridTransform::new(xa, ya),
            b: GridTransform::new(xb, yb),
        });
    }
}

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut xander: EventWriter<XanderEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        xander.send_default(); 
    }
}

