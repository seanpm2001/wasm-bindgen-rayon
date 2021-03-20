/**
 * Copyright 2021 Google Inc. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use hsl::HSL;
use num_complex::Complex64;
use rand::Rng;
use wasm_bindgen::{prelude::*, Clamped};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

type RGBA = [u8; 4];

struct Generator {
    width: u32,
    height: u32,
    palette: Box<[RGBA]>,
}

impl Generator {
    fn new(width: u32, height: u32, max_iterations: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            palette: (0..max_iterations)
                .map(|_| {
                    let (r, g, b) = HSL {
                        h: rng.gen_range(0.0..360.0),
                        s: 0.5,
                        l: 0.6,
                    }
                    .to_rgb();
                    [r, g, b, 255]
                })
                .collect(),
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn get_color(&self, x: u32, y: u32) -> &RGBA {
        let c = Complex64::new(
            (f64::from(x) - f64::from(self.width) / 2.0) * 4.0 / f64::from(self.width),
            (f64::from(y) - f64::from(self.height) / 2.0) * 4.0 / f64::from(self.height),
        );
        let mut z = Complex64::new(0.0, 0.0);
        let mut i = 0;
        while z.norm_sqr() < 4.0 {
            if i == self.palette.len() {
                return &self.palette[0];
            }
            z = z.powi(2) + c;
            i += 1;
        }
        &self.palette[i]
    }
}

#[wasm_bindgen]
pub fn generate(width: u32, height: u32, max_iterations: u32) -> Clamped<Vec<u8>> {
    let generator = &Generator::new(width, height, max_iterations);

    #[cfg(feature = "rayon")]
    let raw_img_data = (0..height)
        .into_par_iter()
        .flat_map_iter(|y| (0..width).flat_map(move |x| generator.get_color(x, y)))
        .copied()
        .collect();

    #[cfg(not(feature = "rayon"))]
    let raw_img_data = (0..height)
        .flat_map(|y| (0..width).flat_map(move |x| generator.get_color(x, y)))
        .copied()
        .collect();

    Clamped(raw_img_data)
}