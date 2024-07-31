use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{GravityScale, LockedAxes},
    geometry::ActiveEvents,
    prelude::{
        CoefficientCombineRule, Collider, ColliderMassProperties, ColliderMassProperties::Mass,
        CollisionGroups, Restitution, RigidBody, Velocity,
    },
};

use crate::boundary::Boundary;
use crate::schedule::InGameSet;

const DEFAULT_GRAVITY: f32 = 0.0;
const DEFAULT_MASS: f32 = 1.0;

#[derive(Component, Debug, Default)]
pub struct Wrappable {
    pub wrapped: bool,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            teleport_at_boundary.in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub active_events: ActiveEvents,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub gravity_scale: GravityScale,
    pub locked_axes: LockedAxes,
    pub mass: ColliderMassProperties,
    pub model: SceneBundle,
    pub restitution: Restitution,
    pub rigidity: RigidBody,
    pub velocity: Velocity,
    pub wrappable: Wrappable,
}

impl Default for MovingObjectBundle {
    fn default() -> Self {
        Self {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::default(),
            collision_groups: CollisionGroups::default(),
            gravity_scale: GravityScale(DEFAULT_GRAVITY),
            locked_axes: LockedAxes::TRANSLATION_LOCKED_Z,
            mass: Mass(DEFAULT_MASS),
            model: SceneBundle::default(),
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            rigidity: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec3::ZERO,
                angvel: Default::default(),
            },
            wrappable: Wrappable::default(),
        }
    }
}

// fn teleport_at_boundary(
//     viewport: Res<ViewportWorldDimensions>,
//     mut wrappable_entities: Query<(&mut Transform, &mut Wrappable)>,
// ) {
//     for (mut transform, mut wrappable) in wrappable_entities.iter_mut() {
//         let original_position = transform.translation;
//         let wrapped_position = calculate_teleport_position(original_position, &viewport);
//         if wrapped_position != original_position {
//             wrappable.wrapped = true;
//             transform.translation = wrapped_position;
//         } else {
//             wrappable.wrapped = false;
//         }
//     }
// }
fn teleport_at_boundary(
    boundary: Res<Boundary>,
    mut wrappable_entities: Query<(&mut Transform, &mut Wrappable)>,
) {
    for (mut transform, mut wrappable) in wrappable_entities.iter_mut() {
        let original_position = transform.translation;
        let wrapped_position = calculate_teleport_position(original_position, &boundary);
        if wrapped_position != original_position {
            wrappable.wrapped = true;
            transform.translation = wrapped_position;
        } else {
            wrappable.wrapped = false;
        }
    }
}

/// given a particular point, what is the point on the opposite side of the screen?
// pub fn calculate_teleport_position(
//     position: Vec3,
//     dimensions: &Res<ViewportWorldDimensions>,
// ) -> Vec3 {
//     let width = dimensions.width;
//     let height = dimensions.height;
//
//     let viewport_right = width / 2.0;
//     let viewport_left = -viewport_right;
//     let viewport_top = height / 2.0;
//     let viewport_bottom = -viewport_top;
//
//     let mut wrapped_position = position;
//
//     if position.x >= viewport_right {
//         wrapped_position.x = viewport_left;
//     } else if position.x <= viewport_left {
//         wrapped_position.x = viewport_right;
//     }
//
//     if position.y >= viewport_top {
//         wrapped_position.y = viewport_bottom;
//     } else if position.y <= viewport_bottom {
//         wrapped_position.y = viewport_top;
//     }
//
//     wrapped_position
// }

pub fn calculate_teleport_position(position: Vec3, boundary: &Res<Boundary>) -> Vec3 {
    let boundary_min = boundary.transform.translation - boundary.transform.scale / 2.0;
    let boundary_max = boundary.transform.translation + boundary.transform.scale / 2.0;

    let mut wrapped_position = position;

    if position.x >= boundary_max.x {
        wrapped_position.x = boundary_min.x;
    } else if position.x <= boundary_min.x {
        wrapped_position.x = boundary_max.x;
    }

    if position.y >= boundary_max.y {
        wrapped_position.y = boundary_min.y;
    } else if position.y <= boundary_min.y {
        wrapped_position.y = boundary_max.y;
    }

    if position.z >= boundary_max.z {
        wrapped_position.z = boundary_min.z;
    } else if position.z <= boundary_min.z {
        wrapped_position.z = boundary_max.z;
    }

    wrapped_position
}
