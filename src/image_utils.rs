use crate::helper::distance;
use crate::models::{Configuration, ExPnt, PointType};
use image::Rgb;
use image::RgbImage;
use rand::prelude::*;
use std::collections::HashMap;

pub fn save_image(pr: Vec<ExPnt>, configuration: &Configuration) {
    let mut image = RgbImage::new(configuration.image.width, configuration.image.height);

    let mut hm: HashMap<String, (ExPnt, f32)> = HashMap::new();
    let mut max_dist = 0.0;
    let mut min_dist = f32::MAX;

    for point in pr {
        let pdist = (
            point.clone(),
            distance(
                configuration.position.x_o,
                configuration.position.y_o,
                configuration.position.z_o,
                point.x,
                point.y,
                point.z,
            ),
        );

        if pdist.1 > max_dist {
            max_dist = pdist.1;
        }
        if pdist.1 < min_dist {
            min_dist = pdist.1;
        }

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

    println!("Max dist: {}/ Min dist {}", max_dist, min_dist);
    

    for x in 0..configuration.image.width {
        for y in 0..configuration.image.height {
            match hm.get(&format!("{}-{}", x, y)) {
                Some((p, _)) => match p.ptype {
                    PointType::PointInt(int) => {
                        image.put_pixel(x, y, Rgb([int, int, int]));
                    }
                    PointType::PointRGB(r, g, b) => {
                        image.put_pixel(x, y, Rgb([r, g, b]));
                    }
                    PointType::Point => {
                        let scale = max_dist - min_dist;
                        let color = (255.0 * (1.0 - (scale / 255.0))).round() as u8;
                        image.put_pixel(x, y, Rgb([color, color, color]));
                    }
                },
                None => {
                    //let int = interpolate_nearest_pixel(&hm, x, y);
                    //image.put_pixel(x, y, Rgb([int, int, int]));
                    image.put_pixel(x, y, Rgb([0, 0, 0]));
                }
            }
        }
    }
    let mut rng = thread_rng();

    let save_path = format!("output_{}.png", rng.gen_range(0..9999));
    image.save(save_path).expect("Failed to save image");
}

//fn interpolate_nearest_pixel(hm: &HashMap<String, (ExPnt, f32)>, x: u32, y: u32) -> u8 {
//    let mut color: Vec<i32> = Vec::new();
//    for i in -1_i32..2_i32 {
//        for j in -1_i32..2_i32 {
//            if let Some((p, _dist)) = hm.get(&format!("{}-{}", x as i32 + i, y as i32 + j)) {
//                color.push(p.int as i32);
//            }
//        }
//    }
//    if color.is_empty() {
//        return 0;
//    }
//    (color.clone().into_iter().sum::<i32>() / color.len() as i32) as u8
//}
