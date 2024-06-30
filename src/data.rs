type DensityAltFt = u16;

type LandingDistanceFt = u16;
type LandingFrom50DistanceFt = u16;
type TakeoffDistance = u16;
type TakeoffTo50DistanceFt = u16;

type LandingWeightLbs = u16;

enum Aircraft {
    GBRPK,
    GSHWK,
    GHERC,
    GORDM
}

fn density_landing_performance_pa28_140(
    density_alt: DensityAltFt,
) -> (LandingDistanceFt, LandingFrom50DistanceFt) {
    // Calculated from 'PA-28-140 CHEROKEE LANDING PERFORMANCE' chart from the POH 9-11.
    // Revised June 13, 1974.
    // https://docs.google.com/spreadsheets/d/1MfRBPeXb-dD4VKzkJaTZdos9Izj5-sAeotru8eLhINs/edit?gid=0#gid=0
    return ((density_alt as f32 * 0.017 + 536.0).round() as LandingDistanceFt, (density_alt as f32 * 0.033 + 1069.0).round() as LandingFrom50DistanceFt)
}

macro_rules! density_landing_performance_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn test_known_landing_performance_density_$name() {
                let (density_alt, expected) = $value;
                assert_eq!(expected, density_landing_performance_pa28_140(density_alt));
            }
        )*
    }
}

// Manually read from the graph:
density_landing_performance_tests! {
    zero: (0, (535, 1070)),
    five_hundred: (500, (545, 1085)),
    one_thousand: (1000, (555, 1100)),
    one_thousand_five_hundred: (1500, (560, 1120)),
}

