// Copyright 2020 Brian J. Tarricone <brian@tarricone.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryFrom;

/// Represents the human-friendly interpretation of the AQI
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AirQualityLevel {
    /// The air quality is good and safe for everyone
    Good,
    /// The air quality is moderate, but unusually sensitive people should avoid heavy outdoor
    /// exertion
    Moderate,
    /// The air quality is unhealthy for those with respiratory issues or other health problems
    UnhealthySensitive,
    /// The air quality is unhealthy for everyone
    Unhealthy,
    /// The air quality is very unhealthy for everyone
    VeryUnhealthy,
    /// The air quality is hazardous and everyone should avoid outdoor exertion
    Hazardous,
}

macro_rules! def_try_from_aq {
    ($tpe:ty) => {
        impl TryFrom<$tpe> for AirQualityLevel {
            type Error = &'static str;
            fn try_from(v: $tpe) -> Result<Self, Self::Error> {
                use AirQualityLevel::*;
                match v {
                    0..=50 => Ok(Good),
                    51..=100 => Ok(Moderate),
                    101..=150 => Ok(UnhealthySensitive),
                    151..=200 => Ok(Unhealthy),
                    201..=300 => Ok(VeryUnhealthy),
                    301..=500 => Ok(Hazardous),
                    _ => Err("Value is out of range for AQI"),
                }
            }
        }
    };
}

def_try_from_aq!(u16);
def_try_from_aq!(i16);
def_try_from_aq!(u32);
def_try_from_aq!(i32);
def_try_from_aq!(u64);
def_try_from_aq!(i64);

/// Result type for AQI calculations
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AirQuality {
    /// The numerical AQI value, in a range between 0 and 500
    pub aqi: u32,
    /// The human-friendly interpretation of the numeric AQI value
    pub level: AirQualityLevel,
}

struct Breakpoint {
    conc_low: f64,
    conc_high: f64,
    aqi_low: u32,
    aqi_high: u32,
    level: AirQualityLevel,
}

const OZONE8_BREAKPOINTS: [Breakpoint; 5] = [
    Breakpoint {
        conc_low: 0.000,
        conc_high: 0.054,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 0.055,
        conc_high: 0.070,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 0.071,
        conc_high: 0.085,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 0.086,
        conc_high: 0.105,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 0.106,
        conc_high: 0.200,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
];
const OZONE1_BREAKPOINTS: [Breakpoint; 5] = [
    Breakpoint {
        conc_low: 0.125,
        conc_high: 0.164,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 0.165,
        conc_high: 0.204,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 0.205,
        conc_high: 0.404,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 0.405,
        conc_high: 0.504,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 0.505,
        conc_high: 0.604,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];
const PM25_BREAKPOINTS: [Breakpoint; 7] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 12.0,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 12.1,
        conc_high: 35.4,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 35.5,
        conc_high: 55.4,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 55.5,
        conc_high: 150.4,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 150.5,
        conc_high: 250.4,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 250.5,
        conc_high: 350.4,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 350.5,
        conc_high: 500.4,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];
const PM10_BREAKPOINTS: [Breakpoint; 7] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 54.0,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 55.0,
        conc_high: 154.0,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 155.0,
        conc_high: 254.0,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 255.0,
        conc_high: 354.0,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 355.0,
        conc_high: 424.0,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 425.0,
        conc_high: 504.0,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 505.0,
        conc_high: 604.0,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];
const CO_BREAKPOINTS: [Breakpoint; 7] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 4.4,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 4.5,
        conc_high: 9.4,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 9.5,
        conc_high: 12.4,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 12.5,
        conc_high: 15.4,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 15.5,
        conc_high: 30.4,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 30.5,
        conc_high: 40.4,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 40.5,
        conc_high: 50.4,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];
const SO2_1_BREAKPOINTS: [Breakpoint; 3] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 35.0,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 36.0,
        conc_high: 75.0,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 76.0,
        conc_high: 185.0,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
];
const SO2_24_BREAKPOINTS: [Breakpoint; 7] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 35.0,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 36.0,
        conc_high: 75.0,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 76.0,
        conc_high: 185.0,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 186.0,
        conc_high: 304.0,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 305.0,
        conc_high: 604.0,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 605.0,
        conc_high: 804.0,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 805.0,
        conc_high: 1004.0,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];
const NO2_BREAKPOINTS: [Breakpoint; 7] = [
    Breakpoint {
        conc_low: 0.0,
        conc_high: 53.0,
        aqi_low: 0,
        aqi_high: 50,
        level: AirQualityLevel::Good,
    },
    Breakpoint {
        conc_low: 54.0,
        conc_high: 100.0,
        aqi_low: 51,
        aqi_high: 100,
        level: AirQualityLevel::Moderate,
    },
    Breakpoint {
        conc_low: 101.0,
        conc_high: 360.0,
        aqi_low: 101,
        aqi_high: 150,
        level: AirQualityLevel::UnhealthySensitive,
    },
    Breakpoint {
        conc_low: 361.0,
        conc_high: 649.0,
        aqi_low: 151,
        aqi_high: 200,
        level: AirQualityLevel::Unhealthy,
    },
    Breakpoint {
        conc_low: 650.0,
        conc_high: 1249.0,
        aqi_low: 201,
        aqi_high: 300,
        level: AirQualityLevel::VeryUnhealthy,
    },
    Breakpoint {
        conc_low: 1250.0,
        conc_high: 1649.0,
        aqi_low: 301,
        aqi_high: 400,
        level: AirQualityLevel::Hazardous,
    },
    Breakpoint {
        conc_low: 1650.0,
        conc_high: 2049.0,
        aqi_low: 401,
        aqi_high: 500,
        level: AirQualityLevel::Hazardous,
    },
];

fn find_breakpoint(breakpoints: &[Breakpoint], concentration: f64) -> Option<&Breakpoint> {
    breakpoints.iter().find(|breakpoint| {
        breakpoint.conc_low <= concentration && concentration <= breakpoint.conc_high
    })
}

fn calc_aqi(breakpoints: &[Breakpoint], concentration: f64) -> Option<AirQuality> {
    find_breakpoint(breakpoints, concentration).map(|breakpoint| {
        let aqi = ((breakpoint.aqi_high as f64 - breakpoint.aqi_low as f64)
            / (breakpoint.conc_high - breakpoint.conc_low))
            * (concentration - breakpoint.conc_low)
            + (breakpoint.aqi_low as f64);
        AirQuality {
            aqi: round(aqi),
            level: breakpoint.level,
        }
    })
}

fn trunc(value: f64, nplaces: u32) -> f64 {
    let truncator = 10_u32.pow(nplaces) as f64;
    ((value * truncator) as u64) as f64 / truncator
}

/// Calculates the Ozone Air Quality Index from the provided 8-hour concentration
///
/// The AQI is defined for concentrations between 0.000 and 0.200 ppm.  For
/// values between 0.201 and 0.604 ppm, a 1-hour concentration should be used if
/// available.
///
/// # Arguments
///
/// * `concentration` - The 8-hour ozone concentration in ppm
pub fn ozone8(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&OZONE8_BREAKPOINTS, trunc(concentration, 3))
}

/// Calculates the ozone Air Quality Index from the provided 1-hour concentration
///
/// The AQI is defined for concentrations between 0.125 and 0.604 ppm.  For
/// values between 0.000 and 0.124 ppm, an 8-hour concentration should be used if
/// available.
///
/// # Arguments
///
/// * `concentration` - The 1-hour ozone concentration in ppm
pub fn ozone1(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&OZONE1_BREAKPOINTS, trunc(concentration, 3))
}

/// Calculates the PM2.5 Air Quality Index from the provided 24-hour concentration
///
/// The AQI is defined for concentrations between 0.0 and 500.4 µg/m³.
///
/// # Arguments
///
/// * `concentration` - The 24-hour PM2.5 concentration in µg/m³
pub fn pm2_5(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&PM25_BREAKPOINTS, trunc(concentration, 1))
}

/// Calcuates the EPA-adjusted PM2.5 Air Quality Index for the provided 24-hour concentration
///
/// See
/// [https://cfpub.epa.gov/si/si_public_record_Report.cfm?dirEntryId=350075&Lab=CEMM](https://cfpub.epa.gov/si/si_public_record_Report.cfm?dirEntryId=350075&Lab=CEMM)
/// for more information.
///
/// The EPA-adjusted AQI is defined for concentrations between 0.0 and
/// 250.0 µg/m³.
///
///
/// # Arguments
///
/// * `concentration` - The 24-hour PM2.5 concentration in µg/m³
/// * `humidity` - Relative humidity % (between 0.0 - 1.0)
pub fn pm2_5_epa(concentration: f64, humidity: f64) -> Option<AirQuality> {
    if (0.0..=1.0).contains(&humidity) {
        calc_aqi(
            &PM25_BREAKPOINTS,
            trunc(0.52 * concentration - 0.085 * humidity + 5.71, 1),
        )
    } else {
        None
    }
}

/// Calcuates the LRAPA-adjusted PM2.5 Air Quality Index for the provided 24-hour concentration
///
/// See
/// [https://www.lrapa.org/DocumentCenter/View/4147/PurpleAir-Correction-Summary](https://www.lrapa.org/DocumentCenter/View/4147/PurpleAir-Correction-Summary)
/// for more information.
///
/// The LRAPA-adjusted AQI is defined for concentrations between 0.66 and
/// 1002.12 µg/m³.
///
///
/// # Arguments
///
/// * `concentration` - The 24-hour PM2.5 concentration in µg/m³
pub fn pm2_5_lrapa(concentration: f64) -> Option<AirQuality> {
    if concentration <= 65.0 {
        calc_aqi(&PM25_BREAKPOINTS, trunc(0.5 * concentration - 0.66, 1))
    } else {
        None
    }
}

/// Calcuates the AQandU-adjusted PM2.5 Air Quality Index for the provided 24-hour concentration
///
/// See
/// [https://www.aqandu.org/airu_sensor#calibrationSection](https://www.aqandu.org/airu_sensor#calibrationSection)
/// for more information.
///
/// The AQandU-adjusted AQI is defined for concentrations between 0.0 and
/// 639.78 µg/m³.
///
/// # Arguments
///
/// * `concentration` - The 24-hour PM2.5 concentration in µg/m³
pub fn pm2_5_aqandu(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&PM25_BREAKPOINTS, trunc(0.778 * concentration + 2.65, 1))
}

/// Calculates the PM10 Air Quality Index from the provided 24-hour concentration
///
/// The AQI is defined for concentrations between 0.0 and 604.0 µg/m³.
///
/// # Arguments
///
/// * `concentration` - The 24-hour PM10 concentration in µg/m³
pub fn pm10(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&PM10_BREAKPOINTS, concentration as u32 as f64)
}

/// Calculates the carbon monoxide Air Quality Index from the provided 8-hour concentration
///
/// The AQI is defined for concentrations between 0.0 and 50.4 ppm.
///
/// # Arguments
///
/// * `concentration` - The 8-hour CO concentration in ppm
pub fn co(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&CO_BREAKPOINTS, trunc(concentration, 1))
}

/// Calculates the sulfur dioxide Air Quality Index from the provided 1-hour concentration
///
/// The AQI is  defined for concentrations between 0 and 185 ppb.  For
/// values between 186 and 1004 ppb, a 24-hour concentration should be used if
/// available.
///
/// # Arguments
///
/// * `concentration` - The 1-hour SO₂ concentration in ppb
pub fn so2_1(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&SO2_1_BREAKPOINTS, trunc(concentration, 0))
}

/// Calculates the sulfur dioxide Air Quality Index from the provided 24-hour concentration
///
/// The AQI is defined for concentrations between 0 and 1004 ppb.
///
/// # Arguments
///
/// * `concentration` - The 24-hour SO₂ concentration in ppb
pub fn so2_24(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&SO2_24_BREAKPOINTS, trunc(concentration, 0))
}

/// Calculates the nitrogen dioxide Air Quality Index from the provided 1-hour concentration
///
/// The AQI is defined for concentrations between 0 and 2049 ppb.
///
/// # Arguments
///
/// * `concentration` - The 1-hour NO₂ concentration in ppb
pub fn no2(concentration: f64) -> Option<AirQuality> {
    calc_aqi(&NO2_BREAKPOINTS, trunc(concentration, 0))
}

fn round(val: f64) -> u32 {
    #[cfg(feature = "std")]
    let res = val.round() as u32;

    #[cfg(not(feature = "std"))]
    let res = {
        let whole = val as u32;
        let frac = val - (whole as f64);
        if frac >= 0.5 {
            whole + 1
        } else {
            whole
        }
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pm2_5() {
        let test_data: [(f64, u32); 22] = [
            (0.0, 0),
            (12.0, 50),
            (12.1, 51),
            (16.0, 59),
            (35.4, 100),
            (35.5, 101),
            (55.4, 150),
            (55.5, 151),
            (85.0, 166),
            (94.0, 171),
            (138.0, 194),
            (150.4, 200),
            (150.5, 201),
            (158.0, 208),
            (175.0, 225),
            (192.0, 242),
            (200.0, 250),
            (250.4, 300),
            (250.5, 301),
            (350.4, 400),
            (350.5, 401),
            (500.4, 500),
        ];

        for (conc, aqi) in test_data.iter() {
            assert_eq!(Some(*aqi), pm2_5(*conc).map(|aq| aq.aqi));
        }
    }

    #[test]
    fn test_round() {
        assert_eq!(round(4.5), 5);
        assert_eq!(round(123.3), 123);
        assert_eq!(round(84.9), 85);
    }
}
