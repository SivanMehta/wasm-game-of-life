use wasm_bindgen::prelude::*;
use js_sys::Math;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct Output {
    avg: f64,
    min: f64,
    max: f64
}

#[wasm_bindgen]
impl Output {
    pub fn avg(&self) -> f64 { return self.avg; }
    pub fn min(&self) -> f64 { return self.min; }
    pub fn max(&self) -> f64 { return self.max; }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct FPS {
    frames: [f64;100],
    last_frame_time_stamp: f64,
    current_frame: usize,
    enough: bool,
}

fn now() -> f64 {
    let perf = web_sys::window().unwrap().performance().unwrap();
    return perf.now()
}

#[wasm_bindgen]
impl FPS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FPS {
        return FPS {
            frames: [0.0; 100],
            last_frame_time_stamp: now(),
            current_frame: 0,
            enough: false,
        }
    }

    pub fn enough(&self) -> bool {
        return self.enough;
    }

    pub fn render(&mut self) -> Output {
        let now = now();
        let delta = (now - self.last_frame_time_stamp) as f64;
        self.last_frame_time_stamp = now;
        let fps = 1.0 / delta * 1000.0;

        // save only the last 100 timings
        self.frames[self.current_frame] = fps;
        self.current_frame = (self.current_frame + 1) % 100;
        if self.current_frame == 99 {
            self.enough = true;
        }

        let mut min = 9999.9;
        let mut max = 0.0;
        let mut sum = 0.0;

        for frame in self.frames {
            min = Math::min(min, frame);
            max = Math::max(max, frame);
            sum += frame;
        }

        let avg = sum / 100.0;
        return Output {
            min: Math::round(min),
            avg: Math::round(avg),
            max: Math::round(max),
        };
    }
}