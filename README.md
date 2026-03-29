# rust-make-a-market
This is a fun Market Making Probability Game for the simple simulation of market making, expected value, and probability games. The current version has simple coin, dice, and cards expected value scenarios with other rational agent players. I plan to implement more different and fun version of each game to make it more interesting in the future!

# How to Play

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/rust-make-a-market.git
   cd rust-make-a-market
   ```

2. **Run the game:**
   ```bash
   cargo run
   ```

3. **Choose a game!**
   When prompted, select one of the available games (more to come!):
   - `coin` 
   - `dice`
   - `cards`

4. **Set your bid/ask spread:**
   Enter your bid price. Your ask price is automatically calculated based on your bid and the spread, each game has a fixed spread. This will be changed later on to have the player set their own spread, a too tight spread can be risky and a too wide spread can result in missing out on trades. 

5. **Trade with rational players:**
   Watch as rational players trade with you based on their own expected value calculations. The goal is to make profitable trades by accurately pricing uncertainty!

## Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)