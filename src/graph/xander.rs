use bevy::prelude::*;
use std::collections::HashSet;
use rand::seq::IteratorRandom;

use crate::graph::grid_transform::GridTransform;
use crate::graph::node::NodeBundle;
use crate::graph::node::Node;
use crate::graph::connection::Connection;

#[derive(Event, Default)]
pub struct XanderEvent;

pub struct XanderPlugin;

impl Plugin for XanderPlugin { 
    fn build (&self, app: &mut App) {
        app
            .add_event::<XanderEvent>()
            .add_systems(Update, xander);
    }
}

fn xander(
    mut commands: Commands,
    mut event: EventReader<XanderEvent>,
    query: Query<&GridTransform, With<Node>>
) {
    let mut rng = rand::thread_rng();
    for _ in event.read() {
        info!("Xander Event processing!");

        let mut nodes = HashSet::new();
        for pos in query.iter() {
            nodes.insert(*pos);
        }
        info!("nodes: {}", nodes.len());

        let mut neighbor_node_pairs = Vec::new();
        for node in &nodes {
            for neighbor_position in node.neighbors() {
                if nodes.contains(&neighbor_position) {
                    neighbor_node_pairs.push((node, neighbor_position));
                }
            }
        }
        info!("neighbor_node_pairs: {}", neighbor_node_pairs.len());

        let mut loop_candidates =  Vec::new();
        for (a, b) in neighbor_node_pairs {
            let mut a_empty_neighbors = Vec::new();
            for pos in a.neighbors() {
                if !nodes.contains(&pos) {
                    a_empty_neighbors.push(pos);
                }
            }

            let mut b_empty_neighbors = HashSet::new();
            for pos in b.neighbors() {
                if !nodes.contains(&pos) {
                    b_empty_neighbors.insert(pos);
                }
            }

            for an in a_empty_neighbors {
                for an2 in an.neighbors() {
                    if b_empty_neighbors.contains(&an2) {
                        loop_candidates.push((a, an, an2, b));
                    }
                }
            }
        }
        info!("loop_candidates: {}", loop_candidates.len());

        if let Some((a, b, c, d)) = loop_candidates.into_iter().choose(&mut rng) {
            commands.spawn(NodeBundle {
                position: b,
                ..Default::default()
            });
            commands.spawn(NodeBundle {
                position: c,
                ..Default::default()
            });

            commands.spawn(Connection {
                a: *a,
                b
            });
            commands.spawn(Connection {
                a: b,
                b: c,
            });
            commands.spawn(Connection {
                a: c,
                b: d,
            });
        }
    }
}
