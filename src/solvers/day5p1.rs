use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;
use super::Solver;

lazy_static! {
    static ref SEED_REGEX: Regex = Regex::new(r"seeds: (?P<seeds>.*)").unwrap();
    static ref MAP_REGEX: Regex = Regex::new(r"(?P<id>.*) map:").unwrap();
}

struct Range {
    source_start: u64,
    dest_start: u64,
    length: u64
}
struct RangeMap {
    _name: String,
    ranges: Vec<Range>
}

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace().map(|part| part.parse::<u64>().unwrap()).collect()
}

fn parse_seeds<I: Iterator<Item = std::io::Result<String>>>(it: &mut I) -> Option<Vec<u64>> {
    let line = it.next();
    if let Some(Ok(line_str)) = line {
        let captures = SEED_REGEX.captures(&line_str).unwrap();
        let seeds = parse_numbers(captures.name("seeds").unwrap().as_str());
        let _skip_empty_line = it.next();
        return Some(seeds);
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
            ranges.push(Range {
                source_start: parts[1],
                dest_start: parts[0],
                length: parts[2]
            });
        }
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

fn apply_range_map(map: &RangeMap, value: u64) -> u64 {
    let mut result = value;
    for range in &map.ranges {
        if value >= range.source_start && value < range.source_start + range.length {
            result = range.dest_start + (value - range.source_start);
            break;
        }
    }
    result
}

fn apply_range_maps(maps: &Vec<RangeMap>, value: u64) -> u64 {
    let mut result = value;
    for map in maps {
        result = apply_range_map(map, result);
    }
    result
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut lines = input.lines();
        let seeds = parse_seeds(&mut lines).expect("Expected seeds");
        let maps = parse_maps(&mut lines);

        let mapped_seeds = seeds.iter().map(|seed| apply_range_maps(&maps, *seed));

        let least_location = mapped_seeds.min().unwrap();
        println!("Least location: {}", least_location);
    }
};