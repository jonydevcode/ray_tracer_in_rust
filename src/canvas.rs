use std::io::Write;

use crate::tuple::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    // Note that the x and y parameters are assumed to be 0-based in this book.
    // That is to say, x may be anywhere from 0 to width - 1 (inclusive), and y may be
    // anywhere from 0 to height - 1 (inclusive).
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            panic!("Canvas bounds error. Tried writing pixel to ({},{}) (x,y) when valid coords are from (0,0) to ({},{})!", x, y, self.width - 1, self.height - 1);
        }
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn to_ppm(&self) -> String {
        let mut result = String::new();
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        result.push_str(&header);

        let mut line = String::new();

        // to fix this with joins
        for row in &self.pixels {
            for pixel in row {
                let (r, g, b) = pixel.ppm_str();
                let vals = [r, g, b];
                for val in vals {
                    let s = val.to_string();
                    // add 1 for the space
                    if line.len() + s.len() + 1 <= 70 {
                        if line.len() > 0 {
                            line = line + " ";
                        }
                        line = line + &s;
                    } else {
                        result = result + &line + "\n";
                        line = String::new();
                        line = line + &s;
                    }
                }
            }
            result = result + &line + "\n";
            line = String::new();
        }
        result
    }

    pub fn write_to_ppm(&self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_ppm().as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let mut c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for y in 0..20 {
            for x in 0..10 {
                assert!(c.pixel_at(x, y).equals(&Color::black()));
            }
        }

        c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert!(c.pixel_at(2, 3).equals(&Color::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn ppm_construction() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = c.to_ppm();
        // println!("{}", ppm);

        let correct = r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#;
        // println!("---\n{}\n-----", correct);
        assert!(ppm == correct)
    }

    #[test]
    fn ppm_long_lines_splitting() {
        let mut c = Canvas::new(10, 2);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        // println!("{}", ppm);

        let correct = r#"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#;
        // println!("---\n{}\n-----", correct);
        assert!(ppm == correct)
    }
}
