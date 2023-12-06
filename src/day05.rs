use std::cmp::min;
use std::collections::{BTreeMap, HashMap};

pub fn part_one(data: &str) -> u64 {
    // Split the data into blocks remove the trailing whitespaces
    let almanac_blocks: Vec<&str> = data.split("\n\n").map(|s| s.trim()).collect();
    // Extract the seeds from the first block
    let seeds = extract_seeds(&almanac_blocks);
    // Extract the almanac entries from the remaining blocks
    let mappings = extract_almanac(&almanac_blocks);

    // Find the minimum location for each seed
    seeds.iter()
        // For each seed, find the location
        .map(|&seed| get_location_for_seed(seed, &mappings))
        // Find the minimum location
        .min()
        .expect("There should be at least one seed")
}

// This is a brute force solution that works for the example but is very slow for the real input
// It is kept here for reference and to compare with the optimized solution below
// More than 3 hours on my machine for 2_333_037_642 seeds
pub fn part_two_brut_force(data: &str) -> u64 {
    let almanac_blocks: Vec<&str> = data.split("\n\n").map(|s| s.trim()).collect();
    let seeds = extract_seeds_from_ranges(&almanac_blocks);
    let mappings = extract_almanac(&almanac_blocks);

    seeds.iter()
        .map(|&(start, end)| {
            (start..=end)
                .map(|seed| get_location_for_seed(seed, &mappings))
                .min()
                .unwrap_or(u64::MAX)
        })
        .min()
        .unwrap_or(u64::MAX)
}

pub fn part_two(data: &str) -> u64 {
    let almanac_blocks: Vec<&str> = data.split("\n\n").map(|s| s.trim()).collect();
    let seeds = extract_seeds_from_ranges(&almanac_blocks);
    let mappings = extract_almanac(&almanac_blocks);

    get_locations_for_seed_ranges(seeds, &mappings)
        .into_iter()
        .min_by_key(|&(start, _)| start)
        .map(|(start, _)| start)
        .unwrap_or(u64::MAX) // or a more appropriate default/error handling
}

struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Mapping {
    // Check if the value is in the source range
    fn in_range(&self, value: u64) -> bool {
        value >= self.source_range_start && value < self.source_range_start + self.range_length
    }

    // Map the value from the source range to the destination range
    fn map_value(&self, value: u64) -> u64 {
        self.destination_range_start + (value - self.source_range_start)
    }
}

struct AlmanacEntry {
    source: String,
    destination: String,
    mappings: BTreeMap<u64, Mapping>,
}

impl AlmanacEntry {
    fn get_mapping(&self, value: u64) -> u64 {
        let potential_match = self.mappings.range(..=value).next_back();
        if let Some((_, mapping)) = potential_match {
            if mapping.in_range(value) {
                return mapping.map_value(value);
            }
        }
        value
    }

    fn get_mapping_for_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        let mut start = range.0;

        while start <= range.1 {
            let floor_mapping = self.mappings.range(..=start).next_back().map(|(&k, v)| (k, v));
            if let Some((_, mapping)) = floor_mapping {
                if mapping.in_range(start) {
                    let end = min(mapping.source_range_start + mapping.range_length - 1, range.1);
                    result.push((mapping.map_value(start), mapping.map_value(end)));
                    start = end + 1;
                } else {
                    let ceil_mapping = self.mappings.range(start..).next().map(|(&k, v)| (k, v));
                    let end = match ceil_mapping {
                        Some((key, _)) => min(range.1, key - 1),
                        None => range.1,
                    };
                    result.push((start, end));
                    start = end + 1;
                }
            } else {
                result.push((start, range.1));
                break;
            }
        }

        result
    }
}

fn get_location_for_seed(seed: u64, almanac: &HashMap<String, AlmanacEntry>) -> u64 {
    let mut value = seed;
    let mut type_key = "seed".to_string();

    while type_key != "location" {
        if let Some(almanac_entry) = almanac.get(&type_key) {
            value = almanac_entry.get_mapping(value);
            type_key = almanac_entry.destination.clone();
        } else {
            // Handle the case where the almanac entry is not found
            panic!("Almanac entry for type '{}' not found", type_key);
        }
    }

    value
}

fn get_locations_for_seed_ranges(seed: Vec<(u64, u64)>, almanac: &HashMap<String, AlmanacEntry>) -> Vec<(u64, u64)> {
    let mut value = seed;
    let mut type_key = "seed".to_string();

    while type_key != "location" {
        if let Some(almanac_entry) = almanac.get(&type_key) {
            value = value.into_iter()
                .flat_map(|range| almanac_entry.get_mapping_for_range(range))
                .collect();
            type_key = almanac_entry.destination.clone();
        } else {
            // Handle the case where the almanac entry is not found
            panic!("Almanac entry for type '{}' not found", type_key);
        }
    }

    value
}

fn extract_almanac(blocks: &Vec<&str>) -> HashMap<String, AlmanacEntry> {
    let mut almanac_entries = HashMap::new();
    for block in blocks.iter().skip(1) {
        let lines: Vec<&str> = block.split('\n').collect();

        let replaced_line = lines[0].replace(" map:", "");
        let parts: Vec<&str> = replaced_line.split("-to-").collect();
        let source = parts[0].to_string();
        let destination = parts[1].to_string();

        let mappings: BTreeMap<u64, Mapping> = lines.iter().skip(1).map(|line| {
            let parts: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let mapping = Mapping {
                destination_range_start: parts[0],
                source_range_start: parts[1],
                range_length: parts[2],
            };
            (mapping.source_range_start, mapping)
        }).collect();

        let almanac_entry = AlmanacEntry {
            source: source.to_string(),
            destination: destination.to_string(),
            mappings,
        };
        almanac_entries.insert(almanac_entry.source.clone(), almanac_entry);
    }
    almanac_entries
}

// Convert Kotlin's `extractSeeds` method to Rust
fn extract_seeds(blocks: &Vec<&str>) -> Vec<u64> {
    blocks.first().unwrap()
        .replace("seeds: ", "")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// Convert Kotlin's `extractSeedsFromRanges` method to Rust
fn extract_seeds_from_ranges(blocks: &Vec<&str>) -> Vec<(u64, u64)> {
    blocks.first().unwrap()
        .replace("seeds: ", "")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    static TRAINING: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part_one_examples() {
        assert_eq!(35, part_one(TRAINING));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(46, part_two(TRAINING));
    }

    #[test]
    fn part_two_brut_force_examples() {
        assert_eq!(46, part_two_brut_force(TRAINING));
    }
}



