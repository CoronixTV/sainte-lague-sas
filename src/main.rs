use std::{collections::HashMap, fmt::Debug};
fn main() {
    let parties = vec![
        Party {
            name: "X".to_string(),
            votes: 14921877,
            seats: None,
        },
        Party {
            name: "Y".to_string(),
            votes: 11252215,
            seats: None,
        },
        Party {
            name: "Z".to_string(),
            votes: 3755699,
            seats: None,
        },
        Party {
            name: "Q".to_string(),
            votes: 3694057,
            seats: None,
        },
        Party {
            name: "M".to_string(),
            votes: 3242569,
            seats: None,
        },
    ];

    let seats = 631;
    let total_votes: u32 = parties.iter().map(|p| p.votes).sum::<u32>();
    let divisor = total_votes as f32 / seats as f32;
    calculate(parties, seats, divisor);
}

#[derive(Debug)]
struct Party {
    name: String,
    votes: u32,
    seats: Option<u32>,
}

fn calculate(distribution: Vec<Party>, seats: u32, divisor: f32) {
    let mut updated_distribution: Vec<Party> = Vec::new();
    for mut party in distribution {
        party.seats = Some((party.votes as f32 / divisor as f32).round() as u32);
        updated_distribution.push(party);
    }
    let total_seats: u32 = updated_distribution
        .iter()
        .map(|p| p.seats.unwrap())
        .sum::<u32>();
    println!("Total seats occupied: {}", total_seats);
    println!("Total seats available: {}", seats);
    if total_seats < seats {
        let new_divisor: f32 = divisor - 0.01;
        calculate(updated_distribution, seats, new_divisor);
        return
    }
    if total_seats > seats {
        let new_divisor: f32 = divisor + 0.01;
        calculate(updated_distribution, seats, new_divisor);
        return
    }
    if total_seats == seats {
        dbg!("{:?}", updated_distribution);
    }
}


