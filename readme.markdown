# Passphrase Guesser

A toy project to experiment with how a passphrase attack fairs when using word lists that differ slightly with the list the user used to generate the passphrase.

## What it does

It creates 1 million 3-word passphrases from a short word list of 4 words: "news", "paper", "elephant", "music". These represent the user's possible passphrases.

Then it compares the efficiency of of a few different attack procedures with a key difference: In half the attacks, the adversary uses the user's exact word list, and in the other half of attacks, the adversary uses a super-set word list of the user's word list ("news", "paper", "elephant", "music" and "newspaper"). 



## Preliminary results/output

* Using user's exact word list and guessing randomly: Over 1M cracks, mean number of guesses was 63.945233
* Using a super-set word list and guessing randomly: Over 1M cracks, mean number of guesses was 125.058919

* Using user's word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was 32.4776
* Using a super-set word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was 47.4764

* Using user's word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was 48.5008. 12.4959% of cracks took fewer than 25 guesses.
* Using a super-set word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was 68.6457. 12.4704% of cracks took fewer than 25 guesses.

