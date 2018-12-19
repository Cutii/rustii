# Mars Rover Kata

The aim of this kata is to implement the specification bellow using pair programming and test driven development.

## Tests

Since Rust is strongly and statically typed, you can apply a "Type and Tests driven development" pattern :

1. Encode the data in the type system
2. Write test to describe a transition
3. Implement it
4. Test it
5. Refactor

## Pair programming Strong Style

Two roles : Driver & Navigator

### Driver

- Write tests and code
- Follow navigator's instructions
- Follow navigator's tempo

So that he/she :

- Trusts the navigator
- Focus on the current function / class

### Navigator

- Give the path to the solution
- Give instructions one by one to the driver
- Use the higher level of abstraction that the driver can understand

So that he/she :

- reassures and explain when required
- review continously the driver's production

## Let's Go

You’re part of the team that explores Mars by sending remotely controlled vehicles to the surface of the planet. Develop an API that translates the commands sent from earth to instructions that are understood by the rover.

### Requirements

- You are given the initial starting point (x,y) of a rover and the direction (N,S,E,W) it is facing.
- The rover receives a character array of commands.
- Implement commands that move the rover forward/backward (f,b).
- Implement commands that turn the rover left/right (l,r).
- Implement wrapping from one edge of the grid to another. (planets are spheres after all)
- Implement obstacle detection before each move to a new square. If a given sequence of commands encounters an obstacle the rover moves up to the last possible point, aborts the sequence and reports the obstacle.

### Rules

- Hardcore TDD. No Excuses!
- Change roles (driver, navigator) after each TDD cycle.
- No red phases while refactoring.
- Be careful about edge cases and exceptions. We can not afford to lose a mars rover, just because the developers overlooked a null pointer.
