use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PersonPreferences {
    pub name: String,
    pub preferences: HashMap<String, i32>  // preference_type -> value (1-5)
}

// #[derive(Debug)]
// pub struct PreferenceMatch {
//     pub person1: String,
//     pub person2: String,
//     pub compatibility_score: f64,
// }

pub fn calculate_compatibility(person1: &PersonPreferences, person2: &PersonPreferences) -> f64 {
    let mut total_diff = 0.0;
    let mut counted_prefs = 0;

    // Compare shared preferences
    for (pref_type, value1) in &person1.preferences {
        if let Some(value2) = person2.preferences.get(pref_type) {
            total_diff += (value1 - value2).abs() as f64;
            counted_prefs += 1;
        }
    }

    if counted_prefs == 0 {
        return 0.0;
    }

    // Convert to a 0-1 scale where 1 is perfect match
    // Assuming preferences are on a 1-5 scale, maximum difference is 4
    1.0 - (total_diff / (counted_prefs as f64 * 4.0))
}

pub fn create_preference_rankings(people: &[PersonPreferences]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = people.len();
    let mut rankings_group1 = vec![vec![0; n]; n/2];
    let mut rankings_group2 = vec![vec![0; n]; n/2];

    // Split people into two groups
    let (group1, group2) = people.split_at(n/2);

    // Create rankings based on compatibility scores
    for (i, person1) in group1.iter().enumerate() {
        let mut scores: Vec<(usize, f64)> = group2
            .iter()
            .enumerate()
            .map(|(j, person2)| (j, calculate_compatibility(person1, person2)))
            .collect();
        
        // Sort by compatibility score (highest first)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Convert to ranking
        rankings_group1[i] = scores.iter().map(|(j, _)| *j).collect();
    }

    // Do the same for group2
    for (i, person2) in group2.iter().enumerate() {
        let mut scores: Vec<(usize, f64)> = group1
            .iter()
            .enumerate()
            .map(|(j, person1)| (j, calculate_compatibility(person2, person1)))
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        rankings_group2[i] = scores.iter().map(|(j, _)| *j).collect();
    }

    (rankings_group1, rankings_group2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_compatibility() {
        let person1 = PersonPreferences {
            name: "Alice".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 1),
                ("networking".to_string(), 4),
                ("formality".to_string(), 2),
            ]),
        };

        let person2 = PersonPreferences {
            name: "Bob".to_string(),
            preferences: HashMap::from([
                ("small_talk".to_string(), 2),
                ("networking".to_string(), 5),
                ("formality".to_string(), 1),
            ]),
        };

        let compatibility = calculate_compatibility(&person1, &person2);
        assert!(compatibility > 0.0 && compatibility < 1.0, "Compatibility should be between 0 and 1");
    }

    #[test]
    fn test_create_preference_rankings() {
        let people = vec![
            PersonPreferences {
                name: "Alice".to_string(),
                preferences: HashMap::from([
                    ("small_talk".to_string(), 1),
                    ("networking".to_string(), 4),
                    ("formality".to_string(), 2),
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

        let (rankings_group1, rankings_group2) = create_preference_rankings(&people);
        assert_eq!(rankings_group1.len(), 2, "There should be two rankings in group 1");
        assert_eq!(rankings_group2.len(), 2, "There should be two rankings in group 2");
    }
} 