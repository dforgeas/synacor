out 72
out 101
out 108
out 108
out 111
out 33
out 10
jmp Init
out 72
out 101
out 108
out 108
out 111
out 33
out 10
Init:
set $6 10
Loop:
in $7
out $7
out 10
add $6 32767 $6
gt $5 $6 0
jt $5 Loop
out 66
out 121
out 101
out 33
out 10
