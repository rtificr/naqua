# Usage
Mac/Windows (Terminal/Shell):  
`naqua <filepath>`  
Windows:  
`file.naq > Open With > naqua.exe`

# Syntax
* `think x` - loads number `x` into single-cell memory
* `thought` - evaluated as the number in single-cell memory
* `print x` - prints out value `x`
* `x in y` - assigns number `y` to stack index `x`
* `out x` - evaluated as the number in stack index `x`
* `x char` - evaluated as the character with ascii index `x`. characters cannot be stored as data (in single-cell memory or in the stack)
* `if x { ... }` - runs code within braces if `x` is equivalent to the single-cell memory
* `loop { ... }` - loops code within braces
* `break` - breaks current loop/if statement
* `#<text here>` - ignored when tokenizing (until the end of the line)

# Examples
## Hello, World!
```
0 in 64 + 8         # H
1 in 96 + 5         # e
2 in 96 + 12        # l
3 in 96 + 12        # l
4 in 96 + 15        # o
5 in 44             # ,
6 in 32             #  
7 in 64 + 23        # W
8 in 96 + 15        # o
9 in 96 + 18        # r
10 in 96 + 12        # l
11 in 96 + 4         # d
12 in 33             # !

print out 0 char
print out 1 char
print out 2 char
print out 3 char
print out 4 char
print out 5 char
print out 6 char
print out 7 char
print out 8 char
print out 9 char
print out 10 char
print out 11 char
print out 12 char
print 10 char # New line
print 10 char # New line x2
```
## Fibonacci Sequence
```
1 in 0   # 1st element
2 in 1   # 2nd element

loop {
    print out 0         # print out current value
    print 10 char       # newline

    0 in out 1 + out 2  # sum of the elements

    1 in out 2          # set 1st element to 2nd element
    2 in out 0          # set 2nd element to sum

    think thought + 1   # increment counter

    if 10 { break }     # break on 10th iteration
}
```
