
struct InputWindInformation {
    heading: u16,
    strength_knots: u8,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum HeadingBugSide {
    Left = 0,
    Center = 1,
    Right = 2,
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedWindSegment {
    human_name: str, // Better as an enum but I wanted to try passing string data.
    start_bearing_inclusive: u16,
    end_bearing_exclusive: u16,

    head_wind_knots: i16,
    head_wind_time_correction: i16,
    cross_wind_knots: u8,
    ground_speed_knots: i16,
    heading_correction_required_bearing: i8,

    heading_correction_double: i8,
    heading_correction_triple: i8,

    human_fly_heading_bug_side: HeadingBugSide,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedWindInformationAtLevel {
    heading: u16,
    altitude_start: u16,
    altitude_end: u16,
    fl_start_inclusive: u8,
    fl_end_inclusive: u8,
    strength_knots: u8,
    segments: Vec<CalculatedWindSegment>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedWindInformation {
    levels: Vec<CalculatedWindInformationAtLevel>
}
