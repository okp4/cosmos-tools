# 🪐 Cosmos tools

[![version](https://img.shields.io/github/v/release/okp4/cosmos-tools?style=for-the-badge)](https://github.com/okp4/cosmos-tools/releases)
[![build](https://img.shields.io/github/workflow/status/okp4/cosmos-tools/Build?label=build&style=for-the-badge)](https://github.com/okp4/cosmos-tools/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/workflow/status/okp4/cosmos-tools/Lint?label=lint&style=for-the-badge)](https://github.com/okp4/cosmos-tools/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/workflow/status/okp4/cosmos-tools/Test?label=test&style=for-the-badge)](https://github.com/okp4/cosmos-tools/actions/workflows/test.yml)
[![codecov](https://img.shields.io/codecov/c/github/okp4/cosmos-tools?style=for-the-badge&token=K5CYM8TQQY)](https://codecov.io/gh/okp4/cosmos-tools)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

Rust command line application with multiple tools to manage cosmos blockchain.

## 💰 Vesting

### Generate cliff

An utility to generate the JSON configuration genesis file of a periodic vesting including a cliff.

#### ❓ Why ?

On a cosmos blockchain, it's possible to create multiple type of vesting account at the start of the chain and during chain life.

- [`ContinuousVestingAccount`](https://docs.cosmos.network/master/modules/auth/05_vesting.html#continuousvestingaccount) : Continuous vesting, where coins begin to vest at a start time and vest linearly with respect to time until an end time is reached
- [`DelayedVestingAccount`](https://docs.cosmos.network/master/modules/auth/05_vesting.html#delayedvestingaccount) : Delayed vesting, where all coins are vested once an end time is reached.
- [`PeriodicVestingAccount`](https://docs.cosmos.network/master/modules/auth/05_vesting.html#periodicvestingaccount) : Periodic vesting, where coins begin to vest at a start time and vest periodically according to number of periods and the vesting amount per period. The number of periods, length per period, and amount per period are configurable. A periodic vesting account is distinguished from a continuous vesting account in that coins can be released in staggered tranches. For example, a periodic vesting account could be used for vesting arrangements where coins are relased quarterly, yearly, or over any other function of tokens over time.
- ...

But there is a missing vesting mode : a continuous vesting account with a cliff. This means that there should be a mode where the vesting starts in a linear way on a given date but that the funds are available only after a certain time (cliff), including an end time like ContinuousVestingAccount.

A temporary solution is to create a periodic vesting account with a relatively low interval so that the funds are available in the most linear way possible. To do that we need to generate a json file that including all vesting period. A generator is necessary to calculate automatically the first vesting amount after the cliff and calculate all vesting period by given an interval.

#### 📄 How to use

##### Command `vesting generate-cliff`

```cli
cosmos_tool vesting generate-cliff --help
```

```cli
Generate a JSON file containing all vesting periods based on interval and cliff duration configured

USAGE:
    cosmos_tools vesting generate-cliff [OPTIONS] --interval <INTERVAL> --duration <DURATION> <TOTAL_AMOUNT>

ARGS:
    <TOTAL_AMOUNT>    The total amount of token to vest

OPTIONS:
    -c, --cliff <CLIFF_DURATION>    Cliff duration (in seconds), if not filled, vesting start
                                    immediately [default: 0]
    -d, --duration <DURATION>       The total duration of vesting (in seconds)
        --denom <DENOM>             Configure the token denom used into json configuration
    -h, --help                      Print help information
    -i, --interval <INTERVAL>       The period interval (in second) which amount is split
    -o, --output <OUTPUT>           The path to the output file where JSON will be write, if not
                                    filled, json will be write on stdout
```

##### Exemple

To generate a vesting account with a 2 years vesting (63 072 000 seconds), a total amount of 40 000, a 6-month cliff (15 768 000 seconds) and a distribution with an interval of 1 day (86 400 seconds). The command will look like this :

```cli
cosmos_tools vesting generate-cliff 40000 --duration 63072000 --interval 86400 --cliff 1576800
```

```json
[
  {
    "length": "15811200",
    "amount": {
      "denom": "uknow",
      "amount": "10027"
    }
  },
  {
    "length": "15897600",
    "amount": {
      "denom": "uknow",
      "amount": "55"
    }
  },
  {
    "length": "15984000",
    "amount": {
      "denom": "uknow",
      "amount": "54"
    }
  },
 ...
]
```

With a linear vesting after 6 month, the first distribution is 10 027uknow. And after 6 month, a distribution is done every day (1 day interval).

## ⚙️ Prerequisites

Be sure you have [Rust](https://www.rust-lang.org/tools/install) properly installed with [cargo-make](https://github.com/sagiegurari/cargo-make).

## Build

```sh
cargo make
```
