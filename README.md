## Google FooBar 2023 Programming Challenge in Rust

This repo contains Google FooBar 2023 Programming Challenge solutions written in Rust.

The solution of each challenge can be found in `src/lib` as individual module.

NOTE: Google FooBar 2023 Programming Challenge only accepts Java or Python code as solution, all solutions of this repo are only for personal training purpose.

## Level 1

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

### Challenge 2: Don't Get Volunteered!

#### Explain

As a henchman on Commander Lambda's space station, you're expected to be resourceful, smart, and a quick thinker. It's not easy building a doomsday device and ordering the bunnies around at the same time, after all! In order to make sure that everyone is sufficiently quick-witted, Commander Lambda has installed new flooring outside the henchman dormitories. It looks like a chessboard, and every morning and evening you have to solve a new movement puzzle in order to cross the floor. That would be fine if you got to be the rook or the queen, but instead, you have to be the knight. Worse, if you take too much time solving the puzzle, you get "volunteered" as a test subject for the LAMBCHOP doomsday device!

To help yourself get to and from your bunk every day, write a function called solution(src, dest) which takes in two parameters: the source square, on which you start, and the destination square, which is where you need to land to solve the puzzle.  The function should return an integer representing the smallest number of moves it will take for you to travel from the source square to the destination square using a chess knight's moves (that is, two squares in any direction immediately followed by one square perpendicular to that direction, or vice versa, in an "L" shape).  Both the source and destination squares will be an integer between 0 and 63, inclusive, and are numbered like the example chessboard below:

```
-------------------------
| 0| 1| 2| 3| 4| 5| 6| 7|
-------------------------
| 8| 9|10|11|12|13|14|15|
-------------------------
|16|17|18|19|20|21|22|23|
-------------------------
|24|25|26|27|28|29|30|31|
-------------------------
|32|33|34|35|36|37|38|39|
-------------------------
|40|41|42|43|44|45|46|47|
-------------------------
|48|49|50|51|52|53|54|55|
-------------------------
|56|57|58|59|60|61|62|63|
-------------------------
```

#### Test Cases

Input:

```
solution.solution(19, 36)
```

Output:

```
1
```

Input:

```
solution.solution(0, 1)
```

Output:

```
3
```
