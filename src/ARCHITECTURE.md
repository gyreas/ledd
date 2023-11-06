# How stuff works
This file tracks my progress as I go through this journey.
This project has 3 branches now: `master`, `vector-strings`, and `ropey`(uncreated).
The `master` branch will go with the current implementation of the especially the `Line` struct while
the `vector-strings` branch will implement the buffer as a vector of strings using some kind of coordinate system
to track the position of the edit cursor; the `ropey` branch implements the buffer using rope data structure (I don't have a
clue how that works now, so I'm gonna bench it).

The cursor struct is, as of now, mainly for decoration purpose
