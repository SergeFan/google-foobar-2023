## Google FooBar 2023 Programming Challenge in Rust

This repo contains Google FooBar 2023 Programming Challenge solutions written in Rust.

The solution of each challenge can be found in `src/lib` as individual module.

NOTE: Google FooBar 2023 Programming Challenge only accepts Java or Python code as solution, all solutions of this repo are only for personal training purpose.

### Challenge 1: Braille Translation

#### Explain
Because Commander Lambda is an equal-opportunity despot, they have several visually-impaired minions. But Lambda never bothered to follow intergalactic standards for workplace accommodations, so those minions have a hard time navigating her space station. You figure printing out Braille signs will help them, and -- since you'll be promoting efficiency at the same time -- increase your chances of a promotion.

Braille is a writing system used to read by touch instead of by sight. Each character is composed of 6 dots in a `2x3` grid, where each dot can either be a bump or be flat (no bump). You plan to translate the signs around the space station to Braille so that the minions under Commander Lambda's command can feel the bumps on the signs and "read" the text with their touch. The special printer which can print the bumps onto the signs expects the dots in the following order:

```
1 4
2 5
3 6
```


So given the plain text word `code`, you get the Braille dots:

```
1 1   1 0   1 1   1 0
0 0   0 1   0 1   0 1
0 0   1 0   0 0   0 0
```

where 1 represents a bump and 0 represents no bump. Put together, `code` becomes the output string `100100101010100110100010`.

Write a function solution (plaintext) that takes a string parameter and returns a string of 1's and 0's representing the bumps and absence of bumps in the input string. Your function should be able to encode the 26 lowercase letters, handle capital letters by adding a Braille capitalization mark before that character, and use a blank character (`000000`) for spaces. All signs on the space station are less than fifty characters long and use only letters and spaces.

#### Test Cases

Input:

```
solution.solution("The quick brown fox jumps over the lazy dog")
```

Output:

```
000001011110110010100010000000111110101001010100100100101000000000110000111010101010010111101110000000110100101010101101000000010110101001101100111100011100000000101010111001100010111010000000011110110010100010000000111000100000101011101111000000100110101010110110
```

Input:

```
solution.solution("code")
```

Output:

```
100100101010100110100010
```

Input:

```
solution.solution("Braille")
```

Output:

```
000001110000111010100000010100111000111000100010
```

## Level 2

### Challenge 1: Ion Flux Relabeling

#### Explain

Oh no! Commander Lambda's latest experiment to improve the efficiency of the LAMBCHOP doomsday device has backfired spectacularly. The Commander had been improving the structure of the ion flux converter tree, but something went terribly wrong and the flux chains exploded. Some of the ion flux converters survived the explosion intact, but others had their position labels blasted off. Commander Lambda is having her henchmen rebuild the ion flux converter tree by hand, but you think you can do it much more quickly -- quickly enough, perhaps, to earn a promotion!

Flux chains require perfect binary trees, so Lambda's design arranged the ion flux converters to form one. To label them, Lambda performed a post-order traversal of the tree of converters and labeled each converter with the order of that converter in the traversal, starting at 1. For example, a tree of 7 converters would look like the following:

```
7
3 6
1 2 4 5
```

Write a function `solution(h, q)` - where h is the height of the perfect tree of converters and q is a list of positive integers representing different flux converters - which returns a list of integers p where each element in p is the label of the converter that sits on top of the respective converter in q, or -1 if there is no such converter.  For example, `solution(3, [1, 4, 7])` would return the converters above the converters at indexes 1, 4, and 7 in a perfect binary tree of height 3, which is `[3, 6, -1]`.

The domain of the integer h is `1 <= h <= 30`, where `h = 1` represents a perfect binary tree containing only the root, `h = 2` represents a perfect binary tree with the root and two leaf nodes, `h = 3` represents a perfect binary tree with the root, two internal nodes and four leaf nodes (like the example above), and so forth.  The lists q and p contain at least one but no more than 10,000 distinct integers, all of which will be between `1` and `2^h-1`, inclusive.

#### Test Cases

Input:

```
solution.solution(5, [19, 14, 28])
```

Output:

```
21,15,29
```

Input:

```
solution.solution(3, [7, 3, 5, 1])
```
Output:

```
-1,7,6,3
```

