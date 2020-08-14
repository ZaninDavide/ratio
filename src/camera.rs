pub struct PerspectiveCamera {
    pos: [f32; 3],
    fov: f32,
    near: f32,
    far: f32,
    aspect_ratio: f32,
}

impl PerspectiveCamera {
    pub fn new(pos: [f32; 3]) -> PerspectiveCamera {
        PerspectiveCamera {
            pos: pos,
            fov: 50.0 * 3.14 / 180.0,
            near: 0.05,
            far: 1000.0,
            aspect_ratio: 1.0,
        }
    }

    /// View-projection matrix for this PerspectiveCamera in collumn-major order.
    pub fn matrix(&self) -> [f32; 16] {
        let a = (self.fov / 2.0).tan() * self.aspect_ratio;
        let b = (self.fov / 2.0).tan();
        let c = self.far / (self.far - self.near);
        let d = -self.far * self.near / (self.far - self.near);
        // view propjection matrix
        [
            1.0 / a,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / b,
            0.0,
            0.0,
            0.0,
            0.0,
            c,
            1.0,
            -self.pos[0] / a,
            -self.pos[1] / b,
            -self.pos[2] * c + d,
            -self.pos[2],
        ]
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.pos = pos;
    }

    pub fn shift_position(&mut self, shift: [f32; 3]) {
        self.pos[0] += shift[0];
        self.pos[1] += shift[1];
        self.pos[2] += shift[2];
    }
}

impl std::fmt::Display for PerspectiveCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Position: [{}, {}, {}], Field of View: {}, Aspect Ratio: {}",
            self.pos[0], self.pos[1], self.pos[2], self.fov, self.aspect_ratio
        )
    }
}
