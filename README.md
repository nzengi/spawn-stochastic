# Stochastic Processes in Rust

This Rust library provides implementations for multiple stochastic processes, including:
- Arithmetic Brownian Motion (ABM)
- Geometric Brownian Motion (GBM)
- Ornstein-Uhlenbeck Process (OU)
- Feller Square Root Process
- Brownian Bridge Process

Each of these processes is commonly used in financial modeling, scientific simulations, and mathematical research.

## Features

This library includes:
- Multiple stochastic process models.
- Secure random number generation using `OsRng`.
- Rust idiomatic safety practices (boundary checks, secure error handling, etc.).
- Modular structure for ease of use and extendability.

## Installation

To include this library in your Rust project, add the following line to your `Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
stochastic_processes = "0.1.0"
```

## Usage
Arithmetic Brownian Motion (ABM)

```rust
use stochastic_processes::abm::ArithmeticBrownianMotion;

fn main() {
    let abm = ArithmeticBrownianMotion::new(0.05, 0.4, 50, 200, 1.0, 200.0);
    let paths = abm.simulate();
    
    for path in paths {
        println!("{:?}", path);
    }
}
```

## Geometric Brownian Motion (GBM)

```rust
use stochastic_processes::gbm::GeometricBrownianMotion;

fn main() {
    let gbm = GeometricBrownianMotion::new(0.2, 0.4, 50, 200, 1.0, 500.0);
    let paths = gbm.simulate();
    
    for path in paths {
        println!("{:?}", path);
    }
}
```

## Ornstein-Uhlenbeck Process (OU)

```rust
use stochastic_processes::ou::OrnsteinUhlenbeck;

fn main() {
    let ou = OrnsteinUhlenbeck::new(10.0, 0.07, 0.1, 50, 200, 1.0, 0.05);
    let paths = ou.simulate();
    
    for path in paths {
        println!("{:?}", path);
    }
}
```

## Feller Square Root Process

```rust
use stochastic_processes::feller::FellerSquareRoot;

fn main() {
    let feller = FellerSquareRoot::new(5.0, 0.1, 0.2, 50, 200, 1.0, 0.05);
    let paths = feller.simulate();
    
    for path in paths {
        println!("{:?}", path);
    }
}
```

## Brownian Bridge Process

```rust
use stochastic_processes::bridge::BrownianBridge;

fn main() {
    let bridge = BrownianBridge::new(0.0, 1.0, 0.2, 50, 200, 1.0);
    let paths = bridge.simulate();
    
    for path in paths {
        println!("{:?}", path);
    }
}
```








