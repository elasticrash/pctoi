mod helper;
mod image_utils;
mod models;
mod rotations;
mod vision;

use crate::helper::distance;
use crate::image_utils::save_image;
use crate::models::{Camera, ExPnt, Point};
use crate::vision::project;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !Path::new(&args[1]).exists() {
        println!("File does not exist, please provide the correct path");
        return;
    }

    let camera: Camera = match read_lines("orientation").expect("file to be there").next() {
        Some(Ok(line)) => Camera::new(line),
        _ => {
            panic!("Error reading camera orientation");
        }
    };
    println!("{}", camera);
    let cloud: Vec<Point> = read_lines(args[1].clone())
        .expect("file to be there")
        .map(|line| Point::new(line.unwrap()))
        .collect();

    println!("no of points: {}", cloud.len());

    let mut real = project(cloud, &camera);
    real.sort_by(|a, b| match a.p_x.cmp(&b.p_x) {
        Ordering::Equal => a.p_y.cmp(&b.p_y),
        other => other,
    });

    let project = threshold(&real, &camera);
    save_image(project, &camera);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
                sort[l + k].x,
                sort[l + k].y,
                sort[l + k].z,
            ));
        }

        tempz.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for t in 0..g {
            if tempz[0] + 0.01
                > distance(
                    camera.x_o,
                    camera.y_o,
                    camera.z_o,
                    sort[l + t].x,
                    sort[l + t].y,
                    sort[l + t].z,
                )
            {
                projection.push(ExPnt {
                    x: sort[l + t].x,
                    y: sort[l + t].y,
                    z: sort[l + t].z,
                    ptype: sort[l + t].ptype,
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
