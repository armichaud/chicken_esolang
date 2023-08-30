# reverse.chn 

Reverse the input string. This file represents each instruction as the number of chickens on each line. Line numbers are prepended for ease of following JUMPs.

Note that this program will loop infinitely in backwards compatible mode: char conversion of 0 results in `&#0;`, which will not pass comparisons to trigger the end of the loop.

First instruction is stored at register 2 (remember that 0 is the instruction register and 1 is the user input).

## Load from user input
2. 11
3. 6
4. 0

## Concatenate NUL to input
5. 10
6. 9
7. 2

## Store resulting string in register 3
8. 13
9. 7

## Load index, get first char from register 3 at index
10. 10
11. 6
12. 3

## Compare char to NUL, if true then we have an empty string, go to end (45 offset)
13. 10
14. 9
15. 5
16. 55
17. 8

## Set register 4 to value of first char from register 3
18. 10
19. 6
20. 3
21. 14
22. 7

## Store 1 in register 2 (initialize index at one because we've already stored the first character)
23. 11
24. 12
25. 7

### Begin Loop
## Get char at index from register 3, compare to NUL, If true Jump to end (23 offset)
26. 12
27. 6
28. 0
29. 6
30. 3
31. 10
32. 9
33. 5
34. 33
35. 8

## Get char at index from register 3, get value at register 4
36. 12
37. 6
38. 0
39. 6
40. 3
41. 14
42. 6
43. 0

## Concatenate, store at register 4
44. 2
45. 14
46. 7

## Increment index
47. 12
48. 6
49. 0
50. 11
51. 2
52. 12
53. 7

## Jump to beginning of loop (-33 offset)
54. 11
55. 10
56. 43
57. 3
58. 8
### End Loop

## Load Register 4, End
59. 14
60. 6
61. 0
62. 0

## Load input, End
63. 11
64. 6
65. 0
66. 0
