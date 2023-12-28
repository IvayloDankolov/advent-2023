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