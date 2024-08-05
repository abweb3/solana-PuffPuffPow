# Solana Itus

Solana Itus is a decentralized application built on the Solana blockchain. This project includes various functionalities such as token initialization, public sales, epoch settlement, reward distribution, and reward claiming.

## Table of Contents

- [Solana Itus](#solana-itus)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [1. Build and Deploy the Program](#1-build-and-deploy-the-program)
    - [2. Running Tests](#2-running-tests)
  - [Program Functions](#program-functions)
  - [Testing](#testing)
  - [Contributing](#contributing)
  - [License](#license)

## Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed [Node.js](https://nodejs.org/en/download/) and [npm](https://www.npmjs.com/get-npm).
- You have installed [Rust](https://www.rust-lang.org/tools/install).
- You have installed [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools).
- You have installed [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html).

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/solana-itus.git
    cd solana-itus
    ```

2. Install dependencies:
    ```bash
    npm install
    ```

## Usage

### 1. Build and Deploy the Program

Before you can interact with the program, you need to build and deploy it.

1. **Start the Local Solana Validator**:
    ```bash
    solana-test-validator
    ```

2. **Build the Program**:
    ```bash
    anchor build
    ```

3. **Deploy the Program**:
    ```bash
    anchor deploy
    ```

### 2. Running Tests

To ensure everything is set up correctly and functioning as expected, run the test suite:

```bash
anchor test
