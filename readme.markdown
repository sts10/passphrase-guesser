# Passphrase Guesser

A toy project to experiment with how a passphrase attack fairs when using word lists that differ slightly with the user.

## What it does

It creates 1 million 3-word passphrases from a short word list of 4 words: "news", "paper", "elephant", "music". These represent the user's possible passphrases.

Then it compares the efficiency of two distinct attacks: One in which the attacker randomly chooses 3 words from the user's exact word list, and another in which the attack randomly chooses from a super-set word list of: "news", "paper", "elephant", "music" and "newspaper". 

## Preliminary results/output

```
Using user's exact word list and guessing randomly: Over 1M cracks, mean number of guesses was 64.006169
Using a super-set word list and guessing randomly: Over 1M cracks, mean number of guesses was 125.015747
---------------------------------
Using user's word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was 32.552266
Using a super-set word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was 47.542548
---------------------------------
Using user's word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was 48.485533
Using a super-set word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was 68.710858
```
