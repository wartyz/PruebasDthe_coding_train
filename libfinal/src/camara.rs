use crate::matem::{pvector3, PVector3, sub_s};
use crate::transform3d::{identity4x4, Matrix4x4};
use crate::transform::mul_escalar;

pub struct Camera {
    pub eye: PVector3,
    pub center: PVector3,
    pub up: PVector3,
}

impl Camera {
    pub fn new0() -> Camera {
        Camera::new3(
            pvector3(0.0, 0.0, -5.0),
            pvector3(0.0, 0.0, 0.0),
            pvector3(0.0, 1.0, 0.0),
        )
    }
    pub fn new3(eye: PVector3, center: PVector3, up: PVector3) -> Camera {
        Camera { eye, center, up }
    }

    #[rustfmt::skip]
    pub fn view_matrix(&self, eye: PVector3, target: PVector3, up: PVector3) -> Matrix4x4 {
        let mut forward = sub_s(&target, &eye);
        forward.normalize();
        // cross es producto vectorial
        //let side = (cross(forward, up)).normalize();
        let side = forward * up;

        //let new_up = (side * forward);
        let new_up = side * forward;

        //let mut view = [[0.0; 4]; 4];
        // view[0][0] = side[0];
        // view[1][0] = side[1];
        // view[2][0] = side[2];
        // view[3][0] = -dot(side, eye);
        //
        // view[0][1] = new_up[0];
        // view[1][1] = new_up[1];
        // view[2][1] = new_up[2];
        // view[3][1] = -dot(new_up, eye);
        //
        // view[0][2] = -forward[0];
        // view[1][2] = -forward[1];
        // view[2][2] = -forward[2];
        // view[3][2] = dot(forward, eye);
        //
        // view[3][3] = 1.0;

        let view = Matrix4x4 {
            data: [side.x, side.y, side.w, -mul_escalar(side, eye),
                new_up.x, new_up.y, new_up.w, -mul_escalar(new_up, eye),
                -forward.x, -forward.y, -forward.w, mul_escalar(forward, eye),
                0.0, 0.0, 0.0, 1.0]
        };

        view

        //Mat4::look_at_rh(self.eye, self.center, self.up)
        //identity4x4()
    }
}


