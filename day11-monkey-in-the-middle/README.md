# Advent of Code - Day 11: Monkey in the Middle
See the question for more details: https://adventofcode.com/2022/day/11

---

## Thinking about modulos of really big numbers for part two...

As each test the monkey uses is simply whether a number is divisible, all we really need to know is its congruency not the full number itself. The ideas I learned from disrete math around modular arithmetic seems like they'd help with this. In this case, we only have two operations that happen during inspect are add and multiply where the result can be determined with the following:
- Addition:       `(A + B) mod C = (A mod C + B mod C) mod C`
- Multiplication: `(A * B) mod C = (A mod C * B mod C) mod C`
- Theres also kind of exponentiation as we can have multiplication that uses itself to square the input

### Let's think through an example:

> Want to see whether `79 * 19 ≡ 0 mod 23` is true or not, that is if 79 * 19 is congruent to 0 mod 23, or yet more words if `79 * 19` is divisible by `23`.
>
> We can write this as 
> - aka:         `(79 * 19) mod 23 == 0`
> - can become:  `(79 mod 23 * 19 mod 23) mod 23`

What we can gather from this example is that we will need to record the chain of operations made so we can play it back for a given divisor test - a given mod n.

> Continuing the example, this item next gets applied the operation: `old + 3`
> 
> So the operations so far are (starting with base value) `["* 19", "+ 3"]`
> 
> This seems like a good use of fold...
> - Original: `((79 * 19) + 3) mod 17`
> - Addition: `((79 * 19) mod 17 + 3 mod 17) mod 17`
> - Multiply: `((79 mod 17 * 19 mod 17) mod 17 + 3 mod 17) mod 17`

So yup, let's chain the operations!
