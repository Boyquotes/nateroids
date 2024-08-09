use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{
        GravityScale,
        LockedAxes,
    },
    geometry::ActiveEvents,
    prelude::{
        CoefficientCombineRule,
        Collider,
        ColliderMassProperties,
        ColliderMassProperties::Mass,
        CollisionGroups,
        Restitution,
        RigidBody,
        Velocity,
    },
};

use crate::{
    boundary::Boundary,
    schedule::InGameSet,
};

const DEFAULT_GRAVITY: f32 = 0.0;
const DEFAULT_MASS: f32 = 1.0;

#[derive(Component, Debug, Default)]
pub struct Teleporter {
    pub just_teleported:          bool,
    pub last_teleported_position: Option<Vec3>,
    pub last_teleported_normal:   Option<Dir3>,
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
    pub active_events:    ActiveEvents,
    pub collider:         Collider,
    pub collision_groups: CollisionGroups,
    pub gravity_scale:    GravityScale,
    pub locked_axes:      LockedAxes,
    pub mass:             ColliderMassProperties,
    pub model:            SceneBundle,
    pub restitution:      Restitution,
    pub rigidity:         RigidBody,
    pub velocity:         Velocity,
    pub wrappable:        Teleporter,
}

impl Default for MovingObjectBundle {
    fn default() -> Self {
        Self {
            active_events:    ActiveEvents::COLLISION_EVENTS,
            collider:         Collider::default(),
            collision_groups: CollisionGroups::default(),
            gravity_scale:    GravityScale(DEFAULT_GRAVITY),
            locked_axes:      LockedAxes::TRANSLATION_LOCKED_Z,
            mass:             Mass(DEFAULT_MASS),
            model:            SceneBundle::default(),
            restitution:      Restitution {
                coefficient:  1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            rigidity:         RigidBody::Dynamic,
            velocity:         Velocity::default(),
            wrappable:        Teleporter::default(),
        }
    }
}

fn teleport_at_boundary(
    boundary: Res<Boundary>,
    mut wrappable_entities: Query<(&mut Transform, &mut Teleporter)>,
) {
    for (mut transform, mut wrappable) in wrappable_entities.iter_mut() {
        let original_position = transform.translation;
        let teleported_position =
            calculate_teleport_position(original_position, &boundary.transform);
        if teleported_position != original_position {
            transform.translation = teleported_position;
            wrappable.just_teleported = true;
            wrappable.last_teleported_position = Some(teleported_position);
            wrappable.last_teleported_normal =
                Some(boundary.get_normal_for_position(teleported_position));
        } else {
            wrappable.just_teleported = false;
            wrappable.last_teleported_position = None;
            wrappable.last_teleported_normal = None;
        }
    }
}

/// given a particular point, what is the point on the opposite side of the
/// boundary?
pub fn calculate_teleport_position(position: Vec3, transform: &Transform) -> Vec3 {
    let boundary_min = transform.translation - transform.scale / 2.0;
    let boundary_max = transform.translation + transform.scale / 2.0;

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
