use godot::prelude::*;
use godot::{
    classes::{INode3D, Input, Node3D},
    global::Key,
    obj::{Base, Singleton},
    register::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct Player {
    base: Base<Node3D>,

    #[export]
    player_speed: f32,
}

#[godot_api]
impl INode3D for Player {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            player_speed: 25.,
        }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut direction = Vector3::ZERO;

        if input.is_action_pressed("ui_left") || input.is_key_pressed(Key::A) {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("ui_right") || input.is_key_pressed(Key::D) {
            direction.x += 1.0;
        }
        if input.is_action_pressed("ui_up") || input.is_key_pressed(Key::W) {
            direction.z -= 1.0;
        }
        if input.is_action_pressed("ui_down") || input.is_key_pressed(Key::S) {
            direction.z += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalized();
            let movement = direction * self.player_speed * delta as f32;
            let old_pos = self.base().get_position();
            let new_pos = old_pos + movement;
            self.base_mut().set_position(new_pos);
        }
    }
}
