use super::ColorTuple;

pub enum GrayScaleMethod {
  Average,
  AverageProminent,
  Luminance,
  Rec709,
  Rec2100
}

static R_YUV_FACTOR: f32 = 0.299;
static G_YUV_FACTOR: f32 = 0.587;
static B_YUV_FACTOR: f32 = 0.114;

static R_REC709_FACTOR: f32 = 0.2126;
static G_REC709_FACTOR: f32 = 0.7152;
static B_REC709_FACTOR: f32 = 0.0722;

static R_REC2100_FACTOR: f32 = 0.2627;
static G_REC2100_FACTOR: f32 = 0.6780;
static B_REC2100_FACTOR: f32 = 0.0593;

fn rgb_to_grayscale_lum(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb;
  (r * R_YUV_FACTOR, g * G_YUV_FACTOR, b * B_YUV_FACTOR)
}

fn rgb_to_grayscale_rec709(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb;
  (r * R_REC709_FACTOR, g * G_REC709_FACTOR, b * B_REC709_FACTOR)
}
fn rgb_to_grayscale_rec2100(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb;
  (r * R_REC2100_FACTOR, g * G_REC2100_FACTOR, b * B_REC2100_FACTOR)
}

fn rgb_to_grayscale_avg(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb;
  let y = (r + g + b) / 3.0;
  (y, y, y)
}

fn rgb_to_grayscale_avg_prom(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb;
  let rgb_vec = vec![r, g, b];
  let max = rgb_vec.iter().fold(std::f32::MIN, |a, &b| a.max(*b));
  let min = rgb_vec.iter().fold(std::f32::MAX, |a, &b| a.min(*b));
  let y = (max + min) / 2.0;
  (y, y, y)
}


pub fn rgb_grayscale(rgb: &ColorTuple, method: GrayScaleMethod) -> ColorTuple {
  match method {
    GrayScaleMethod::Average => rgb_to_grayscale_avg(rgb),
    GrayScaleMethod::AverageProminent => rgb_to_grayscale_avg_prom(rgb),
    GrayScaleMethod::Luminance => rgb_to_grayscale_lum(rgb),
    GrayScaleMethod::Rec709 => rgb_to_grayscale_rec709(rgb),
    GrayScaleMethod::Rec2100 => rgb_to_grayscale_rec2100(rgb),
  }
}
