use cgmath::{prelude::InnerSpace, Angle, Deg, Matrix4, Point3, Vector3};

const ZOOM_MIN: f32 = 25.;
const ZOOM_MAX: f32 = 45.;
const PITCH_MIN: f32 = -89.;
const PITCH_MAX: f32 = 89.;

// type Camera = (Point3<f32>, Vector3<f32>, Vector3<f32>);

#[derive(Clone)]
pub struct Camera {
    position: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    zoom: f32,
    aspect_ratio: f32,
    yaw: f32,
    pitch: f32,
    fly: f32,
}

pub enum Way {
    Forward(f32),
    Backward(f32),
    Left(f32),
    Right(f32),
}

impl Camera {
    /// An attempt to implement Matrix4::look_at_rh
    #[rustfmt::skip]
    pub fn view(&self) -> cgmath::Matrix4<f32> {
        let front = Vector3::new(-self.front.x, -self.front.y, -self.front.z);
        let right = self.up.cross(front).normalize();
        let up = front.cross(right).normalize();
        let rotation = Matrix4::new(
            right.x, up.x, front.x, 0.,
            right.y, up.y, front.y, 0.,
            right.z, up.z, front.z, 0.,
            0., 0., 0., 1.,
        );

        let translation = Matrix4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            -self.position.x, -self.position.y, -self.position.z, 1.,
        );

        rotation * translation // swap to make 3rd person view
    }

    pub fn projection(&self) -> cgmath::Matrix4<f32> {
        cgmath::perspective(Deg(self.zoom), self.aspect_ratio, 0.1, 100.)
    }

    pub fn shift_zoom(&mut self, value: f32) {
        self.zoom = (self.zoom + value).max(ZOOM_MIN).min(ZOOM_MAX);
    }

    pub fn go(&mut self, value: Way) {
        let mut front = self.front;
        front.y = front.y * self.fly;
        match value {
            Way::Forward(speed) => self.position += front * speed,
            Way::Backward(speed) => self.position -= front * speed,
            Way::Left(speed) => self.position -= self.right_vector() * speed,
            Way::Right(speed) => self.position += self.right_vector() * speed,
        }
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32) {
        self.pitch = (self.pitch + pitch).min(PITCH_MAX).max(PITCH_MIN);
        self.yaw += yaw;
        self.front.x = Deg(self.yaw).cos() * Deg(self.pitch).cos();
        self.front.y = Deg(self.pitch).sin();
        self.front.z = Deg(self.yaw).sin() * Deg(self.pitch).cos();
        self.front = self.front.normalize();
    }

    fn right_vector(&self) -> Vector3<f32> {
        self.front.cross(self.up).normalize()
    }
}

pub struct CameraOptions {
    output: Camera,
}

impl CameraOptions {
    pub fn new() -> Self {
        Self {
            output: Camera {
                position: [0., 0., 0.].into(),
                front: [0., 0., 0.].into(),
                up: [0., 0., 0.].into(),
                zoom: 0.,
                aspect_ratio: 0.,
                yaw: 0.,
                pitch: 0.,
                fly: 1.,
            },
        }
    }

    pub fn position(&mut self, value: impl Into<Point3<f32>>) -> &mut Self {
        self.output.position = value.into();
        self
    }

    pub fn front(&mut self, value: impl Into<Vector3<f32>>) -> &mut Self {
        self.output.front = value.into();
        self
    }

    pub fn up(&mut self, value: impl Into<Vector3<f32>>) -> &mut Self {
        self.output.up = value.into();
        self
    }

    pub fn zoom(&mut self, value: f32) -> &mut Self {
        self.output.zoom = value;
        self
    }

    pub fn aspect_ratio(&mut self, value: f32) -> &mut Self {
        self.output.aspect_ratio = value;
        self
    }

    pub fn pitch(&mut self, value: f32) -> &mut Self {
        self.output.pitch = value;
        self
    }

    pub fn yaw(&mut self, value: f32) -> &mut Self {
        self.output.yaw = value;
        self
    }

    pub fn fly(&mut self, value: bool) -> &mut Self {
        self.output.fly = if value { 1. } else { 0. };
        self
    }

    pub fn build(&self) -> Camera {
        self.output.clone()
    }
}
