mod solid_color;
pub use solid_color::SolidColorTexture;

mod checker;
pub use checker::CheckerTexture;

mod image;
pub use self::image::ImageTexture;

use nalgebra::Vector3;

pub trait Texture: Sync + Send {
    fn get_color(&self, uv: (f64, f64), point: &Vector3<f64>) -> Vector3<f64>;
}
