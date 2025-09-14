use crate::spring::Spring;
use crate::world::Parameters;
use crate::{Rigidbody, World};
use egui_wgpu::wgpu;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::Window;
use crate::pivot_joint::PivotJoint;
use crate::weld_joint::WeldJoint;

pub struct App {
    world: Option<World>,
    polygons: Vec<Rigidbody>,
    springs: Vec<Spring>,
    weld_joints: Vec<WeldJoint>,
    pivot_joints: Vec<PivotJoint>,
    parameters: Parameters,

}

impl App {
    pub fn new(
        polygons: Vec<Rigidbody>,
        springs: Vec<Spring>,
        weld_joints: Vec<WeldJoint>,
        pivot_joints: Vec<PivotJoint>,
        parameters: Parameters,
    ) -> Self {
        Self {
            world: None,
            polygons,
            springs,
            weld_joints,
            pivot_joints,
            parameters,
        }
    }
}

impl ApplicationHandler<World> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.world = Some(
                pollster::block_on(World::new(
                    window,
                    self.polygons.clone(),
                    self.springs.clone(),
                    self.weld_joints.clone(),
                    self.pivot_joints.clone(),
                    self.parameters.clone(),
                )),
            );
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: World) {
        self.world = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = self.world.as_mut().unwrap();
        state
            .render.egui_renderer
            .handle_input(state.render.window.as_ref(), &event);

        let world = match &mut self.world {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => world.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                world.update();
                match world.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = world.render.window.inner_size();
                        world.resize(size.width, size.height);
                    }
                    Err(e) => {
                        eprintln!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                world.ui.handle_mouse_input(state, button, &mut world.physics, &mut world.color_system)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => if  !world.ui.is_pointer_used {world.ui.handle_key(&mut world.physics, &mut world.color_system, &mut world.parameters, event_loop, code, key_state.is_pressed())},
            WindowEvent::MouseWheel { delta, .. } => world.ui.handle_scroll(delta),
            WindowEvent::CursorMoved { position, .. } => world.ui.handle_cursor_movement(position),
            _ => {}
        }
    }
}

pub fn run(
    rigidbodys: Vec<Rigidbody>,
    springs: Vec<Spring>,
    weld_joints: Vec<WeldJoint>,
    pivot_joints: Vec<PivotJoint>,
    parameters: Parameters,
) {
    let event_loop = EventLoop::with_user_event().build().expect("Unable to create event loop");
    let mut app = App::new(
        rigidbodys,
        springs,
        weld_joints,
        pivot_joints,
        parameters,
    );
    event_loop.run_app(&mut app).expect("Error");
}