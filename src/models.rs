use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Camera {
    pub c: f32,
    pub xo: f32,
    pub yo: f32,
    pub width: u32,
    pub height: u32,
    pub x_o: f32,
    pub y_o: f32,
    pub z_o: f32,
    pub omega: f32,
    pub phi: f32,
    pub kappa: f32,
}

impl Camera {
    pub fn new(orientation: String) -> Self {
        let parts = orientation.split_whitespace().collect::<Vec<&str>>();
        let c = parts[0].parse::<f32>().unwrap();
        let xo = parts[1].parse::<f32>().unwrap();
        let yo = parts[2].parse::<f32>().unwrap();
        let width = parts[3].parse::<u32>().unwrap();
        let height = parts[4].parse::<u32>().unwrap();
        let x_o = parts[5].parse::<f32>().unwrap();
        let y_o = parts[6].parse::<f32>().unwrap();
        let z_o = parts[7].parse::<f32>().unwrap();
        let omega = parts[8].parse::<f32>().unwrap();
        let phi = parts[9].parse::<f32>().unwrap();
        let kappa = parts[10].parse::<f32>().unwrap();
        Camera {
            c,
            xo,
            yo,
            width,
            height,
            x_o,
            y_o,
            z_o,
            omega,
            phi,
            kappa,
        }
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Camera c: {}", self.c)?;
        writeln!(f, "Camera xo: {} yo: {}", self.xo, self.yo)?;
        writeln!(f, "User Xo: {} Yo: {} Zo: {}", self.x_o, self.y_o, self.z_o)?;
        writeln!(
            f,
            "Rotations omega: {} phi: {} kappa: {}",
            self.omega, self.phi, self.kappa
        )?;
        write!(f, "")
    }
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
        let x = parts[0].parse::<f32>().unwrap() * 1000f32;
        let y = parts[1].parse::<f32>().unwrap() * 1000f32;
        let z = parts[2].parse::<f32>().unwrap() * 1000f32;

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
