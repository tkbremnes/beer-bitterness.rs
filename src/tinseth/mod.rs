pub struct Tinseth;

const BIGNESS_GRAVITY_BASE:       f64 = 0.000125;
const BIGNESS_GRAVITY_MULTIPLIER: f64 = 1.65;
const CURVE_SHAPE:                f64 = -0.04;
const MAX_UTILIZATION:            f64 = 4.15;

// tinseth source: http://realbeer.com/hops/research.html
fn alpha_acid_concentration(
    alpha_acids: f32,
    hop_weight: u32,
    batch_size: u32
) -> f64 {
    // returns mg/l of added alpha acids
    // I have seen calculators that does (batch_size - (batch_size * 0.1)). dunno why
    ((alpha_acids as f64 * hop_weight as f64 * 1000.0) / batch_size as f64)
}

fn bigness_factor(
    wort_gravity: f32
) -> f64 {
    // The Bigness factor accounts for reduced utilization due to higher wort gravities. Use an
    // average gravity value for the entire boil to account for changes in the wort volume.
    BIGNESS_GRAVITY_MULTIPLIER * (BIGNESS_GRAVITY_BASE.powf(wort_gravity as f64 - 1.0))
}

fn boil_time_factor(
    boil_time: u32
) -> f64 {
    // The Boil Time factor accounts for the change in utilization due to boil time
    let e = ::std::f64::consts::E;
    ((1.0 - e.powf(CURVE_SHAPE * boil_time as f64)) / MAX_UTILIZATION)
}

impl Tinseth {
    pub fn calculate_ibu(
        hop_weight: u32,
        boil_time: u32,
        alpha_acids: f32,
        batch_size: u32,
        wort_gravity: f32
    ) -> f64 {
        // IBUs = decimal alpha acid utilization * mg/l of added alpha acids
        // decimal alpha acid utilization = Bigness factor * Boil Time factor
        (
            bigness_factor(wort_gravity) *
            boil_time_factor(boil_time) *
            alpha_acid_concentration(alpha_acids, hop_weight, batch_size)
        )
    }
}
