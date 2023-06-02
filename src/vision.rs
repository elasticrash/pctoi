use crate::helper::gtr;
use crate::models::{Camera, ExPnt, Point};
use crate::rotations::{r11, r12, r13, r21, r22, r23, r31, r32, r33};

pub fn project(points: Vec<Point>, camera: &Camera) -> Vec<ExPnt> {
    let mut projected: Vec<ExPnt> = Vec::new();
    for point in points {
        let xa: f32 = collinearity_x(
            camera.xo,
            camera.c,
            point.x,
            camera.x_o,
            point.y,
            camera.y_o,
            point.z,
            camera.z_o,
            camera.omega,
            camera.phi,
            camera.kappa,
        );
        let ya: f32 = collinearity_y(
            camera.yo,
            camera.c,
            point.x,
            camera.x_o,
            point.y,
            camera.y_o,
            point.z,
            camera.z_o,
            camera.omega,
            camera.phi,
            camera.kappa,
        );

        projected.push(ExPnt {
            x: point.x,
            y: point.y,
            z: point.z,
            ptype: point.ptype,
            p_x: (xa + camera.width as f32) as i32,
            p_y: (-ya + camera.height as f32) as i32,
            p_z: 1,
        });
    }
    projected
}

pub fn collinearity_x(
    xo: f32,
    c: f32,
    x_a: f32,
    x_o: f32,
    y_a: f32,
    y_o: f32,
    z_a: f32,
    z_o: f32,
    omega: f32,
    phi: f32,
    kapa: f32,
) -> f32 {
    let o: f32 = gtr(omega);
    let f: f32 = gtr(phi);
    let k: f32 = gtr(kapa);

    let xa: f32 = xo
        - c * ((x_a - x_o) * r11(f, k) + (y_a - y_o) * r21(f, k) + (z_a - z_o) * r31(f))
            / ((x_a - x_o) * r13(o, f, k) + (y_a - y_o) * r23(o, f, k) + (z_a - z_o) * r33(o, f));

    xa
}

pub fn collinearity_y(
    yo: f32,
    c: f32,
    x_a: f32,
    x_o: f32,
    y_a: f32,
    y_o: f32,
    z_a: f32,
    z_o: f32,
    omega: f32,
    phi: f32,
    kapa: f32,
) -> f32 {
    let o: f32 = gtr(omega);
    let f: f32 = gtr(phi);
    let k: f32 = gtr(kapa);

    let ya: f32 = yo
        - c * ((x_a - x_o) * r12(o, f, k) + (y_a - y_o) * r22(o, f, k) + (z_a - z_o) * r32(o, f))
            / ((x_a - x_o) * r13(o, f, k) + (y_a - y_o) * r23(o, f, k) + (z_a - z_o) * r33(o, f));
    ya
}
