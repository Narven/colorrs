#[allow(dead_code)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: Option<f32>,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: Option<f32>) -> Self {
        match a {
            Some(alpha) => Self {
                r,
                g,
                b,
                a: Some(alpha),
            },
            _ => Self { r, g, b, a: None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_rgb() {
        let rgb = Rgba::new(10, 10, 10, None);

        assert_eq!(rgb.r, 10);
        assert_eq!(rgb.g, 10);
        assert_eq!(rgb.b, 10);
        assert_eq!(rgb.a, None);
    }

    #[test]
    fn create_new_rgba() {
        let rgb = Rgba::new(10, 10, 10, Some(1.0));

        assert_eq!(rgb.r, 10);
        assert_eq!(rgb.g, 10);
        assert_eq!(rgb.b, 10);
        assert_eq!(rgb.a, Some(1.0));
    }
}
