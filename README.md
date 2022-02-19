# Wordle Solver

A CLI tool to help you solve Wordle riddles!

Syntax:

`+c` => Character exactly at position
`^c` => Some character, not in this position
`*c` => In word, but not in this position
`!c` => Not in any position

So something like:

![Picture of "Swiss" in Wordle, with S, W and I highlighted as green](doc/swiss.png)

Will look like:

`+s+w+i*s*s`

and something like:

![Picture of "Weary" in Wordle, with W highlighted as yellow](doc/weary.png)

Will look like:

`^w!e!a!r!y`
