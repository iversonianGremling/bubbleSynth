use crate::TWOPI;
pub enum OscillatorType {
    Sine,
    Triangle,
    Square,
    Saw,
}

pub fn oscillator(waveform: OscillatorType, t: f32) -> f32 {
    match waveform {
        OscillatorType::Sine => t.sin(),
        OscillatorType::Triangle => {
            let t = t/TWOPI + 0.25;
            if t < 0.5 {
                (t - 0.25) * 4.0
            } else {
                (-t + 0.75) * 4.0
            }
        },
        OscillatorType::Square => {
            let mut result = 0.0;
            let sine = t.sin();
            if sine > 0.0 {
                1.0
            } else {
                -1.0
            }
        },
        OscillatorType::Saw => {
            (t/TWOPI - 0.5) * 2.0
        },
    }
}

//pub fn next(&mut self) -> f64 {
//    let t = self.phase / TWO_PI;
//
//    let value = match self.shape {
//        Shape::Sine => self.next_aliased(Shape::Sine),
//        Shape::Saw => self.next_aliased(Shape::Saw) - self.poly_blep(t),
//        Shape::Noise => self.next_aliased(Shape::Noise),
//        _ => {
//            let mut v = self.next_aliased(Shape::Square)
//                + self.poly_blep(t)
//                - self.poly_blep((t + 0.5) % 1.0);
//            if let Shape::Triangle = self.shape {
//                v = self.phase_increment * v + (1.0 - self.phase_increment) * self.last_output;
//                self.last_output = v;
//            }
//            v
//        }
//    };
//    pub fn poly_blep(&self, t: f64) -> f64 {
//        let dt = self.phase_increment / TWO_PI;
//        // 0 <= t < 1
//        if t < dt {
//            let t = t / dt;
//            return t + t - t * t - 1.0;
//        }
//        // -1 < t < 0
//        else if t > 1.0 - dt {
//            let t = (t - 1.0) / dt;
//            return t * t + t + t + 1.0;
//        }
//        // 0 otherwise
//        else {
//            return 0.0;
//        }
    //    }
