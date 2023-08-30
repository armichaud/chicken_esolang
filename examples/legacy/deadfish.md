## Load 0 Index of Register 2, Pushes undefined
2. 10
3. 6
4. 2

## Compare 0 and 1
5. 10
6. 11
7. 5

## Push chicken, concat twice, store "undefinedfalsechicken" in register 4
8. 1
9. 2
10. 2
11. 14
12. 7

## Get space, store in register 5
13. 18
14. 14
15. 4
16. 9
17. 15
18. 7

## Load register 5, store in register 6
19. 15
20. 6
21. 0
22. 16
23. 7

## Store -1 in register 3
24. 10
25. 11
26. 3
27. 13
28. 7

## Store 0 in register 2
29. 10
30. 12
31. 7

### Initial Stack: [ , , 0, -1, "undefinedfalsechicken", " ", " "]
### General Purpose: [ , input, running value, input index, i/d/s, full output string, temp output string]

## Load from register 2, compare with -1, if true jump back 14
32. 10
33. 11
34. 3
35. 12
36. 6
37. 0
38. 5
39. 10
40. 24
41. 3
42. 8

## Compare contents of register 2 to 144, if true jump back 27
43. 18
44. 18
45. 4
46. 14
47. 4
48. 12
49. 6
50. 0
51. 5
52. 10
53. 37
54. 3
55. 8

## Increment register 3
56. 11
57. 13
58. 6
59. 0
60. 2
61. 13
62. 7

## Load from register 2, subtract 1
63. 12
64. 6
65. 0
66. 11
67. 3

## Load from register 3, Use as index to load from input
68. 13
69. 6
70. 0
71. 6
72. 1

## Load d from register 4, compare, if so go back 51
73. 12
74. 6
75. 4
76. 5
77. 10
78. 61
79. 3
80. 8

## Add 2, Load again input at index stored in register 3
81. 12
82. 2
83. 13
84. 6
85. 0
86. 6
87. 1

# Load i from register 4, compare, if true, go back 66
88. 15
89. 6
90. 4
91. 5
92. 10
93. 76
94. 3
95. 8

## Subtract 1, Load register 2, Multiply
96. 11
97. 3
98. 12
99. 6
100. 0
101. 4

## Load s from register 4, Load again input at index stored in register 3 
102. 22
103. 6
104. 4
105. 13
106. 6
107. 0
108. 6
109. 1

## Compare, jump back 85 if true
110. 5
111. 10
112. 95
113. 3
114. 8

## Load register 5, then again input at index stored in register 3
115. 15
116. 6
117. 0
118. 13
119. 6
120. 0
121. 6
122. 1

## Jump forward 1
123. 11
124. 8
125. 0

## Load register 2, add
126. 12
127. 6
128. 0
129. 2

## Load from register 6, concatenate, store in register 5
130. 16
131. 6
132. 0
133. 2
134. 15
135. 7

## Jump back 109
136. 11
137. 10
138. 119
139. 3
140. 8
