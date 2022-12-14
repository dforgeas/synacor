#!/usr/bin/awk -f
BEGIN {
	max = 2^15
	# unit tests
	if (bitwise_and(1000, 100) != 96) exit 1
	if (bitwise_and(max - 1, max - 1) != max - 1) exit 2
	if (bitwise_or(max - 1, max - 1) != max - 1) exit 3
	if (bitwise_or(0, max - 1) != max - 1 || bitwise_or(max - 1, 0) != max - 1) exit 4
	if (bitwise_or(1000, 100) != 1004) exit 5
	if (bitwise_not(0) != max - 1) exit 6
	if (bitwise_not(max - 1) != 0) exit 7
	if (bitwise_not(1000) != 31767) exit 8
	ord_index = ""
	# initialize our character code function
	for (i = 1; i < 256; i+=4) ord_index = sprintf("%s%c%c%c%c", ord_index, i, 1+i, 2+i, 3+i) # not found is 0
	# try to load our savestate
	if (! loadstate())
	{
		# the virtual machine memory areas
		for (i = 0; i < max; i++) mem[i] = 0
		for (i = 0; i < 8; i++) regs[i] = 0
		stack_p = 1
		# load the program in memory
		program = "od --endian=little -dv challenge.bin"
		j = 0
		while ((program | getline) > 0) {
			for (i = 2; i <= NF; i++) mem[j++] = $i + 0
		}
		close(program)
		pc = 0
	}
	input = ""
	while (1) {
		i = mem[pc++]
		if (i == 2) {
			stack[stack_p++] = rreg(pc++)
		} else if (i == 3) {
			if (stack_p == 1) exit 100
			else wreg(pc++, stack[--stack_p])
		} else if (i == 12) {
			a = pc++
			b = pc++
			wreg(a, bitwise_and(rreg(b), rreg(pc++)))
		} else if (i == 17) {
			a = pc++
			stack[stack_p++] = pc
			pc = rreg(a)
		} else if (i == 18) {
			if (stack_p == 1) exit
			else pc = stack[--stack_p]
		} else if (i == 14) {
			a = pc++
			wreg(a, bitwise_not(rreg(pc++)))
		} else if (i == 13) {
			a = pc++
			b = pc++
			wreg(a, bitwise_or(rreg(b), rreg(pc++)))
		} else if (i == 9) {
			a = pc++
			b = pc++
			wreg(a, (rreg(b) + rreg(pc++)) % max)
		} else if (i == 15) {
			a = pc++
			wreg(a, mem[rreg(pc++)])
		} else if (i == 1) {
			a = pc++
			wreg(a, rreg(pc++))
		} else if (i == 8) {
			a = pc++
			b = pc++
			if (rreg(a) == 0) pc = rreg(b)
		} else if (i == 4) {
			a = pc++
			b = pc++
			wreg(a, rreg(b) == rreg(pc++))
		} else if (i == 16) {
			a = pc++
			mem[rreg(a)] = rreg(pc++)
		} else if (i == 10) {
			a = pc++
			b = pc++
			wreg(a, (rreg(b) * rreg(pc++)) % max)
		} else if (i == 7) {
			a = pc++
			b = pc++
			if (rreg(a) != 0) pc = rreg(b)
		} else if (i == 19) {
			printf "%c", rreg(pc++)
		} else if (i == 5) {
			a = pc++
			b = pc++
			wreg(a, rreg(b) > rreg(pc++))
		} else if (i == 6) {
			pc = rreg(pc)
		} else if (i == 21) {
		} else if (i == 11) {
			a = pc++
			b = pc++
			wreg(a, rreg(b) % rreg(pc++))
		} else if (i == 20) {
			if (input == "") {
				i = getline input
				if (i <= 0) { # EOF or input error
					savestate(pc - 1)
					exit
				} else if (input == "!prepare teleporter") {
			# set 6 into the first register instead, the expected result from ack
					mem[5483 + 2] = 6
			# replace the call to ack with noops
					mem[5489] = mem[5489 + 1] = 21
			# set the eigth register to the correct teleporter value
					regs[7] = 25734
					printf "\nPrepared.\n"
					input = "" # hide this line to the program
				}
				input = input ORS
			}
			wreg(pc++, index(ord_index, substr(input, 1, 1)))
			input = substr(input, 2)
		} else if (i == 0) {
			exit
		} else {
			exit 101
		}
	}
}

function rreg(addr,     x) {
	x = mem[addr]
	if (x >= max) return regs[x - max]
	else return x
}

function wreg(addr, val,     x) {
	x = mem[addr]
	if (x >= max) regs[x - max] = val
}

function bitwise_and(a, b,     i, j, r) {
	r = 0
	for (i = 0; i < 15; i++) {
		j = 2^i
		r += j * (int(a / j) * int(b / j) % 2)
	}
	return r
}

function bitwise_or(a, b,     i, j, r) {
	r = 0
	for (i = 0; i < 15; i++) {
		j = 2^i
		r += j * (int(a / j) % 2 + int(b / j) % 2 > 0)
	}
	return r
}

function bitwise_not(a,     i, j, r) {
	r = 0
	for (i = 0; i < 15; i++) {
		j = 2^i
		r += j * (int(a / j) % 2 == 0)
	}
	return r
}

function savestate(pc,     i, file) {
	file = "bzip2 -zc > savestate.txt.bz2"
	for (i = 0; i < max; i++) print mem[i] | file
	for (i = 0; i < 8; i++) print regs[i] | file
	print pc | file
	for (i = 1; i < stack_p; i++) print stack[i] | file
	close(file)
}

function loadstate(     i, file) {
	file = "savestate.txt.bz2"
	if ((getline < file) <= 0) { return 0 } # cannot read file
	else close(file) # can read: close and load
	file = "bzip2 -dc < savestate.txt.bz2"
	if ((file | getline) <= 0) { return 0 } # cannot read file
	else close(file) # can read: close and load

	for (i = 0; i < max; i++) { file | getline; mem[i] = $1 }
	for (i = 0; i < 8; i++) { file | getline; regs[i] = $1 }
	file | getline; pc = $1
	stack_p = 1
	while ((file | getline) > 0) stack[stack_p++] = $1
	close(file)
	return 1
}
