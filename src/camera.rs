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
}

pub enum Way {
    Forward(f32),
    Backward(f32),
    Left(f32),
    Right(f32),
}

impl Camera {
    pub fn view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn projection(&self) -> cgmath::Matrix4<f32> {
        cgmath::perspective(Deg(self.zoom), self.aspect_ratio, 0.1, 100.)
    }

    pub fn shift_zoom(&mut self, value: f32) {
        self.zoom = (self.zoom + value).max(ZOOM_MIN).min(ZOOM_MAX);
    }

    pub fn go(&mut self, value: Way) {
        match value {
            Way::Forward(speed) => self.position += self.front * speed,
            Way::Backward(speed) => self.position -= self.front * speed,
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

    pub fn build(&self) -> Camera {
        self.output.clone()
    }
}
