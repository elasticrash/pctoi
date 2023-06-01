pub fn r11(phi: f32, kapa: f32) -> f32 {
    phi.cos() * kapa.cos()
}

pub fn r12(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.cos() * kapa.sin() + omega.sin() * phi.sin() * kapa.cos()
}

pub fn r13(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.sin() * kapa.sin() - omega.cos() * phi.sin() * kapa.cos()
}

pub fn r21(phi: f32, kapa: f32) -> f32 {
    -phi.cos() * kapa.sin()
}

pub fn r22(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.cos() * kapa.cos() - omega.sin() * phi.sin() * kapa.sin()
}

pub fn r23(omega: f32, phi: f32, kapa: f32) -> f32 {
    omega.sin() * kapa.cos() + omega.cos() * phi.sin() * kapa.sin()
}

pub fn r31(phi: f32) -> f32 {
    phi.sin()
}

pub fn r32(omega: f32, phi: f32) -> f32 {
    -omega.sin() * phi.cos()
}

pub fn r33(omega: f32, phi: f32) -> f32 {
    omega.cos() * phi.cos()
}
