use serde_derive::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub camera: Camera,
    pub image: Image,
    pub position: Position,
}

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub c: f32,
    pub xo: f32,
    pub yo: f32,
    pub omega: f32,
    pub phi: f32,
    pub kappa: f32,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x_o: f32,
    pub y_o: f32,
    pub z_o: f32,
}
#[derive(Debug, Deserialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum PointType {
    PointInt(u8),
    PointRGB(u8, u8, u8),
    Point,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub ptype: PointType,
}

impl Point {
    pub fn new(line: String) -> Point {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let x = parts[0].parse::<f32>().unwrap();
        let y = parts[1].parse::<f32>().unwrap();
        let z = parts[2].parse::<f32>().unwrap();

        if parts.len() == 4 {
            let int = parts[3].parse::<u8>().unwrap();
            Point {
                x,
                y,
                z,
                ptype: PointType::PointInt(int),
            }
        } else if parts.len() == 6 {
            let r = parts[3].parse::<u8>().unwrap();
            let g = parts[4].parse::<u8>().unwrap();
            let b = parts[5].parse::<u8>().unwrap();

            Point {
                x,
                y,
                z,
                ptype: PointType::PointRGB(r, g, b),
            }
        } else {
            Point {
                x,
                y,
                z,
                ptype: PointType::Point,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExPnt {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub ptype: PointType,
    pub p_x: i32,
    pub p_y: i32,
    pub p_z: i32,
}
