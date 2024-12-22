### WIP | Not working yet | Not tested | Not ready for production | Not ready for use

# Preference Matcher

This project implements a preference matching system using a variation of the Gale-Shapley algorithm. Instead of individuals making discrete choices, each person has a set of preferences represented by a slider (or scale) for different attributes. The system calculates compatibility scores based on these preferences and uses the Gale-Shapley algorithm to find stable matches.

## Files

- **src/main.rs**: Contains the main function that demonstrates how to use the preference matcher.
- **src/preference_matcher.rs**: Contains the logic for calculating compatibility scores and creating preference rankings.

## How It Works

### PersonPreferences

Each person is represented by the `PersonPreferences` struct, which includes a name and a set of preferences. Preferences are stored in a `HashMap` where the key is the preference type (e.g., "small_talk") and the value is an integer from 1 to 5 indicating the level of preference.

### Compatibility Calculation

The `calculate_compatibility` function computes a compatibility score between two people. It compares shared preferences and calculates the total difference. The score is then normalized to a 0-1 scale, where 1 indicates a perfect match.

### Preference Rankings

The `create_preference_rankings` function splits the list of people into two groups and calculates compatibility scores between each pair of individuals from different groups. It then sorts these scores to create rankings for each person.

### Gale-Shapley Algorithm

The Gale-Shapley algorithm is used to find stable matches between the two groups. This implementation uses the compatibility rankings instead of discrete choices. The algorithm iteratively matches individuals from one group to the other based on their rankings, ensuring that no pair of individuals would prefer each other over their current matches.

### Example Usage

In `src/main.rs`, an example usage is provided:

1. A list of people with their preferences is created.
2. Preference rankings are generated using the `create_preference_rankings` function.
3. The Gale-Shapley algorithm is applied to find stable matches.
4. The matches are printed to the console.

## Running the Code

To run the code, ensure you have Rust installed and execute the following command in the terminal:
```bash
cargo run
```

This will compile and run the program, displaying the stable matches found by the algorithm.

## Dependencies

- `galeshapley`: A crate that provides the Gale-Shapley algorithm implementation.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
