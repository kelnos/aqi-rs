# `aqi`

[![crates.io][crates-shield]][crates-url]
[![Documentation][docs-shield]][docs-url]
[![Apache 2.0][license-shield]][license-url]
[![Build Status][build-shield]][build-url]

The `aqi` crate provides functions for calculating the Air Quality
Index based on concentrations of particuate matter.

The AQI is defined for ozone (O₃), 1.0-2.5 micron particulate matter
(PM2.5), 2.5-10 micron particulate matter (PM10), carbon monoxide
(CO), sulfur dioxide (SO₂), and nitrogen dioxide (NO₂).

The AQI helps make air quality more understandable to laypersons,
normalizing air quality on a scale from 0 to 500, with round-number
ranges that indicate qualities such as "Good", "Unhealthy", and
"Hazardous".

For further information, see [AQI basics][aqi-info] and [AQI techinical
assistance][aqi-detail].

Additionally, this library supports "adjusted" PM2.5 AQI values, using
the LRAPA and AQandU conversion formulas.

[crates-shield]: https://img.shields.io/crates/v/aqi.svg
[crates-url]: https://crates.io/crates/aqi
[docs-shield]: https://docs.rs/aqi/badge.svg
[docs-url]: https://docs.rs/aqi
[license-shield]: https://img.shields.io/crates/l/aqi.svg
[license-url]: https://github.com/kelnos/aqi/blob/master/LICENSE
[build-shield]: https://img.shields.io/github/workflow/status/kelnos/aqi-rs/CI
[build-url]: https://github.com/kelnos/aqi-rs/actions

[aqi-info]: https://www.airnow.gov/aqi/aqi-basics/
[aqi-detail]: https://www.airnow.gov/publications/air-quality-index/technical-assistance-document-for-reporting-the-daily-aqi/
