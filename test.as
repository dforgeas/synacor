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
set $0 10
call Factorial
call PrintNumber
jmp TestJumps
Factorial:
gt $5 $0 1
jt $5 Factorial_recur
set $0 1
ret
Factorial_recur:
push $0
add $0 $0 32767
call Factorial
pop $1
mult $0 $1 $0
ret
PrintNumber:
eq $5 $0 24320
jf $5 PrintNumber_WrongAnswer
out 71
out 111
out 116
out 32
out 50
out 52
out 51
out 50
out 48
out 10
ret
PrintNumber_WrongAnswer:
out 71
out 111
out 116
out 32
out 115
out 111
out 109
out 101
out 116
out 104
out 105
out 110
out 103
out 32
out 101
out 108
out 115
out 101
out 10
ret
TestJumps:
set $4 Jump4
set $5 Jump5
set $6 Jump6
set $0 0
set $1 1
jt $0 $4
jf $1 $5
jf $0 $6
continue6:
jt $1 $5
continue5:
jf $0 $4
continue4:
halt
Jump4:
out 106
out 117
out 109
out 112
out 101
out 100
out 32
out 116
out 111
out 32
out 102
out 111
out 117
out 114
out 10
jmp continue4
Jump5:
out 106
out 117
out 109
out 112
out 101
out 100
out 32
out 116
out 111
out 32
out 102
out 105
out 118
out 101
out 10
jmp continue5
Jump6:
out 106
out 117
out 109
out 112
out 101
out 100
out 32
out 116
out 111
out 32
out 115
out 105
out 120
out 10
jmp continue6
