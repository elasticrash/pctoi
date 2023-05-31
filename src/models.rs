#[derive(Debug)]
pub struct Camera {
    pub name: String,
    pub c: f32,
    pub xo: f32,
    pub yo: f32,
    pub k1: f32,
    pub k2: f32,
    pub width: i32,
    pub height: i32,
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
        let name = parts[0].to_string();
        let c = parts[1].parse::<f32>().unwrap();
        let xo = parts[2].parse::<f32>().unwrap();
        let yo = parts[3].parse::<f32>().unwrap();
        let k1 = parts[4].parse::<f32>().unwrap();
        let k2 = parts[5].parse::<f32>().unwrap();
        let width = parts[6].parse::<i32>().unwrap();
        let height = parts[7].parse::<i32>().unwrap();
        let x_o = parts[8].parse::<f32>().unwrap();
        let y_o = parts[9].parse::<f32>().unwrap();
        let z_o = parts[10].parse::<f32>().unwrap();
        let omega = parts[11].parse::<f32>().unwrap();
        let phi = parts[12].parse::<f32>().unwrap();
        let kappa = parts[13].parse::<f32>().unwrap();
        Camera {
            name,
            c,
            xo,
            yo,
            k1,
            k2,
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

#[derive(Debug)]
pub struct Point {
    pub r_x: f32,
    pub r_y: f32,
    pub r_z: f32,
    pub int: u8,
}

impl Point {
    pub fn new(line: String) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let r_x = parts[0].parse::<f32>().unwrap();
        let r_y = parts[1].parse::<f32>().unwrap();
        let r_z = parts[2].parse::<f32>().unwrap();
        let int = parts[3].parse::<u8>().unwrap();

        Point { r_x, r_y, r_z, int }
    }
}

#[derive(Debug)]
pub struct ExPnt {
    pub r_x: f32,
    pub r_y: f32,
    pub r_z: f32,
    pub int: u8,
    pub p_x: i32,
    pub p_y: i32,
    pub p_z: i32,
}
