use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct AnalogDetectThresholds {
    a0_low: u8,
    a0_high: u8,
    a1_low: u8,
    a1_high: u8,
    a2_low: u8,
    a2_high: u8,
    a3_low: u8,
    a3_high: u8,
    a4_low: u8,
    a4_high: u8,
    a5_low: u8,
    a5_high: u8,
}

impl Default for AnalogDetectThresholds {
    fn default() -> Self {
        AnalogDetectThresholds {
            a0_low: 0,
            a0_high: 0,
            a1_low: 0,
            a1_high: 0,
            a2_low: 0,
            a2_high: 0,
            a3_low: 0,
            a3_high: 0,
            a4_low: 0,
            a4_high: 0,
            a5_low: 0,
            a5_high: 0,
        }
    }
}
