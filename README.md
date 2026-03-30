# rust-make-a-market
This is a fun Market Making Game for the simple simulation of market making, expected value, and probability scenarios. The current version has simple coin, dice, and cards expected value problems with other rational agent players. I plan to implement fun, higher difficulty versions of each game in the future to make it more interesting!

# How to Play

### 1. Clone the repository:
   ```bash
   git clone https://github.com/sqxiao000/rust-make-a-market.git
   cd rust-make-a-market
   ```

### 2. Run the game:
   ```bash
   cargo run
   ```

### 3. Choose a game!
   When prompted, select one of the available games (more to come!):
   - `coin` 
   - `dice`
   - `cards`

### 4. Set your bid/ask spread:
   Enter your bid price. Your ask price is automatically calculated based on your bid and the spread, each game has a fixed spread (for now!). This will be changed later on to have the player set their own spread, a too tight spread can be risky and a too wide spread can result in missing out on trades. 

### 5. Trade with rational players:
   Watch as rational players trade with you based on their own expected value calculations, derived from their own information. The goal is to make profitable trades by accurately pricing uncertainty!

## Requirements

- Rust >= 1.70
- Cargo
