pub struct BilinearTransform {
  s: f32,
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self { s: t / 2. }
  }

  pub fn process(&self, (mut b, mut a): ([f32; 2], [f32; 2])) -> ([f32; 2], [f32; 2]) {
    b[1] *= self.s;
    let b0 = b[0] + b[1];
    let b1 = b[0] - b[1];

    a[1] *= self.s;
    let a0 = a[0] + a[1];
    let a1 = a[0] - a[1];

    ([b0 / a0, -b1 / a0], [1., -a1 / a0])
  }
}
