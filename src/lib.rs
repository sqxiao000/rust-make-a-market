pub mod lib {
    extern crate rand;
    use rand::thread_rng;
    use rand::Rng;
    use rand::seq::SliceRandom;

    pub fn setup_game(input: &String) -> (i32, i32, Vec<i32>, f64, i32, i32) {

        let mut output: i32 = 0;
        let mut x_value: i32 = 0;
        let mut other_players: Vec<i32> = Vec::new();
        let mut spread: f64 = 0.0;
        let mut upper_bound: i32 = 0;
        let mut lower_bound: i32 = 0;

        if input.trim() == "coin" {
            println!("");
            println!("You chose the coin game.");
            println!("");
            println!("In this game, there are 9 other players. An unfair coin has an X% chance of landing on heads. X is an integer.");
            println!("For each player, the coin is flipped 10 times, and the number of heads is recorded and shared with that player.");
            println!("");
            let mut rng = thread_rng();
            x_value = rng.gen_range(0..100); // Generate a random integer (%) between 0 and 100 for the unfair coin's bias
            spread = 10.0; // Fixed spread for the coin game
            lower_bound = 0; // Minimum number of heads is 0
            upper_bound = 100; // Maximum number of heads is 10
            // Simulate the coin flips for the player and the other players
            for _ in 0..10 {
                let flip: f64 = rng.r#gen(); // Generate a random float between 0 and 1
                if flip < (x_value as f64 / 100.0) {
                    output += 1; // Increment heads count if the flip is less than x%
                }  
            }
            for _ in 0..9 {
                let mut other_heads_count: i32 = 0;
                for _ in 0..10 {
                    let flip: f64 = rng.r#gen(); // Generate a random float between 0 and 1
                    if flip < (x_value as f64 / 100.0) {
                        other_heads_count += 1; // Increment heads count if the flip is less than x%
                    }
                }
                other_players.push(other_heads_count);
            }
            println!("Your number of heads: {output}");
            println!("");
            println!("Make a market on X: the bias for heads on the coin for the other players to trade on. Fixed spread: 10");

        } else if input.trim() == "dice" {
            println!("");
            println!("You chose the dice game.");
            println!("");
            println!("In this game, there are 9 other players. A fair dice is rolled 10 times. X is the sum of the 10 rolls.");
            println!("For each player, the rolled number is recorded and shared with that player.");
            println!("");
            let mut rng = thread_rng();
            spread = 6.0; // Fixed spread for the dice game
            lower_bound = 10; // Minimum sum of 10 rolls is 10 (if all rolls are 1)
            upper_bound = 60; // Maximum sum of 10 rolls is 60 (if

            for i in 0..10 {
                let roll: i32 = rng.gen_range(1..=6); // Generate a random integer between 1 and 6 for the dice roll
                x_value += roll; // Add the roll to the player's total
                if i == 0 {
                    output = roll; // The first roll is the output that the player sees
                }
                else {
                    other_players.push(roll); // The other rolls are not revealed to the player
                }
            }
    
            println!("Your roll: {output}");
            println!("");
            println!("Make a market on X: the sum of the 10 rolls for the other players to trade on. Fixed spread: 6");

        } else if input.trim() == "cards" {
            println!("");
            println!("You chose the cards game.");
            println!("");
            println!("In this game, there are 9 other players. 10 cards are drawn from a 52-card deck without replacement. X is the sum of the 10 cards drawn.");
            println!("Each player is dealt one of the 10 cards.");
            println!("");
            let mut rng = thread_rng();
            spread = 10.0; // Fixed spread for the cards game
            lower_bound = 18; // Minimum sum of 10 cards is 18
            upper_bound = 122; // Maximum sum of 10 cards is 122

            let mut deck: Vec<i32> = (1..=13).flat_map(|x| std::iter::repeat(x).take(4)).collect();
            deck.shuffle(&mut rng);
            for i in 0..10 {
                let card = deck.pop().unwrap(); // Pop first 10 cards from the shuffled deck
                x_value += card; 
                if i == 0 {
                    output = card; // The first card is given to user
                }
                else {
                    other_players.push(card);
                }
            }

            println!("Your card: {output}");
            println!("");
            println!("Make a market on X: the sum of the 10 cards drawn for the other players to trade on. Fixed spread: 10");

        } else {
            println!("Invalid input. Please choose either 'coin', 'dice', or 'cards'.");
        }
        // Return
        (output, x_value, other_players, spread, lower_bound, upper_bound)
    }
}