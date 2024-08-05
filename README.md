Solana Itus
Solana Itus is a decentralized application built on the Solana blockchain. This project includes various functionalities such as token initialization, public sales, epoch settlement, reward distribution, and reward claiming.

Table of Contents
Solana Itus
Table of Contents
Prerequisites
Installation
Usage
1. Build and Deploy the Program
2. Running Tests
Program Functions
Testing
Contributing
License
Prerequisites
Before you begin, ensure you have met the following requirements:

You have installed Node.js and npm.
You have installed Rust.
You have installed Solana CLI.
You have installed Anchor CLI.
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/your-username/solana-itus.git
cd solana-itus
Install dependencies:

bash
Copy code
npm install
Usage
1. Build and Deploy the Program
Before you can interact with the program, you need to build and deploy it.

Start the Local Solana Validator:

bash
Copy code
solana-test-validator
Build the Program:

bash
Copy code
anchor build
Deploy the Program:

bash
Copy code
anchor deploy
2. Running Tests
To ensure everything is set up correctly and functioning as expected, run the test suite:

bash
Copy code
anchor test
Program Functions
The Solana Itus program includes several key functions:

initialize: Initializes the state of the program.
initialize_tokens: Initializes the tokens.
initialize_pools: Initializes the liquidity pools.
public_sale: Handles the public sale of tokens.
settle_epoch: Settles the current epoch and updates the state.
distribute_rewards: Distributes rewards to the reward account.
claim_rewards: Allows users to claim their rewards for a given epoch.
Testing
The project includes comprehensive tests to ensure all functionalities work correctly. The tests are located in the tests directory and cover:

State initialization.
Voting for epoch duration.
Public sale operations.
Epoch settlement.
Reward distribution.
Reward claiming.
To run the tests, simply execute:

bash
Copy code
anchor test
Contributing
Contributions are always welcome!

Fork the repository.
Create your feature branch (git checkout -b feature/your-feature).
Commit your changes (git commit -am 'Add some feature').
Push to the branch (git push origin feature/your-feature).
Create a new Pull Request.
License
This project is licensed under the MIT License - see the LICENSE file for details.

