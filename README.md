# a horribly ~~flawed~~ for fun version of the merkle-hellman knapsack cryptosystem

It does not work correctly, and is prone to overflowing and crashing. Its just
written to understand the math behind knapsack cryptosystems.

## fun and inconsequential math bits 

The cryptosystem is based on the subset sum problem - given a set S of integers
and a target sum T, find a subset of S that sums to T. This problem is
NP-complete, but is solvable with a simple greedy algorithm in polynomial time
if S is superincreasing. 

In the Merkle-Hellman cryptosystem, the private key W is transformed by
multiplying the elements within by r mod q to form the public key, B. The two
pieces of information, r and q, then act as a "trapdoor" to transform the hard
NP-complete problem of finding the subset sum with B, to an easy problem by
finding the subset sum with the superincreasing set W.
