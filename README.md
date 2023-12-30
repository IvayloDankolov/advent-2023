# Advent of code 2023 

I was looking for an excuse to learn Rust. Advenc of Code exists. It was rather simple putting two and two together, really.

That's all this project is. A simple CLI run harness plus the solutions for each day.

I know I'm a bit late to the party. We'll see how many of these we get through, only now getting free time to look at these around the holidays.


## Fun moments / insights

### Day 3

I confess, I fell to temptation and went all "eh, I can't be bothered to parse this nicely. For loops to the rescue #yolo"
I then had to spend an embarassing amount of time debugging a subtle off by one error

Then I wrote the solution to part 2 as a declarative one liner in about 90 seconds and it worked first time.
If that's not a parable to not fall to the tempations of iterative programming and stick to functional purity, I don't know what is.

### Day 6

I briefly considered solving this as a quadratic inequality (as in windup * (time - windup) > record, solve for "windup")

That lasted all of 5 seconds until I remembered how much I'd rather not deal with floating point errors.

Afterwards I went for the "old faithful" binary search (technically, monotonic binary predicate satisfaction). All the while laughing at how much of an overkill it is for numbers you can easily do by hand.

At which point I read part 2 and was laughing for entirely different reasons.

### Day 7

Both parts are pretty trivial when using a common insight from regular poker, specifically that to compare hand ranks, you can just order the card groups from largest to smallest and compare lexicographically. E.g. four-of-a kind (4,1) > full house (3,2) > two pair (2,2) and so forth.

Having five-of-a-kind here does not disrupt that really.

The more fiddly part was working out how to dedfine "I'm a function that just derives something from a sequence without allocating a specific collection like a vector". I sort of fumbled through it feels like there should be a simpler way and I'm not sure my intuition on template lifetime types compiles to what I think it compiles.

Ah well, I'm sure there's a macro for it or something.


### Day 8

Graphs baybe!

I uh definitely misinterpreted part 2 in my head. There's no reason that after reaching an ending from a start position that it would take the same number of cycles of the instructions as it originally did to get back to an end position again.

The fact that the LCM solution just worked implies people's inputs get generated by adding up individual graph cycles and the A/Z pairings are independent. 

I suppose it's a quick way to generate a non-brute-forcable solution, so my mind naturally went there.

Sometimes lucky, I suppose...

### Day 9 

Nice try, AoC, with the attempted misdirection you have to calculate all the differences and work backwards.

You can totally apply the derivatives forwards as well, though, just have to mind the signs in part 2.

### Day 10

Now we're getting somewhere. Graph searches and geometry!

I also used the opportunity to look up how to implement traits and methods in Rust, as well as bit fields.
It's definitely very sensible overall (other than derive being annoyingly limited, though I suppose they were afraid a good-enough-most-of-the-time definition for something like Add/Sub that just distributes the oeprations might be too much of a footgun).

Overall quite happy with what I got to try on this noe.

By the way, anyone who's done geometric algorithms is probably well familiar with how to check if something is inside a given polygon via raycasting, but doing it in the context of a neighbour-list matrix is super damned fiddly.

### Day 11

I did it entirely the 'OOP' way. Or something. Not a huge enjoyer of 'make everything a member function' and that's unlikely to change in Rust.

As for the task itself, I get that it was trying to misdirect people into actually 'expanding' the universe and generating a modified map. Unfortunately for the setters I couldn't be bothered with any of that nonsense, so the two parts differ by just the expansion constant in terms of code.

### Day 12

Now we're getting some proper (relatively lightweight) contest-style stuff.

I really stubbornly wanted to brute force this, restorting to running it in parallel with a really dumb tight inner loop for the combination generator and I made it short-circuit based on a just as stupidly iterative prefix search. I left that in a separate text file to document my shame for the ages.

Obviously the memoized solution takes about 1/10th of the effort to write (and is immesurably faster). But sometimes you just have this morbid fascination to just see how far you can push a language, you know...

Also, apparently recursive closures aren't a thing. I get that borrowing stuff gets complicated, but if recursive methods are a thing (that by necessity borrow "self"), surely it's solvable. Anyway, as far as gripes go this is a rather incosequential one.