use godot::classes::{
    CharacterBody3D, Engine, ICharacterBody3D, InputEvent, InputEventMouseMotion,
};
use godot::global::{clamp, deg_to_rad, move_toward, Key};
use godot::prelude::*;

use crate::managers::global_settings::GlobalSettings;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct PlayerCharacter {
    #[export]
    jump_force: f32,

    #[export]
    speed: f32,

    #[export]
    cam_handler: Option<Gd<Node3D>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for PlayerCharacter {
    fn ready(&mut self) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(mouse_input) = event.try_cast::<InputEventMouseMotion>().ok() {
            self.base_mut().rotate_y(
                -mouse_input.get_relative().x
                    * Engine::singleton()
                        .get_singleton(&StringName::from("GlobalSettings"))
                        .unwrap()
                        .cast::<GlobalSettings>()
                        .bind_mut()
                        .sens as f32
                    * 0.0001,
            );

            self.cam_handler.clone().unwrap().rotate_x(
                -mouse_input.get_relative().y
                    * Engine::singleton()
                        .get_singleton(&StringName::from("GlobalSettings"))
                        .unwrap()
                        .cast::<GlobalSettings>()
                        .bind_mut()
                        .sens as f32
                    * 0.0001,
            );

            let mut cam_rot: Vector3 = self.cam_handler.clone().unwrap().get_rotation();

            cam_rot.x = clamp(
                &cam_rot.x.to_variant(),
                &deg_to_rad(-90.0).to_variant(),
                &deg_to_rad(90.0).to_variant(),
            )
            .try_to::<f32>()
            .unwrap();
            self.cam_handler.clone().unwrap().set_rotation(cam_rot);
        }
    }

    fn process(&mut self, _delta: f64) {
        let mut velocity: Vector3 = self.base().get_velocity();

        if !self.base().is_on_floor() {
            velocity += self.base().get_gravity() * (_delta as f32);
        }

        if Input::singleton().is_key_pressed(Key::SPACE) && self.base().is_on_floor() {
            velocity.y = self.jump_force;
        }

        let input_dir = Input::singleton().get_vector("left", "right", "forw", "back");

        let dir = (self.base().get_transform().basis * Vector3::new(input_dir.x, 0.0, input_dir.y))
            .try_normalized();

        if !dir.is_none() {
            velocity.x = dir.unwrap().x * self.speed;
            velocity.z = dir.unwrap().z * self.speed;
        } else {
            velocity.x = move_toward(velocity.x.into(), 0.0, self.speed.into()) as f32;
            velocity.z = move_toward(velocity.z.into(), 0.0, self.speed.into()) as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
