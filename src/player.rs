use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query.iter_mut() {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.rotation = Quat::default();
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            transform.translation.x -= movement_amount;
        }
    }
}