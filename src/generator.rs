
use std::io::Write;
use std::mem;

use apng_encoder::*;
use css_color_parser::{Color as CssColor};
use rand::prelude::*;

use crate::errors::AppResultU;



pub struct Config {
    pub background_color: CssColor,
    pub fern_color: CssColor,
    pub frames: u32,
    pub height: u32,
    pub interval: u16,
    pub length: u32,
    pub width: u32,
}

pub struct Generator {
    config: Config,
    background_color: u32,
    fern_color: u32,
    rng: ThreadRng,
}


impl Generator {
    pub fn new(config: Config) -> Self {
        let background_color =  color_to_u32(config.background_color);
        let fern_color = color_to_u32(config.fern_color);
        Self {
            background_color,
            config,
            fern_color,
            rng: rand::thread_rng(),
        }
    }

    pub fn generate<W: Write>(&mut self, ouput: &mut W) -> AppResultU {
        let meta = Meta {
            width: self.config.width,
            height: self.config.height,
            color: Color::RGBA(8),
            frames: self.config.frames,
            plays: None, // Infinite loop
        };
        let frame = Frame { delay: Some(Delay::new(self.config.interval, 1000)), ..Default::default() };

        let mut encoder = Encoder::create(ouput, meta)?;

        for i in 0 .. self.config.frames {
            eprintln!("Frame {}", i + 1);
            let mut buffer: Vec<u32> = vec![];

            buffer.resize((self.config.width * self.config.height) as usize, 0);
            self.reset(&mut buffer);
            self.process(&mut buffer, i64::from(self.config.length)  + i64::from(i) * 5, 0.0, 0.0);

            unsafe {
                let ptr = buffer.as_mut_ptr() as *mut u8;
                let converted: Vec<u8> = Vec::from_raw_parts(ptr, buffer.len() * 4, buffer.capacity() * 4);
                mem::forget(buffer);
                encoder.write_frame(&converted, Some(&frame), None, None)?;
            }
        }

        eprintln!("Finish");
        encoder.finish()?;

        Ok(())
    }

    fn reset(&mut self, buffer: &mut [u32]) {
        for ptr in buffer.iter_mut() {
            *ptr = self.background_color;
        }
    }

    fn process(&mut self, buffer: &mut [u32], k: i64, x: f64, y: f64) {
        if 0 <= k {
            self.process(buffer, k - 1,  w1x(x, y), w1y(x, y));
            if self.rng.gen::<f64>() <= 0.3 {
                self.process(buffer, k - 1, w2x(x, y), w2y(x, y));
            }
            if self.rng.gen::<f64>() <= 0.3 {
                self.process(buffer, k - 1, w3x(x, y), w3y(x, y));
            }
            if self.rng.gen::<f64>() <= 0.3 {
                self.process(buffer, k - 1, w4x(x, y), w4y(x, y));
            }
        } else {
            let xi = (x + 0.5) * 0.98 * f64::from(self.config.width);
            let yi = (1.0 - y * 0.98) * f64::from(self.config.height);
            let (xi, yi) = (xi.floor() as usize, yi.floor() as usize);
            let base = yi * self.config.width as usize + xi;
            buffer[base] = self.fern_color;
        }
    }
}


fn w1x(x: f64, y: f64) -> f64 { x * 0.836 + 0.044 * y }
fn w1y(x: f64, y: f64) -> f64 { x * -0.044 + 0.836 * y + 0.169 }
fn w2x(x: f64, y: f64) -> f64 { x * -0.141 + 0.302 * y }
fn w2y(x: f64, y: f64) -> f64 { x * 0.302 + 0.141 * y + 0.127 }
fn w3x(x: f64, y: f64) -> f64 { x * 0.141 + -0.302 * y }
fn w3y(x: f64, y: f64) -> f64 { x * 0.302 + 0.141 * y + 0.169 }
fn w4x(_: f64, _: f64) -> f64 { 0.0 }
fn w4y(_: f64, y: f64) -> f64 { 0.175_337 * y }

fn color_to_u32(color: CssColor) -> u32 {
    let a = color.a * 255.0;
    let a = if 255.0 < a { 255 } else { a as u32 };
    u32::from(color.r) + (u32::from(color.g) << 8) + (u32::from(color.b) << 16) + (a << 24)
}
