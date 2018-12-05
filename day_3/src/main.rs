use std::{
    env,
    fs::File,
    io::prelude::*,
    str::FromStr,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()).unwrap());
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum ParseClaimError {
    MissingHash,
    MissingAt,
    MissingComma,
    MissingColon,
    MissingX,
    MisorderedSeparators,
    InvalidId,
    InvalidLeft,
    InvalidTop,
    InvalidWidth,
    InvalidHeight,
}

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let marker_hash = s.find("#").ok_or(ParseClaimError::MissingHash)?;
        let marker_at = s.find("@").ok_or(ParseClaimError::MissingAt)?;
        let marker_comma = s.find(",").ok_or(ParseClaimError::MissingComma)?;
        let marker_colon = s.find(":").ok_or(ParseClaimError::MissingColon)?;
        let marker_x = s.find("x").ok_or(ParseClaimError::MissingX)?;

        if marker_hash < marker_at && marker_at < marker_comma && marker_comma < marker_colon && marker_colon < marker_x {
            Ok(Claim {
                id: s[marker_hash+1..marker_at].trim().parse().map_err(|_| ParseClaimError::InvalidId)?,
                left: s[marker_at+1..marker_comma].trim().parse().map_err(|_| ParseClaimError::InvalidLeft)?,
                top: s[marker_comma+1..marker_colon].trim().parse().map_err(|_| ParseClaimError::InvalidTop)?,
                width: s[marker_colon+1..marker_x].trim().parse().map_err(|_| ParseClaimError::InvalidWidth)?,
                height: s[marker_x+1..].trim().parse().map_err(|_| ParseClaimError::InvalidHeight)?,
            })
        } else {
            Err(ParseClaimError::MisorderedSeparators)
        }
    }
}

const CLOTH_WIDTH: usize = 1000;
const CLOTH_HEIGHT: usize = 1000;

fn claim_cloth<'a>(claims: impl Iterator<Item = &'a Claim>) -> Vec<u8> {
    let mut cloth = Vec::new();
    cloth.resize(CLOTH_WIDTH * CLOTH_HEIGHT, 0u8);

    for claim in claims {
        for x in claim.left..claim.left+claim.width {
            for y in claim.top..claim.top+claim.height {
                cloth[x + y * CLOTH_WIDTH] += 1;
            }
        }
    }

    cloth
}

fn part_1(data: &str) -> usize {
    let claims: Vec<_> = data.lines().map(|c| c.parse::<Claim>().unwrap()).collect();
    claim_cloth(claims.iter()).iter().filter(|&&x| x > 1).count()
}

fn part_2(data: &str) -> Option<usize> {
    let claims: Vec<_> = data.lines().map(|c| c.parse::<Claim>().unwrap()).collect();
    let cloth = claim_cloth(claims.iter());

    for claim in claims.iter() {
        let mut overlaps = false;
        'outer: for x in claim.left..claim.left+claim.width {
            for y in claim.top..claim.top+claim.height {
                if cloth[x + y * CLOTH_WIDTH] != 1 {
                    overlaps = true;
                    break 'outer;
                }
            }
        }
        if !overlaps {
            return Some(claim.id);
        }
    }

    None
}