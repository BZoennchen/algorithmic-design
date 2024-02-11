use nannou::noise::{NoiseFn, Perlin, OpenSimplex};

pub struct Noise<T> 
    where T: NoiseFn<[f64; 2]> + NoiseFn<[f64; 3]> + NoiseFn<[f64; 4]>,
{
    generator: T,
}

impl<T> Noise<T>
    where T: NoiseFn<[f64; 2]> + NoiseFn<[f64; 3]> + NoiseFn<[f64; 4]>,
{
    fn new(generator: T) -> Self {
        Noise { generator, }
    }

    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        self.noise2d_as_f32(x, y)
    }

    pub fn noise3d(&self, x: f32, y: f32, z: f32) -> f32 {
        self.noise3d_as_f32(x, y, z)
    }

    pub fn noise(&self, t: f32) -> f32 {
        self.noise_as_f32(t)
    }

    pub fn noise_polar(&self, t: f32, std: f32) -> f32 {
        let x = ((t * std::f32::consts::PI * 2.0).cos() + 1.0) * std;
        let y = ((t * std::f32::consts::PI * 2.0).sin() + 1.0) * std;
        self.noise2d(x, y)
    }

    pub fn noise_polar3d(&self, t: f32, std: f32, z: f32) -> f32 {
        let x = ((t * std::f32::consts::PI * 2.0).cos() + 1.0) * std;
        let y = ((t * std::f32::consts::PI * 2.0).sin() + 1.0) * std;
        self.noise3d(x, y, z)
    }

    pub fn noise_as_f32(&self, t: f32) -> f32 {
        self.generator.get([t as f64, 1.0]) as f32
    }

    pub fn noise2d_as_f32(&self, x: f32, y: f32) -> f32 {
        self.generator.get([x as f64, y as f64]) as f32
    }

    pub fn noise3d_as_f32(&self, x: f32, y: f32, z: f32) -> f32 {
        self.generator.get([x as f64, y as f64, z as f64]) as f32
    }
}

impl Noise<Perlin> {
    pub fn default() -> Self {
        Self::new(Perlin::new())
    }
}

impl Noise<OpenSimplex> {
    pub fn default_simplex() -> Self {
        Self::new(OpenSimplex::new())
    }
}

pub fn noise2d(x: f32, y: f32) -> f32 {
    let noise: Noise<Perlin> = Noise::default();
    noise.noise2d_as_f32(x, y)
    
}

pub fn noise3d(x: f32, y: f32, z: f32) -> f32 {
    let noise: Noise<Perlin> = Noise::default();
    noise.noise3d_as_f32(x, y, z)
}

pub fn noise(t: f32) -> f32 {
    let noise: Noise<Perlin> = Noise::default();
    noise.noise(t)
}

pub fn noise_polar(t: f32, std: f32) -> f32 {
    let noise: Noise<Perlin> = Noise::default();
    noise.noise_polar(t, std)
}

pub fn noise_polar3d(t: f32, std: f32, z: f32) -> f32 {
    let noise: Noise<Perlin> = Noise::default();
    noise.noise_polar3d(t, std, z)
}