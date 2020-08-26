use crate::ColorTuple;

use super::Rgb;

pub enum GrayScaleMethod {
  Average,
  AverageProminent,
  Luminance,
  Rec709,
  Rec2100,
}

static R_YUV_FACTOR: f64 = 0.299;
static G_YUV_FACTOR: f64 = 0.587;
static B_YUV_FACTOR: f64 = 0.114;
static YUV_FACTORS: ColorTuple = (R_YUV_FACTOR, G_YUV_FACTOR, B_YUV_FACTOR);

static R_REC709_FACTOR: f64 = 0.2126;
static G_REC709_FACTOR: f64 = 0.7152;
static B_REC709_FACTOR: f64 = 0.0722;
static REC709_FACTORS: ColorTuple =
  (R_REC709_FACTOR, G_REC709_FACTOR, B_REC709_FACTOR);

static R_REC2100_FACTOR: f64 = 0.2627;
static G_REC2100_FACTOR: f64 = 0.6780;
static B_REC2100_FACTOR: f64 = 0.0593;
static REC2100_FACTORS: ColorTuple =
  (R_REC2100_FACTOR, G_REC2100_FACTOR, B_REC2100_FACTOR);

fn mul(rgb: &mut Rgb, factors: ColorTuple) {
  rgb.r *= factors.0;
  rgb.g *= factors.1;
  rgb.b *= factors.2;
}

fn rgb_to_grayscale_lum(rgb: &mut Rgb) {
  mul(rgb, YUV_FACTORS)
}

fn rgb_to_grayscale_rec709(rgb: &mut Rgb) {
  mul(rgb, REC709_FACTORS);
}
fn rgb_to_grayscale_rec2100(rgb: &mut Rgb) {
  mul(rgb, REC2100_FACTORS);
}

fn rgb_to_grayscale_avg(rgb: &mut Rgb) {
  let y = (rgb.r + rgb.g + rgb.b) / 3.0;
  rgb.r = y;
  rgb.g = y;
  rgb.b = y;
}

fn rgb_to_grayscale_avg_prom(rgb: &mut Rgb) {
  let rgb_vec = vec![rgb.r, rgb.g, rgb.b];
  let max = rgb_vec.iter().fold(std::f64::MIN, |a, &b| a.max(b));
  let min = rgb_vec.iter().fold(std::f64::MAX, |a, &b| a.min(b));
  let y = (max + min) / 2.0;
  rgb.r = y;
  rgb.g = y;
  rgb.b = y;
}

pub fn rgb_grayscale(rgb: &mut Rgb, method: GrayScaleMethod) {
  match method {
    GrayScaleMethod::Average => rgb_to_grayscale_avg(rgb),
    GrayScaleMethod::AverageProminent => rgb_to_grayscale_avg_prom(rgb),
    GrayScaleMethod::Luminance => rgb_to_grayscale_lum(rgb),
    GrayScaleMethod::Rec709 => rgb_to_grayscale_rec709(rgb),
    GrayScaleMethod::Rec2100 => rgb_to_grayscale_rec2100(rgb),
  }
}
