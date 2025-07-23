# Airdrop Token Program

This project implements an airdrop mechanism on Solana using a Bloom filter to efficiently track which users have claimed their tokens. The Bloom filter allows for fast membership checks with minimal storage overhead, making it ideal for on-chain use.

## What is a Bloom Filter?

A Bloom filter is a probabilistic data structure used to test whether an element is a member of a set. It can quickly tell you if an item is *definitely not* in the set or *possibly* in the set (with a small false positive rate). Bloom filters are space-efficient and do not store the actual items, only a bit array and a set of hash functions.

### Bloom Filter Formula

Given:
- `n`: Number of expected elements to store
- `m`: Size of the bit array (in bits)
- `k`: Number of hash functions

The probability of a false positive (an element incorrectly reported as present) is:

```
p = (1 - e^(-kn/m))^k
```

Where:
- `e` is Euler's number (≈ 2.71828)
- `k` is typically chosen as `k = (m/n) * ln(2)` for optimal performance

### How It Works in This Program

- Each user’s public key is hashed with multiple hash functions.
- Each hash determines a bit position in the bit array to set.
- To check if a user has claimed, the program checks all corresponding bits; if any are unset, the user has not claimed.
- This approach prevents double claims and keeps on-chain storage minimal.

### False Positives and Poisson Distribution

A Bloom filter can return a false positive, meaning it may indicate that an address has already claimed tokens even if it has not. This occurs because multiple addresses can set overlapping bits in the filter. The probability of a false positive is closely related to the Poisson distribution, which describes the likelihood of a given number of events (bit settings) occurring in a fixed interval (the bit array).

#### Poisson Formula

The probability that a particular bit remains unset after all insertions is given by the Poisson formula:

```
P(X = 0) = e^{-λ}
```

Where:
- `λ = (k * n) / m`
- `k`: Number of hash functions
- `n`: Number of inserted addresses
- `m`: Size of the bit array

This means the probability that a bit is set is `1 - e^{-λ}`. For a query, the probability that all `k` bits are set (resulting in a false positive) is:

```
p = (1 - e^{-λ})^k
```

As more addresses are added, or as the bit array size decreases, the likelihood of false positives increases. This is a trade-off between storage efficiency and accuracy, and parameters should be chosen to keep the false positive rate acceptably low for your application.

## Usage

- Deploy the program to Solana.
- Call `initialize` to set up the Bloom filter.
- Users call `claim_airdrop` to claim tokens; the Bloom filter tracks claims.

## References

- [Bloom filter - Wikipedia](https://en.wikipedia.org/wiki/Bloom_filter)
- [Solana Anchor Framework](https://book.anchor-lang.com/)
