//ADSR ---------------------------------------------------------------------------------
pub enum ADSRstage {//Note, this value isn't read at any moment
    Attack,
    Decay,
    Sustain,
    Release
}
pub struct ADSR {
    counter: f32,
    sample_rate: f32,
    ad_ended: bool,
    last_value_after_trigger_off: f32,
    last_value_release: f32,
    stage: ADSRstage,
    adsr_started: bool,
    release_flag: bool
}

impl ADSR {
    pub fn new() -> ADSR {
        ADSR {
            counter: 0.0,
            sample_rate: 0.0,
            ad_ended: false,
            last_value_after_trigger_off: 0.0,
            last_value_release: 0.0,
            stage: ADSRstage::Attack,
            adsr_started: false,
            release_flag: false
        }
    }
    pub fn set_adsr (&mut self, sr: f32) {
        self.counter = 0.0;
        self.ad_ended = false;
        self.sample_rate = sr;

    }
    pub fn gen_adsr(&mut self, trigger: bool) -> f32{
        let result: f32;
        let attack_time = 300.0/1000.0*self.sample_rate;
        let decay_time = 30.0/1000.0*self.sample_rate;
        let sustain_value: f32 = 1.0;
        let release_time: f32 = 600.0/1000.0*self.sample_rate;

        if trigger {
            self.release_flag = false;
            if self.counter < attack_time {
                self.stage = ADSRstage::Attack;
                result = self.counter / attack_time * (1f32 - self.last_value_release) + self.last_value_release;
                self.last_value_after_trigger_off = result;
                self.counter += 1.0;
            } else if self.counter < attack_time + decay_time {
                self.stage = ADSRstage::Decay;
                let normalized_counter = self.counter - attack_time;
                result = (1.0 - normalized_counter/decay_time) * (1f32 - sustain_value) + sustain_value;
                self.last_value_after_trigger_off = result;
                self.counter += 1.0;
            } else {
                self.stage = ADSRstage::Sustain;
                result = sustain_value;
                self.last_value_after_trigger_off = result;
            }
            result
        } else {
            self.stage = ADSRstage::Release;
            if !self.release_flag {
                self.release_flag = true;
                self.counter = 0.0;
            }
            result = (self.last_value_after_trigger_off - ((self.counter) / release_time) * self.last_value_after_trigger_off).max(0f32);
            self.last_value_release = result;
            self.counter += 1.0;
            if result == 0.0 {
                self.ad_ended = true;
            }
            result
        }
    }
}
//--------------------------------------------------------------------------------------
