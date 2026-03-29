mod lib;
use std::io;
use crate::lib::lib::setup_game;

fn main() {
    
    // Introduction and Setup
    println!("Welcome to Make A Market! A probabilistic market making game.");
    println!("");
    println!("Pick your poison:");
    println!("");
    println!("> coin");
    println!("> dice");
    println!("> cards");
    println!("");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Setup game parameters and trial information
    let (user_info, x_value, other_players, spread, lower_bound, upper_bound) = setup_game(&input);
    
    println!("");
    println!("Set your bid:");
    let mut bid = String::new();
    io::stdin()
        .read_line(&mut bid)
        .expect("Failed to read line");
    let bid_value = bid.trim().parse::<f64>().expect("Please enter a valid number");
    let ask_value: f64 = bid_value + spread; // Fixed spread

    if bid_value < lower_bound as f64 || ask_value > upper_bound as f64 {
        println!("Your bid and ask must be within the bounds of {} and {}", lower_bound, upper_bound);
        return;
    }

    println!("Your bid: {bid_value}");
    println!("Your ask: {ask_value}");
    println!("");
    println!("Trading with other players...");
    println!("");

    // Every other player has rational fair value based on their own information and expected value
    let mut other_players_fair_values: Vec<f64> = Vec::new();
    let mut value: f64 = 0.0;
    for (i, other_player_value) in other_players.iter().enumerate() {
        if input.trim() == "coin" {
            value = *other_player_value as f64 * 10.0; //  because using bps
        } else if input.trim() == "dice" {
            value = *other_player_value as f64 + 3.5*9.0;
        } else if input.trim() == "cards" {
            value = *other_player_value as f64 + (364.0-*other_player_value as f64)*9.0/51.0;
        }
        other_players_fair_values.push(value);
    }

    let mut fair_price: f64 = 0.0;
    fair_price = x_value as f64;

    let mut pnl: f64 = 0.0;
    let mut share_position: i32 = 0;
    for (i, players_value) in other_players_fair_values.iter().enumerate() {
        if (bid_value >= *players_value) {
            println!("Bought from Player {} at price {}", i + 1, bid_value);
            share_position += 1;
            pnl -= bid_value - fair_price;
        } else if (ask_value <= *players_value) {
            println!("Sold to Player {} at price {}", i + 1, ask_value);
            share_position -= 1;
            pnl += ask_value - fair_price;
        } else {
            println!("No trade executed with Player {}", i + 1);
        }
    }

    println!("");
    println!("Your final position: {} shares", share_position);
    println!("");
    if fair_price <= ask_value && fair_price >= bid_value {
        println!("Actual value of X: {} is between your bid and ask. You have a good market!", x_value);
    } else if fair_price < bid_value {
        println!("Actual value of X: {} is below your bid.", x_value);
    } else if fair_price > ask_value {
        println!("Actual value of X: {} is above your ask.", x_value);
    }
    println!("");
    println!("Your final P&L: ${:.2}", pnl);
    if pnl > 0.0 {
        println!("Congratulations! You made a profit!");
    } else if pnl <= 0.0 {
        println!("Better luck next time!");
    }
    println!("");
    println!("Thanks for playing! Goodbye.");
}
