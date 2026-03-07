//!```cargo
//![dependencies]
//!glam="*"
//! ```


use glam::{vec2, Vec3, Vec4};



fn main() {
    println!("i32: {}", size_of::<i32>());
    println!("f32: {}", size_of::<f32>());
    println!("Vec3: {}", size_of::<Vec3>());
    println!("Vec4: {}", size_of::<Vec4>());
}
