# Simple passphrase generator

A simple passphrase generator inspired by [this article](https://www.eff.org/dice) from [EFF](https://www.eff.org/), that lets you specify which one of these three wordlist to use:

* **long** corresponding to [EFF's Long Wordlist](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), for use with five dice
* **shortest** corresponding to [EFF's Short Wordlist #1](https://eff.org/files/2016/09/08/eff_short_wordlist_1.txt), featuring only short words, for use with four dice
* **short** corresponding to [EFF's Short Wordlist #2](https://eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt), for use with four dice, featuring longer words that may be more memorable.


Some examples of a generated passphrases with different wordlist:

* aroma algorithm attach concrete freeway endpoint
* chop bust fang shush drown cone
* imaging vagrancy cesspool oncoming silicon excerpt
## Usage

    Usage:
        passphrase_generator [OPTIONS]

    Passphrase generator that uses a word list and dices.

    optional arguments:
      -h,--help             show this help message and exit
      -v,--verbose          Be verbose
      -w,--wordlist WORDLIST
                            Word list to use (one of [long, short, shortest]), default is long
      -n,--length LENGTH    Length of the passphrase, default is 6
