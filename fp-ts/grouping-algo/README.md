# TypeScript implementation of a live student grouping algorithm

First draft: use double weighted (undirected) graph system with live addition
and manual re-shuffling. One graph of students and one graph of groups
    - Only option of either fixing/preferring group-size or group number
    - First set max group-size and group-number and then set preference of
      which is more important
