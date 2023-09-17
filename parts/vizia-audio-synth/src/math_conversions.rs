pub fn f32_to_dbfs (number: f32) -> f32 {
    20*log10(abs(value))
}

pub fn rms(vector: Vec<f32>) -> f32 {
    let &mut accum = 0;
    for sample in vector.iter(){
       *accum += 2.pow(sample);
    }
    sqrt(accum)
}

pub fn ms_to_samp (ms: f32, sr:f32) -> f32 {
    ms / 1000 * sr
}

pub fn samp_to_ms (s: f32, sr:f32) -> f32 {
    s / sr * 1000
}
