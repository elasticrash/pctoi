pub mod models;
use image::Rgb;
use image::RgbImage;
use models::{Camera, ExPnt, Point};
use std::cmp::Ordering;
use std::collections::HashMap;
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
    println!("{:?}", camera);
    let cloud: Vec<Point> = read_lines("cloud.xyzi")
        .expect("file to be there")
        .map(|line| Point::new(line.unwrap()))
        .collect();

    println!("no of points: {}", cloud.len());

    let mut real = project(cloud, &camera);
    real.sort_by(|a, b| match a.p_x.cmp(&b.p_x) {
        Ordering::Equal => a.p_y.cmp(&b.p_y),
        other => other,
    });

    let mut project = threshold(&real, &camera);
    save_image(project, &camera);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn save_image(pr: Vec<ExPnt>, camera: &Camera) {
    let mut image = RgbImage::new(camera.width, camera.height);

    let mut hm: HashMap<String, (ExPnt, f32)> = HashMap::new();

    for point in pr {
        let pdist = (
            point.clone(),
            distance(camera.x_o, camera.y_o, camera.z_o, point.r_x, point.r_y, point.r_z),
        );

        match hm.get_mut(&format!("{}-{}", point.p_x, point.p_y)) {
            Some(pnt) => {
                if pnt.1 > pdist.1 {
                    *hm.get_mut(&format!("{}-{}", point.p_x, point.p_y)).unwrap() = pdist;
                }
            }
            None => {
                hm.insert(format!("{}-{}", point.p_x, point.p_y), pdist);
            }
        }
    }

    for x in 0..camera.width {
        for y in 0..camera.height {
            match hm.get(&format!("{}-{}", x, y)) {
                Some((p, _)) => {
                    image.put_pixel(x, y, Rgb([p.int, p.int, p.int]));
                }
                None => {
                    image.put_pixel(x, y, Rgb([0, 0, 0]));
                }
            }
        }
    }

    let save_path = "output.png";
    image.save(save_path).expect("Failed to save image");
}

fn threshold(sort: &Vec<ExPnt>, camera: &Camera) -> Vec<ExPnt> {
    let mut projection: Vec<ExPnt> = Vec::new();

    let mut l = 0;
    while l < sort.len() - 1 {
        let mut g = 1;

        while l + g + 1 < sort.len()
            && sort[l + g].p_x == sort[l + g + 1].p_x
            && sort[l + g].p_y == sort[l + g + 1].p_y
        {
            g += 1;
        }

        let mut tempz = Vec::with_capacity(g);
        for k in 0..g {
            tempz.push(distance(
                camera.x_o,
                camera.y_o,
                camera.z_o,
                sort[l + k].r_x,
                sort[l + k].r_y,
                sort[l + k].r_z,
            ));
        }

        tempz.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for t in 0..g {
            if tempz[0] + 0.01
                > distance(
                    camera.x_o,
                    camera.y_o,
                    camera.z_o,
                    sort[l + t].r_x,
                    sort[l + t].r_y,
                    sort[l + t].r_z,
                )
            {
                projection.push(ExPnt {
                    r_x: sort[l + t].r_x,
                    r_y: sort[l + t].r_y,
                    r_z: sort[l + t].r_z,
                    int: sort[l + t].int,
                    p_x: sort[l + t].p_x,
                    p_y: sort[l + t].p_y,
                    p_z: 0,
                });
            }
        }
        l += g + 1;
    }

    projection
}

fn project(points: Vec<Point>, camera: &Camera) -> Vec<ExPnt> {
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
    let o: f32 = gtr(omega);
    let f: f32 = gtr(phi);
    let k: f32 = gtr(kapa);

    let xa: f32 = xo
        - c * ((x_a - x_o) * r11(f, k) + (y_a - y_o) * r21(f, k) + (z_a - z_o) * r31(f))
            / ((x_a - x_o) * r13(o, f, k) + (y_a - y_o) * r23(o, f, k) + (z_a - z_o) * r33(o, f));

    xa
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
    let o: f32 = gtr(omega);
    let f: f32 = gtr(phi);
    let k: f32 = gtr(kapa);

    let ya: f32 = yo
        - c * ((x_a - x_o) * r12(o, f, k) + (y_a - y_o) * r22(o, f, k) + (z_a - z_o) * r32(o, f))
            / ((x_a - x_o) * r13(o, f, k) + (y_a - y_o) * r23(o, f, k) + (z_a - z_o) * r33(o, f));
    ya
}

fn gtr(r: f32) -> f32 {
    (r / 200f32) * std::f32::consts::PI
}

fn r11(phi: f32, kapa: f32) -> f32 {
    phi.cos() * kapa.cos()
}

fn r12(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.cos() * kapa.sin() + omega.sin() * phi.sin() * kapa.cos()
}

fn r13(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.sin() * kapa.sin() - omega.cos() * phi.sin() * kapa.cos()
}

fn r21(phi: f32, kapa: f32) -> f32 {
    -phi.cos() * kapa.sin()
}

fn r22(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.cos() * kapa.cos() - omega.sin() * phi.sin() * kapa.sin()
}

fn r23(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.sin() * kapa.cos() + omega.cos() * phi.sin() * kapa.sin()
}

fn r31(phi: f32) -> f32 {
    phi.sin()
}

fn r32(omega: f32, phi: f32) -> f32 {
    -omega.sin() * phi.cos()
}

fn r33(omega: f32, phi: f32) -> f32 {
    omega.cos() * phi.cos()
}

fn distance(xa: f32, ya: f32, za: f32, xb: f32, yb: f32, zb: f32) -> f32 {
    ((xb - xa).powi(2) + (yb - ya).powi(2) + (zb - za).powi(2)).sqrt()
}
