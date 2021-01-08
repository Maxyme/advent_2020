// Passport verification exercise. Check that all mandatory fields are present in the entries.
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn validate_byr(byr: i32) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    byr >= 1920 && byr <= 2002
}

fn validate_iyr(iyr: i32) -> bool {
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    iyr >= 2010 && iyr <= 2020
}

fn validate_eyr(eyr: i32) -> bool {
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    eyr >= 2020 && eyr <= 2030
}

fn validate_hgt(hgt: i32, metric: bool) -> bool {
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    match metric {
        true => hgt >= 150 && hgt <= 193,
        false => hgt >= 59 && hgt <= 76,
    }
}

fn validate_hcl(hcl: &str) -> bool {
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let re = Regex::new(r"#([a-f0-9]{6})").unwrap();
    re.is_match(hcl)
}

fn validate_ecl(ecl: &str) -> bool {
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn validate_pid(pid: &str) -> bool {
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(pid)
}

fn validate_key_values(values: Vec<&str>) -> bool {
    // insert into hashmap
    let tags: HashMap<&str, &str> = values
        .into_iter()
        .map(|kv| kv.split(':').collect::<Vec<&str>>())
        .map(|vec| {
            assert_eq!(vec.len(), 2);
            (vec[0], vec[1])
        })
        .collect();

    let byr: i32 = tags["byr"].parse::<i32>().expect("Expect int");
    let iyr = tags["iyr"].parse::<i32>().expect("Expect int");
    let eyr = tags["eyr"].parse::<i32>().expect("Expect int");

    let digits_re = Regex::new(r"[0-9]+").unwrap();
    let hgt;
    match digits_re.captures(tags["hgt"]) {
        Some(v) => {
            hgt = v
                .get(0)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("Expect int")
        }
        None => return false,
    };

    let chars_re = Regex::new(r"[a-zA-Z]{2}").unwrap();
    let metric = {
        match chars_re.captures(tags["hgt"]) {
            Some(v) => v.get(0).unwrap().as_str() == "cm",
            None => return false,
        }
    };
    let hcl = tags["hcl"];
    let ecl = tags["ecl"];
    let pid = tags["pid"];
    let valid = validate_byr(byr)
        && validate_iyr(iyr)
        && validate_eyr(eyr)
        && validate_hgt(hgt, metric)
        && validate_hcl(hcl)
        && validate_ecl(ecl)
        && validate_pid(pid);
    valid
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");

    // Need to split on all empty lines
    let input_entries: Vec<_> = Regex::new(r"\n\n").unwrap().split(&f).collect::<Vec<_>>();

    let mandatory_keys = ["byr", "hgt", "ecl", "hcl", "pid", "iyr", "eyr"];
    let mut valid_passport_count_part_1 = 0;
    let mut valid_passport_count_part_2 = 0;
    for entry in input_entries {
        // split values by linebreaks and whitespace
        let values = entry.split_whitespace().collect::<Vec<_>>();
        let keys: Vec<_> = values
            .iter()
            .map(|x| x.split(":").collect::<Vec<_>>()[0])
            .collect();

        if mandatory_keys.iter().all(|&i| keys.contains(&i)) {
            valid_passport_count_part_1 += 1;
            if validate_key_values(values) {
                // only validate when all the mandatory keys are present
                valid_passport_count_part_2 += 1;
            }
        }
    }
    println!(
        "valid_passport_count part_1 {}",
        valid_passport_count_part_1
    );
    println!(
        "valid_passport_count part_2 {}",
        valid_passport_count_part_2
    );
}
