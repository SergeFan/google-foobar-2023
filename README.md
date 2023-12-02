## Google Foobar 2023 Programming Challenge in Rust

This repo contains Google FooBar 2023 Programming Challenge solutions written in Rust.

The solution of each challenge can be found in `src/lib` as individual module.

NOTE: Google FooBar 2023 Programming Challenge only accepts Java 8 or Python 2.7 code as solution,
all solutions of this repo are only for personal practice purpose.

## Level 1

Success!
You've managed to infiltrate Commander Lambda's evil organization,
and finally earned yourself an entry-level position as a Minion on their space station.
From here,
you just might be able to subvert Commander Lambda's plans to use the LAMBCHOP doomsday device to destroy Bunny Planet.
Problem is, Minions are the lowest of the low in the Lambda hierarchy.
Better buck up and get working, or you'll never make it to the top...

### Challenge 1: Braille Translation

> Commander Lambda sure is a task-master, aren't they? You're being worked to the bone!

#### Explain
Because Commander Lambda is an equal-opportunity despot, they have several visually-impaired minions.
But Lambda never bothered to follow intergalactic standards for workplace accommodations,
so those minions have a hard time navigating her space station.
You figure printing out Braille signs will help them,
and -- since you'll be promoting efficiency at the same time -- increase your chances of a promotion.

Braille is a writing system used to read by touch instead of by sight.
Each character is composed of 6 dots in a `2x3` grid, where each dot can either be a bump or be flat (no bump).
You plan to translate the signs around the space station to Braille
so that the minions under Commander Lambda's command can feel the bumps on the signs and "read" the text with their touch.
The special printer which can print the bumps onto the signs expects the dots in the following order:

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

Write a function solution (plaintext) that takes a string parameter
and returns a string of 1's and 0's representing the bumps and absence of bumps in the input string.
Your function should be able to encode the 26 lowercase letters,
handle capital letters by adding a Braille capitalization mark before that character, and use a blank character (`000000`) for spaces.
All signs on the space station are less than fifty characters long and use only letters and spaces.

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

You survived a week in Commander Lambda's organization, and you even managed to get yourself promoted.
Hooray!
Henchmen still don't have the kind of security access you'll need to take down Commander Lambda, though,
so you'd better keep working. Chop chop!

### Challenge 1: Ion Flux Relabeling

> At least all this time spent running errands all over Commander Lambda's space station have given you a really good understanding of the station's layout.
> You'll need that when you're finally ready to destroy the LAMBCHOP and rescue the bunny workers.

#### Explain

Oh no!
Commander Lambda's latest experiment to improve the efficiency of the LAMBCHOP doomsday device has backfired spectacularly.
The Commander had been improving the structure of the ion flux converter tree,
but something went terribly wrong and the flux chains exploded.
Some of the ion flux converters survived the explosion intact, but others had their position labels blasted off.
Commander Lambda is having her henchmen rebuild the ion flux converter tree by hand,
but you think you can do it much more quickly -- quickly enough, perhaps, to earn a promotion!

Flux chains require perfect binary trees,
so Lambda's design arranged the ion flux converters to form one.
To label them,
Lambda performed a post-order traversal of the tree of converters 
and labeled each converter with the order of that converter in the traversal, starting at 1.
For example, a tree of 7 converters would look like the following:

```
7
3 6
1 2 4 5
```

Write a function `solution(h, q)` - 
where `h` is the height of the perfect tree of converters and `q` is a list of positive integers representing different flux converters -
which returns a list of integers `p` where each element in `p` is the label of the converter that sits on top of the respective converter in q,
or -1 if there is no such converter.
For example,
`solution(3, [1, 4, 7])` would return the converters above the converters at indexes 1, 4, and 7 in a perfect binary tree of height 3,
which is `[3, 6, -1]`.

The domain of the integer h is `1 <= h <= 30`,
where `h = 1` represents a perfect binary tree containing only the root,
`h = 2` represents a perfect binary tree with the root and two leaf nodes,
`h = 3` represents a perfect binary tree with the root,
two internal nodes and four leaf nodes (like the example above), and so forth.
The lists `q` and `p` contain at least one but no more than 10,000 distinct integers,
all of which will be between `1` and `2^h-1`, inclusive.

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

> Rumor has it the bunny trainers are inexplicably fond of bananas.
> You're an apple person yourself, but you file the information away for future reference.
> You never know when you might need to bribe a trainer (or three)...

#### Explain

As a henchman on Commander Lambda's space station,
you're expected to be resourceful, smart, and a quick thinker.
It's not easy building a doomsday device and ordering the bunnies around at the same time, after all!
In order to make sure that everyone is sufficiently quick-witted,
Commander Lambda has installed new flooring outside the henchman dormitories.
It looks like a chessboard, and every morning and evening you have to solve a new movement puzzle in order to cross the floor.
That would be fine if you got to be the rook or the queen, but instead, you have to be the knight.
Worse, if you take too much time solving the puzzle, you get "volunteered" as a test subject for the LAMBCHOP doomsday device!

To help yourself get to and from your bunk every day,
write a function called solution(src, dest) which takes in two parameters:
the source square, on which you start, and the destination square, which is where you need to land to solve the puzzle.
The function should return an integer representing the smallest number of moves it will take
for you to travel from the source square to the destination square using a chess knight's moves
(that is, two squares in any direction immediately followed by one square perpendicular to that direction, or vice versa, in an "L" shape).
Both the source and destination squares will be an integer between 0 and 63, inclusive,
and are numbered like the example chessboard below:

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

## Level 3

Awesome!
Commander Lambda was so impressed by your efforts that you've been promoted to personal assistant.
You'll be helping the Commander directly,
which means you'll have access to all of Lambda's files -- including the ones on the LAMBCHOP doomsday device.
This is the chance you've been waiting for.
Can you use your new access to finally topple Commander Lambda's evil empire?

### Challenge 1: The Grandest Staircase Of Them All

> Commander Lambda has six suits, three dress uniforms, four casual outfits, and one Dress-Uniform-For-Important-Speeches-Only.
> You know this because you've already had to take all of them to the dry cleaner's. Twice!


#### Explain

With the LAMBCHOP doomsday device finished,
Commander Lambda is preparing to debut on the galactic stage -- but in order to make a grand entrance, Lambda needs a grand staircase!
As the Commander's personal assistant, you've been tasked with figuring out how to build the best staircase EVER.

Lambda has given you an overview of the types of bricks available, plus a budget.
You can buy different amounts of the different types of bricks (for example, 3 little pink bricks, or 5 blue lace bricks).
Commander Lambda wants to know how many different types of staircases can be built with each amount of bricks, so they can pick the one with the most options.

Each type of staircase should consist of 2 or more steps.
No two steps are allowed to be at the same height - each step must be lower than the previous one.
All steps must contain at least one brick. A step's height is classified as the total amount of bricks that make up that step.

For example,
when `N = 3`, you have only 1 choice of how to build the staircase,
with the first step having a height of 2 and the second step having a height of 1: (`#` indicates a brick)

```
#
# #
2 1
```

When `N = 4`, you still only have 1 staircase choice:

```
#
#
# #
3 1
```

But when `N = 5`, there are two ways you can build a staircase from the given bricks.
The two staircases can have heights (4, 1) or (3, 2), as shown below:

```
#
#     #
#     # #
# #   # #
4 1   3 2
```

Write a function called `solution(n)` that takes a positive integer `n` and returns the number of different staircases that can be built from exactly `n` bricks.
`n` will always be at least 3 (so you can have a staircase at all), but no more than 200, because Commander Lambda's not made of money!

#### Test Case

Input:

```
solution.solution(200)
```

Output:

```
487067745
```

Input:

```
solution.solution(3)
```

Output:

```
1
```

### Challenge 2: Find the Access Codes

> One of these days you're going to manage to glimpse Commander Lambda's password over their shoulder.
> But the Commander is very careful about security and you haven't managed it yet...

#### Explain

In order to destroy Commander Lambda's LAMBCHOP doomsday device, you'll need access to it.
But the only door leading to the LAMBCHOP chamber is secured with a unique lock system whose number of passcodes changes daily.
Commander Lambda gets a report every day that includes the locks' access codes,
but only the Commander knows how to figure out which of several lists contains the access codes.
You need to find a way to determine which list contains the access codes once you're ready to go in.

Fortunately, now that you're Commander Lambda's personal assistant,
Lambda has confided to you that all the access codes are "lucky triples" in order to make it easier to find them in the lists.
A "lucky triple" is a tuple `(x, y, z)` where `x` divides `y` and `y` divides `z`, such as `(1, 2, 4)`.
With that information,
you can figure out which list contains the number of access codes that matches the number of locks on the door when you're ready to go in
(for example, if there's 5 passcodes, you'd need to find a list with 5 "lucky triple" access codes).

Write a function `solution(l)` that takes a list of positive integers `l` and counts the number of "lucky triples" of `(li, lj, lk)`
where the list indices meet the requirement `i < j < k`.
The length of `l` is between `2` and `2000` inclusive.
The elements of `l` are between `1` and `999999` inclusive.
The solution fits within a signed 32-bit integer.
Some of the lists are purposely generated without any access codes to throw off spies, so if no triples are found, return 0.

For example, `[1, 2, 3, 4, 5, 6]` has the triples: `[1, 2, 4]`, `[1, 2, 6]`, `[1, 3, 6]`, making the solution 3 total.

#### Test Case

Input:

```
solution.solution([1, 1, 1])
```

Output:

```
1
```

Input:

```
solution.solution([1, 2, 3, 4, 5, 6])
```

Output:

```
3
```

### Challenge 3: Prepare the Bunnies' Escape

> There are a lot of difficult things about being undercover as Commander Lambda's personal assistant,
> but you have to say, the personal spa and private hot cocoa bar are pretty awesome.

#### Explain

You're awfully close to destroying the LAMBCHOP doomsday device and freeing Commander Lambda's bunny workers,
but once they're free of the work duties the bunnies are going to need to escape Lambda's space station via the escape pods as quickly as possible.
Unfortunately,
the halls of the space station are a maze of corridors and dead ends that will be a deathtrap for the escaping bunnies.
Fortunately,
Commander Lambda has put you in charge of a remodeling project that will give you the opportunity to make things a little easier for the bunnies.
Unfortunately (again),
you can't just remove all obstacles between the bunnies and the escape pods - at most you can remove one wall per escape pod path,
both to maintain structural integrity of the station and to avoid arousing Commander Lambda's suspicions.

You have maps of parts of the space station, each starting at a work area exit and ending at the door to an escape pod.
The map is represented as a matrix of 0s and 1s, where 0s are passable space and 1s are impassable walls.
The door out of the station is at the top left `(0,0)` and the door into an escape pod is at the bottom right `(w-1,h-1)`.

Write a function `solution(map)` that generates the length of the shortest path from the station door to the escape pod,
where you are allowed to remove one wall as part of your remodeling plans.
The path length is the total number of nodes you pass through, counting both the entrance and exit nodes.
The starting and ending positions are always passable (`0`).
The map will always be solvable, though you may or may not need to remove a wall.
The height and width of the map can be from 2 to 20.
Moves can only be made in cardinal directions; no diagonal moves are allowed.

#### Test Case

Input:

```
solution([[0, 0, 0, 0, 0, 0], [1, 1, 1, 1, 1, 0], [0, 0, 0, 0, 0, 0], [0, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1], [0, 0, 0, 0, 0, 0]])
```

Output:

```
11
```

Input:

```
solution([[0, 1, 1, 0], [0, 0, 0, 1], [1, 1, 0, 0], [1, 1, 1, 0]])
```

Output:

```
7
```
