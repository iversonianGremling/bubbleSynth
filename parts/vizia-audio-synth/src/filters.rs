//NOTA, se pueden crear mas filtros biquad http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt
//rollo shelf, peak, AP, estas weas
pub enum FilterType {
    LPF,
    HPF,
    BPF,
    BRF,
    None,
}

#[derive(Debug)]
struct Coefficients {
    b0: f32,
    b1: f32,
    b2: f32,
    a0: f32,
    a1: f32,
    a2: f32,
}

pub struct Filter {
    coefficients: Coefficients,
    input_history: [f32; 2],
    output_history: [f32; 2],
}

impl Filter {
    pub fn new () -> Self {
        Filter {
            coefficients: Coefficients {
                b0: 0.0,
                b1: 0.0,
                b2: 0.0,
                a0: 1.0,
                a1: 0.0,
                a2: 0.0,
            },
            input_history: [0.,0.],
            output_history: [0.,0.],
        }
    }
    pub fn set_filter(&mut self, cutoff: f32, q_factor: f32, sample_rate: f32, filter_type: FilterType) {
        let w0 = 2. * std::f32::consts::PI * cutoff / sample_rate;
        let alpha = w0.sin() / (2. * q_factor);
        let cos_w0 = w0.cos();
        let sin_w0: f32 = w0.sin();
        match filter_type{
            FilterType::LPF => {
                self.coefficients.b0 = (1. - cos_w0) / 2.;
                self.coefficients.b1 =  1. - cos_w0;
                self.coefficients.b2 = (1. - cos_w0) / 2.;
                self.coefficients.a0 =  1. + alpha;
                self.coefficients.a1 = -2. * cos_w0;
                self.coefficients.a2 =  1. - alpha;
            }
            FilterType::HPF => {
                self.coefficients.b0 = (1. + cos_w0)/2.;
                self.coefficients.b1 =-(1. + cos_w0);
                self.coefficients.b2 = (1. + cos_w0)/2.;
                self.coefficients.a0 =  1. + alpha;
                self.coefficients.a1 = -2. * cos_w0;
                self.coefficients.a2 =  1. - alpha;
            }
            FilterType::BPF => {
                self.coefficients.b0 =  sin_w0/2.;
                self.coefficients.b1 =  0.;
                self.coefficients.b2 = -sin_w0/2.;
                self.coefficients.a0 =  1. + alpha;
                self.coefficients.a1 = -2. * cos_w0;
                self.coefficients.a2 =  1. - alpha;
            }
            FilterType::BRF => {
                self.coefficients.b0 =  1.;
                self.coefficients.b1 = -2. * cos_w0;
                self.coefficients.b2 =  1.;
                self.coefficients.a0 =  1. + alpha;
                self.coefficients.a1 = -2. * cos_w0;
                self.coefficients.a2 =  1. - alpha;
            }
            FilterType::None => {}
        }
    }
    pub fn gen_filter(&mut self, input: f32) -> f32 {
        let a0 = self.coefficients.a0;
        let output = (self.coefficients.b0/a0) * input
            + (self.coefficients.b1/a0) * self.input_history[1]  + (self.coefficients.b2/a0) * self.input_history[0]
            - (self.coefficients.a1/a0) * self.output_history[1] - (self.coefficients.a2/a0) * self.output_history[0];
        self.input_history  = [self.input_history[1], input];
        self.output_history = [self.output_history[1], output];
        output
    }
}
