use std::collections::HashSet;
use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt};
use nom::sequence::tuple;
use nom::IResult;

use crate::util::Coordinates;
use crate::{DaySolution, FromInput};

pub struct Day15(Vec<BeaconSensor>);

#[derive(Debug)]
struct BeaconSensor {
    sensor: Coordinates,
    beacon: Coordinates,
}

impl BeaconSensor {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("Sensor at "),
                parse_coords,
                tag(": closest beacon is at "),
                parse_coords,
            )),
            |(_, sensor, _, beacon)| Self { sensor, beacon },
        )(input)
    }

    fn beacon_distance(&self) -> isize {
        (self.beacon - self.sensor).manhattan()
    }

    fn non_beacon_spots_at_height(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let distance_from_sensor = (y - self.sensor.y).abs();

        if distance_from_sensor <= self.beacon_distance() {
            let distance_from_vertical = self.beacon_distance() - distance_from_sensor;
            Some(
                (self.sensor.x - distance_from_vertical)..=(self.sensor.x + distance_from_vertical),
            )
        } else {
            None
        }
    }
}

fn parse_coords(input: &str) -> IResult<&str, Coordinates> {
    map(
        tuple((tag("x="), parse_isize, tag(", y="), parse_isize)),
        |(_, x, _, y)| Coordinates { x, y },
    )(input)
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(
        tuple((opt(tag("-")), digit1)),
        |(sign, digits): (Option<&str>, &str)| {
            let sign = if sign.is_some() { -1 } else { 1 };
            digits.parse::<isize>().map(|d| d * sign)
        },
    )(input)
}

impl FromInput for Day15 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, beacon_sensor) =
                        BeaconSensor::parse(&line).expect("Invalid beacon sensor");
                    beacon_sensor
                })
                .collect(),
        )
    }
}

impl DaySolution for Day15 {
    fn part_one(&self) -> String {
        let height = 2_000_000;

        let non_beacon_ranges = self.non_beacon_ranges_at_height(height);
        let beacons_in_ranges = self
            .beacons_at_height(height)
            .into_iter()
            .filter(|pos| non_beacon_ranges.iter().any(|range| range.contains(pos)))
            .count() as isize;

        let non_beacon_count = non_beacon_ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<isize>()
            - beacons_in_ranges;

        non_beacon_count.to_string()
    }

    fn part_two(&self) -> String {
        let max_height = 4_000_000;

        let distress_beacon = (0..=max_height)
            .find_map(|y| {
                Some((y, self.non_beacon_ranges_at_height(y))).filter(
                    |(_y, ranges)| !matches!(&ranges[..], [single] if single.start() <= &0 && single.end() >= &max_height),
                )
            })
            .and_then(|(y, ranges)| {
                for x in 0..=max_height {
                    if !ranges.iter().any(|range| range.contains(&x)) {
                        return Some(Coordinates { x, y });
                    }
                }

                None
            })
            .expect("Could not find distress signal");

        (distress_beacon.x * 4_000_000 + distress_beacon.y).to_string()
    }
}

impl Day15 {
    fn non_beacon_ranges_at_height(&self, y: isize) -> Vec<RangeInclusive<isize>> {
        let mut all_non_beacon_ranges = self
            .0
            .iter()
            .filter_map(|bs| bs.non_beacon_spots_at_height(y))
            .collect::<Vec<_>>();

        Self::consolidate_ranges(&mut all_non_beacon_ranges);

        all_non_beacon_ranges
    }

    fn consolidate_ranges(ranges: &mut Vec<RangeInclusive<isize>>) {
        ranges.sort_by_cached_key(|range| *range.start());

        'outer: loop {
            for index in 1..ranges.len() {
                if ranges[index - 1].end() >= ranges[index].start() {
                    let end = std::cmp::max(ranges[index - 1].end(), ranges[index].end());
                    ranges[index - 1] = (*ranges[index - 1].start())..=(*end);
                    ranges.remove(index);

                    continue 'outer;
                }
            }

            break;
        }
    }

    fn beacons_at_height(&self, y: isize) -> HashSet<isize> {
        self.0
            .iter()
            .filter_map(move |bs| {
                if bs.beacon.y == y {
                    Some(bs.beacon.y)
                } else {
                    None
                }
            })
            .collect()
    }
}
