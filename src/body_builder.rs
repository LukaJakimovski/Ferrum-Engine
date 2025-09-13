use glam::Vec2;
use crate::enums::{BodyType, ColorType};
use crate::spring::Spring;
use crate::{color, ColorRGBA, Rigidbody};

#[derive(Clone)]
pub struct RigidbodyParams {
    pub(crate) sides: u32,
    pub(crate) radius: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) pos: Vec2,
    pub(crate) mass: f32,
    pub(crate) restitution: f32,
    pub(crate) color: Option<ColorRGBA>,
    pub(crate) collides: bool,
    pub(crate) rotation: f32,
    pub(crate) angular_velocity: f32,
    pub(crate) velocity: Vec2,
    pub(crate) color_type: ColorType,
    pub(crate) gravity_multiplier: f32,
    pub(crate) eternal: bool,
}

#[derive(Clone, Default)]
pub struct SpringParams {
    pub(crate) stiffness: f32,
    pub(crate) dampening: f32,
    pub(crate) rest_length: f32,
    pub(crate) body_a: usize,
    pub(crate) body_b: usize,
    pub(crate) anchor_a: Vec2,
    pub(crate) anchor_b: Vec2,
}

#[derive(Clone)]
pub struct BodyBuilder {
    pub(crate) body_type: BodyType,
    pub(crate) rigidbody_params: RigidbodyParams,
    pub(crate) spring_params: SpringParams,
}

impl BodyBuilder {
    pub fn create_rigidbody(&self, palette: &Option<Vec<ColorRGBA>>) -> Rigidbody {
        let body_params = &self.rigidbody_params;
        let mut rigidbody: Rigidbody;
        let random_color;
        if palette.is_some() {
            random_color = ColorRGBA::random_from_palette(&palette.clone().unwrap());
        } else {
            random_color = ColorRGBA::random_hsl();
        }

        match self.body_type {
            BodyType::Rectangle => {
                if body_params.color.is_some() {
                    rigidbody = Rigidbody::rectangle(
                        body_params.width,
                        body_params.height,
                        body_params.pos,
                        body_params.mass,
                        body_params.restitution,
                        body_params.color.unwrap(),
                    )
                } else {
                    rigidbody = Rigidbody::rectangle(
                        body_params.width,
                        body_params.height,
                        body_params.pos,
                        body_params.mass,
                        body_params.restitution,
                        random_color,
                    )
                }
            }
            _ => {
                if body_params.color.is_some() {
                    rigidbody = Rigidbody::polygon(
                        body_params.sides,
                        body_params.radius,
                        body_params.pos,
                        body_params.mass,
                        body_params.restitution,
                        body_params.color.unwrap(),
                    )
                } else {
                    rigidbody = Rigidbody::polygon(
                        body_params.sides,
                        body_params.radius,
                        body_params.pos,
                        body_params.mass,
                        body_params.restitution,
                        random_color,
                    )
                }
            }
        };
        rigidbody.eternal = body_params.eternal;
        rigidbody.collision = body_params.collides;
        rigidbody.rotate(body_params.rotation);
        rigidbody.angular_velocity = body_params.angular_velocity;
        rigidbody.velocity = body_params.velocity;
        rigidbody.gravity_multiplier = body_params.gravity_multiplier;
        rigidbody
    }

    pub fn create_spring(&self, rigidbodies: &mut Vec<Rigidbody>) -> Spring {
        assert_ne!(self.body_type, BodyType::Spring, "Body Type is not spring");
        let spring_params = &self.spring_params;
        Spring::new(
            spring_params.body_a,
            spring_params.body_b,
            spring_params.anchor_a,
            spring_params.anchor_b,
            spring_params.rest_length,
            spring_params.stiffness,
            spring_params.dampening,
            rigidbodies,
        )
    }

    pub fn create_joint() -> Rigidbody {
        let mut rigidbody = Rigidbody::polygon(
            32,
            0.05,
            Vec2::ZERO,
            1.0,
            0.0,
            ColorRGBA::white(),
        );
        rigidbody.eternal = false;
        rigidbody.collision = false;
        rigidbody.gravity_multiplier = 0.0;
        rigidbody
    }
}
