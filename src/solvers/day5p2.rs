use std::{io::BufRead, cmp::min};
use lazy_static::lazy_static;
use regex::Regex;
use super::Solver;

lazy_static! {
    static ref SEED_REGEX: Regex = Regex::new(r"seeds: (?P<seeds>.*)").unwrap();
    static ref MAP_REGEX: Regex = Regex::new(r"(?P<id>.*) map:").unwrap();
}

struct Range {
    start: u64,
    length: u64
}

struct RangeMapSegment {
    source_start: u64,
    dest_start: u64,
    length: u64
}
struct RangeMap {
    _name: String,
    ranges: Vec<RangeMapSegment>
}

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace().map(|part| part.parse::<u64>().unwrap()).collect()
}

fn parse_seeds<I: Iterator<Item = std::io::Result<String>>>(it: &mut I) -> Option<Vec<Range>> {
    let line = it.next();
    if let Some(Ok(line_str)) = line {
        let captures = SEED_REGEX.captures(&line_str).unwrap();
        let seeds = parse_numbers(captures.name("seeds").unwrap().as_str());
        let ranges = seeds
            .chunks(2)
            .map(|chunk| Range {start: chunk[0], length: chunk[1]})
            .collect();
        let _skip_empty_line = it.next();
        return Some(ranges);
    }
    return None;
}
    

fn parse_map<I: Iterator<Item = std::io::Result<String>>>(it: &mut I) -> Option<RangeMap> {
    let description_line = it.next();
    if let Some(Ok(description)) = description_line {
        let captures = MAP_REGEX.captures(&description).unwrap();
        let name = captures.name("id").unwrap().as_str().to_string();
        let mut ranges = Vec::new();
        for line in it {
            let line_str = line.expect("Should end map with empty line instead of EOF");
            if line_str == "" {
                break;
            }
            let parts = parse_numbers(&line_str);
            ranges.push(RangeMapSegment {
                source_start: parts[1],
                dest_start: parts[0],
                length: parts[2]
            });
        }
        ranges.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        let map = RangeMap {
            _name: name,
            ranges
        };
        return Some(map);
    }
    return None;
}

fn parse_maps<I: Iterator<Item = std::io::Result<String>>>(it: &mut I) -> Vec<RangeMap> {
    let mut maps = Vec::new();
    while let Some(map) = parse_map(it) {
        maps.push(map);
    }
    maps
}

fn apply_map_to_range(map: &RangeMap, range: Range) -> Vec<Range> {
    let mut result = Vec::new();
    let mut current_start = range.start;
    let mut remaining = range.length;
    for segment in &map.ranges {
        if remaining == 0 {
            break;
        }
        if current_start > segment.source_start + segment.length {
            continue;
        }
        if current_start >= segment.source_start {
            let overlap = min(remaining, segment.source_start + segment.length - current_start);
            result.push(Range {start: segment.dest_start + (current_start - segment.source_start), length: overlap});
            current_start += overlap;
            remaining -= overlap;
        } else {
            let overlap = min(remaining, segment.source_start - current_start);
            result.push(Range {start: current_start, length: overlap});
            current_start += overlap;
            remaining -= overlap;
        }
    }
    if remaining > 0 {
        result.push(Range {start: current_start, length: remaining});
    }
    result
}

fn apply_all_maps_to_ranges(maps: &Vec<RangeMap>, ranges: Vec<Range>) -> Vec<Range> {
    let mut result = ranges;
    for map in maps {
        let mut new_result = Vec::new();
        for range in result {
            new_result.append(&mut apply_map_to_range(map, range));
        }
        result = new_result;
    }
    result
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut lines = input.lines();
        let seeds = parse_seeds(&mut lines).expect("Expected seeds");
        let maps = parse_maps(&mut lines);

        let mapped_seeds = apply_all_maps_to_ranges(&maps, seeds);

        let least_location = mapped_seeds.iter().min_by(|a, b| a.start.cmp(&b.start)).unwrap().start;
        println!("Least location: {}", least_location);
    }
};