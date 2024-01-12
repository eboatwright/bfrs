# BF.rs
 Brainf\*ck Interpreter in Rust<br>
 Inspired by [Lucrecious' interpreter in Crystal](https://www.youtube.com/watch?v=c1p7Cnbn8Xw)<br><br>
 If you use this code verbatim or as a reference, please credit me!<br><br>
 [The Official Brainf\*ck Website](https://brainfuck.org/)

## Usage
 To run a .bf file, use this command:
 ``
 ./bfrs your_file_here.bf
 ``
 <br>
 BF.rs also has 2 custom instructions useful for debugging:
 ```
 : Prints out the raw value at the current memory index
 # Prints out all memory to the terminal
 ```
 To enable them, just add --ci to the end of your run command

## Helpful Sources
 [Brainf\*ck Wiki Page](https://en.wikipedia.org/wiki/Brainfuck) | 
 [Basics of Brainf\*ck](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
