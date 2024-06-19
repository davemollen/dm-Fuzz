use bilinear_transform::BilinearTransform;

mod bilinear_transform;

const R1: f32 = 10000.;
const R2: f32 = 47000.;
const R3: f32 = 100000.;
const C1: f32 = 2.2e-8;

pub struct Filter {
  bilinear_transform: BilinearTransform,
  z: f32,
}

impl Filter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(tone);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.apply_filter(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(tone: f32) -> ([f32; 2], [f32; 2]) {
    let r3_a = (1. - tone) * R3;
    let r3_b = tone * R3;

    let b0 = r3_b * C1 * r3_a + r3_b * C1 * R2;
    let b1 = R3;
    let a0 = b0 + C1 * R1 * r3_a + C1 * R1 * R2;
    let a1 = b1 + R2 + R1;

    ([b0, b1], [a0, a1])
  }

  fn apply_filter(&mut self, x: f32, (b, a): ([f32; 2], [f32; 2])) -> f32 {
    let y = x * b[0] + self.z;
    self.z = x * b[1] - y * a[1];

    y
  }
}
