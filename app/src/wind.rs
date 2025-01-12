
#[wasm_bindgen]
#[derive(Debug)]
struct InputWindInformation {
    heading: u16,
    strength_knots: u8,
    altitude_start_ft: u16,
    altitude_end_ft: u16,
}

#[wasm_bindgen]
#[derive(Debug)]
struct InputDesiredTrack {
    track: u16,
    altitude_ft: u16,
    ias_knot: u16,
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
    head_wind_time_correction_seconds: i16,
    cross_wind_knots: u8,
    heading_correction_relative_bearing: i8,
    heading_correction_relative_bearing_double: i8,
    heading_correction_relative_bearing_triple: i8,

    human_fly_heading_bug_side: HeadingBugSide,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedWindInformationAtLevel {
    heading: u16,
    altitude_start_ft: u16,
    altitude_end_ft: u16,
    fl_start_inclusive: u8,
    fl_end_inclusive: u8,
    strength_knots: u8,
    segments: Vec<CalculatedWindSegment>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedForDesiredTrack {
    cross_wind_knots: u8,
    heading_absolute: u16,
    heading_absolute_double: i8,
    heading_absolute_triple: i8,
    human_fly_heading_bug_side: HeadingBugSide,
    
    head_wind_knots: i16,
    head_wind_time_correction_seconds: i16,
    ground_speed_knots: i16,

    segment_index: usize,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct CalculatedWindInformation {
    levels: Vec<CalculatedWindInformationAtLevel>,
    for_desired_track: Option<CalculatedForDesiredTrack>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct WindState {
    calculated: Option<CalculatedWindInformation>,
    input_data: Vec<InputWindInformation>,
    input_exact_desired_track: InputDesiredTrack,
}

#[wasm_bindgen]
impl WindState {
    pub fn new() -> WindState {
        WindState {
            calculated: None,
            input_data: vec![
                InputWindInformation {
                    heading: 0,
                    strength_knots: 0,
                    altitude_start_ft: 0,
                    altitude_end_ft: 60000,
                }
            ],
            input_exact_desired_track: InputDesiredTrack {
                track: 0,
                altitude_ft: 2000,
                ias_knot: 100,
            }
        }
    }

    pub fn recalculate(self) {
        self.calculated = Some(CalculatedWindInformation{
            levels: vec!(
                CalculatedWindInformationAtLevel {
                    heading: u16,
                    altitude_start_ft: u16,
                    altitude_end_ft: u16,
                    fl_start_inclusive: u8,
                    fl_end_inclusive: u8,
                    strength_knots: u8,
                    segments: vec!(
                        CalculatedWindSegment {
                            human_name: "Everywhere",
                            start_bearing_inclusive: 0,
                            end_bearing_exclusive: 361,

                            head_wind_knots: 0,
                            head_wind_time_correction: 0,
                            cross_wind_knots: 0,
                            heading_correction_required_bearing: 0,

                            heading_correction_double: 0,
                            heading_correction_triple: 0,

                            human_fly_heading_bug_side: HeadingBugSide::Center,
                        }
                    )
                }
            ),
            for_desired_track: CalculatedForDesiredTrack {
                cross_wind_knots: 0,
                heading_absolute: 0,
                heading_absolute_double: 0,
                heading_absolute_triple: 0,
                human_fly_heading_bug_side: HeadingBugSide::Center,
                
                head_wind_knots: 0,
                head_wind_time_correction_seconds: 0,
                ground_speed_knots: 0,

                segment_index: 0,
            }
        })
    }
}


