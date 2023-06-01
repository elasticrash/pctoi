pub mod models;
use models::{Camera, ExPnt, Point};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let camera: Camera = match read_lines("orientation").expect("file to be there").next() {
        Some(Ok(line)) => Camera::new(line),
        _ => {
            panic!("Error reading camera orientation");
        }
    };

    let cloud: Vec<Point> = read_lines("cloud.xyzi")
        .expect("file to be there")
        .map(|line| Point::new(line.unwrap()))
        .collect();

    println!("no of points: {}", cloud.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn project(points: Vec<Point>, camera: Camera) -> Vec<ExPnt> {
    let mut projected: Vec<ExPnt> = Vec::new();
    for point in points {
        let xa: f32 = collinearity(
            camera.xo,
            camera.c,
            point.r_x,
            camera.x_o,
            point.r_y,
            camera.y_o,
            point.r_z,
            camera.z_o,
            camera.omega,
            camera.phi,
            camera.kappa,
        );
        let ya: f32 = collinearityy(
            camera.yo,
            camera.c,
            point.r_x,
            camera.x_o,
            point.r_y,
            camera.y_o,
            point.r_z,
            camera.z_o,
            camera.omega,
            camera.phi,
            camera.kappa,
        );

        projected.push(ExPnt {
            r_x: point.r_x,
            r_y: point.r_y,
            r_z: point.r_z,
            int: point.int,
            p_x: (xa + camera.width as f32) as i32,
            p_y: (-ya + camera.height as f32) as i32,
            p_z: 1,
        });
    }
    projected
}

fn collinearity(
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
    o: f32 = gtr(omega);
    f: f32 = gtr(phi);
    k: f32 = gtr(kapa);


    0.0
}

fn collinearityy(
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
    0.0
}

fn gtr(r: f32) -> f32 {
    (r / 200f32) * std::f32::consts::PI
}


fn r11(phi:f32, kapa:f32) -> f32 {
    phi.cos() * kapa.cos()
}

fn r12(omega:f32, phi:f32, kapa:f32) -> f32 {
    omega.cos() * kapa.sin() + omega.sin() * phi.sin() * kapa.cos()
}

fn r13(omega:f32, phi:f32, kapa:f32) -> f32 {
    omega.sin() * kapa.sin() - omega.cos() * phi.sin() * kapa.cos()
}

fn r21(phi:f32, kapa:f32) -> f32 {
    -phi.cos() * kapa.sin()
}


