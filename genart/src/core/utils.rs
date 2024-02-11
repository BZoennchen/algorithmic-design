use nannou::prelude::*;

/// Returns the intensified value `x`. The intensity depends on `intensity`.
///
/// # Arguments
/// 
/// * `intensity` - A parameter that controls how much `x` is intensified
/// * `x` - The argument that has to be in [0,1]
/// 
/// # Examples
/// 
/// ```
/// use genart::core::utils::ease;
/// let result = ease(1.0, 0.7);
/// assert!((result - 0.7).abs() < 0.0001);
/// ```
/// 
/// ```
/// use genart::core::utils::ease;
/// let result = ease(2.0, 0.7);
/// assert!(result > 0.7);
/// assert!(result <= 1.0);
/// ```
/// 
/// ```
/// use genart::core::utils::ease;
/// let result = ease(0.3, 0.7);
/// assert!(result < 0.7);
/// assert!(result > 0.0);
/// ```
/// 
pub fn ease(intensity: f32, x: f32) -> f32 {
    if x < 0.5 {
        0.5 * (2.0*x).pow(intensity)
    }
    else {
        1.0 - 0.5 * (2.0 * (1.0-x)).pow(intensity)
    }
}

/// Returns the linear interpolation between `a` and `b` depending on `t`.
/// 
/// # Arguments
/// 
/// * `a` - The first floating value
/// * `b` - The second floating value
/// * `t` - The interpolation factor in the range [0,1].
/// 
/// # Examples
/// 
/// ```
/// use genart::core::utils::lerp;
/// let result = lerp(1.0, 2.0, 0.0);
/// assert_eq!(result, 1.0);
/// ```
/// 
/// ```
/// use genart::core::utils::lerp;
/// let result = lerp(-2.0, 0.5, 1.0);
/// assert_eq!(result, 0.5);
/// ```
/// 
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0-t)*a + t*b
}

/// Returns a non-linear transition value between `edge0` and `edge1`. Compare https://en.wikipedia.org/wiki/Smoothstep.
/// 
/// * `edge0` - The left edge parameter, has to be smaller than `edge1`
/// * `edge1` - The right edge parameter, has to be greater than `edge0`
/// * `x` - The real number as argument
/// 
/// # Examples
/// 
/// ```
/// use genart::core::utils::smoothstep;
/// let result = smoothstep(0.0, 1.0, -10.0);
/// assert_eq!(result, 0.0);
/// ```
/// 
/// ```
/// use genart::core::utils::smoothstep;
/// let result = smoothstep(0.0, 1.0, 10.0);
/// assert_eq!(result, 1.0);
/// ```
/// 
/// ```
/// use genart::core::utils::smoothstep;
/// let result = smoothstep(0.0, 1.0, 0.5);
/// assert!((result-0.5).abs() < 0.001);
/// ```
/// 
/// ```
/// use genart::core::utils::smoothstep;
/// let result = smoothstep(0.0, 1.0, 0.6);
/// assert!(result > 0.6);
/// ```
/// 
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let mut y = (x - edge0) / (edge1 - edge0);
    y = y.clamp(0.0, 1.0);
    y * y * (3.0 - 2.0 * y)
}


pub mod colors {
    use super::*;

    pub fn color(t: f32, a: Vec3, b: Vec3, c: Vec3, d: Vec3, alpha: f32) -> Rgba<f32>{
    //let color = a + b * ((c*t+d)).cos();
    
        let tmp = c * t + d;
        let tmp = vec3(
            (2.0 * f32::PI()* tmp.x).cos(), 
            (2.0 * f32::PI()* tmp.y).cos(),
            (2.0 * f32::PI()* tmp.z).cos());
        
        let tmp = a + b * tmp;

        rgba(tmp.x, tmp.y, tmp.z, alpha)
    }

    pub fn rainbow(t: f32, alpha: f32) -> Rgba<f32>{
        let a = vec3(0.5, 0.5, 0.5);
        let b = vec3(0.5, 0.5, 0.5);
        let c = vec3(1.0, 1.0, 1.0);
        let d = vec3(0.00, 0.33, 0.67);
        color(t, a, b, c, d, alpha)
    }

     pub fn red_green(t: f32, alpha: f32) -> Rgba<f32>{
        let a = vec3(0.8, 0.5, 0.4);
        let b = vec3(0.2, 0.4, 0.2);
        let c = vec3(2.0, 1.0, 1.0);
        let d = vec3(0.00, 0.25, 0.25);
        color(t, a, b, c, d, alpha)
    }
}

pub mod consts {
    pub const TWO_PI: f32 = std::f32::consts::PI * 2.0;
}