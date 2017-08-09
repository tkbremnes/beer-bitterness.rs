pub struct Tinseth;

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
    1.65 * ((0.000125 as f64).powf(wort_gravity as f64 - 1.0))
}

fn boil_time_factor(
    boil_time: u32
) -> f64 {
    // The Boil Time factor accounts for the change in utilization due to boil time
    let e = ::std::f64::consts::E;
    ((1.0 - e.powf(-0.04 * boil_time as f64)) / 4.15)
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
