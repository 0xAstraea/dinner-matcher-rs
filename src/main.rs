use crate::preference_matcher::{PersonPreferences, create_preference_rankings};
use std::collections::HashMap;
use galeshapley::GaleShapley;
mod preference_matcher;

fn main() {
    // Example usage
    let people = vec![
        PersonPreferences {
            name: "Alice".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 1),  // Prefers deep conversations
                ("networking".to_string(), 4),   // Prefers networking
                ("formality".to_string(), 2),    // Prefers casual
            ]),
        },
        PersonPreferences {
            name: "Bob".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 2),
                ("networking".to_string(), 5),
                ("formality".to_string(), 1),
            ]),
        },
        PersonPreferences {
            name: "Charlie".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 5),
                ("networking".to_string(), 1),
                ("formality".to_string(), 4),
            ]),
        },
        PersonPreferences {
            name: "Diana".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 4),
                ("networking".to_string(), 2),
                ("formality".to_string(), 5),
            ]),
        },
    ];

    // Create preference rankings
    let (rankings_group1, rankings_group2) = create_preference_rankings(&people);

    // Use the Gale-Shapley algorithm to find stable matches
    let mut matcher = GaleShapley::init(rankings_group1, rankings_group2);
    let matches = matcher.find_stable_marriage();

    // Print matches
    for (i, j) in matches {
        println!("Matched: {} with {}", people[i].name, people[j + people.len()/2].name);
    }
} 