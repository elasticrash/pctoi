use crate::helper::distance;
use crate::models::{Camera, ExPnt};
use image::Rgb;
use image::RgbImage;
use std::collections::HashMap;

pub fn save_image(pr: Vec<ExPnt>, camera: &Camera) {
    let mut image = RgbImage::new(camera.width, camera.height);

    let mut hm: HashMap<String, (ExPnt, f32)> = HashMap::new();

    for point in pr {
        let pdist = (
            point.clone(),
            distance(
                camera.x_o, camera.y_o, camera.z_o, point.r_x, point.r_y, point.r_z,
            ),
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
