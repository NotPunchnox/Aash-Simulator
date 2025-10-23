extern crate kiss3d;
use rand::random;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};
use kiss3d::event::{Action, Key, WindowEvent};

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Kiss3d: Cube Control");
    window.set_background_color(0.9, 0.9, 0.9);

    let mut cube = window.add_cube(0.2, 0.2, 0.2);
    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    // UI state
    let mut rotation_speed = 0.014f32;
    let mut cube_color = [1.0, 0.0, 0.0];

    // Render loop
    while window.render().await {
        // Handle keyboard input
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Up, Action::Press, _) => {
                    rotation_speed = (rotation_speed + 0.005).min(0.1);
                }
                WindowEvent::Key(Key::Down, Action::Press, _) => {
                    rotation_speed = (rotation_speed - 0.005).max(0.0);
                }
                WindowEvent::Key(Key::R, Action::Press, _) => {
                    cube_color = [random(), random(), random()];
                }
                _ => {}
            }
        }

        // Rotate cube
        let rot_current = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation_speed);
        cube.prepend_to_local_rotation(&rot_current);

        // Update cube color
        cube.set_color(cube_color[0], cube_color[1], cube_color[2]);
    }
}