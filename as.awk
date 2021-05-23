#!/usr/bin/awk -f

BEGIN {
	code[code_i++] = 21 # noop
	code[code_i++] = 21 # noop
}

{
	if (code_i > 32767) {
		printf("Too many instructions from line %d: %s\n", NR, $0) > "/dev/stderr"
		exit 1
	}
}

# ================
/^[ \t]*halt/ {
	code[code_i++] = 0
	next
}
/^[ \t]*set[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 1
	process_args()
	next
}
/^[ \t]*push[ \t]+\$?[0-9]+/ {
	code[code_i++] = 2
	process_args()
	next
}
/^[ \t]*pop[ \t]+\$?[0-9]+/ {
	code[code_i++] = 3
	process_args()
	next
}
/^[ \t]*eq[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 4
	process_args()
	next
}
/^[ \t]*gt[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 5
	process_args()
	next
}
/^[ \t]*jmp[ \t]+\$?[0-9]+/ {
	code[code_i++] = 6
	process_args()
	next
}
/^[ \t]*jt[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 7
	process_args()
	next
}
/^[ \t]*jf[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 8
	process_args()
	next
}
/^[ \t]*add[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 9
	process_args()
	next
}
/^[ \t]*mult[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 10
	process_args()
	next
}
/^[ \t]*mod[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 11
	process_args()
	next
}
/^[ \t]*and[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 12
	process_args()
	next
}
/^[ \t]*or[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 13
	process_args()
	next
}
/^[ \t]*not[ \t]+\$?[0-9]+[ \t]+\$?[0-9]+/ {
	code[code_i++] = 14
	process_args()
	next
}
/^[ \t]*rmem[ \t]+\$?[0-9]+[ \t]+\(\$?[0-9]+\)/ {
	code[code_i++] = 15
	process_args()
	next
}
/^[ \t]*wmem[ \t]+\(\$?[0-9]+\)[ \t]+\$?[0-9]+/ {
	code[code_i++] = 16
	process_args()
	next
}
/^[ \t]*call[ \t]+\$?[0-9]+/ {
	code[code_i++] = 17
	process_args()
	next
}
/^[ \t]*ret/ {
	code[code_i++] = 18
	next
}
/^[ \t]*out[ \t]+\$?[0-9]+/ {
	code[code_i++] = 19
	process_args()
	next
}
/^[ \t]*in[ \t]+\$?[0-9]+/ {
	code[code_i++] = 20
	process_args()
	next
}
/^[ \t]*noop/ {
	code[code_i++] = 21
	next
}
/^[ \t]*set[ \t]+\$?[0-9]+[ \t]+[A-Za-z_][A-Za-z0-9_]+/ {
	code[code_i++] = 1
	process_jump(1)
	next
}
/^[ \t]*jmp[ \t]+[A-Za-z_][A-Za-z0-9_]+/ {
	code[code_i++] = 6
	process_jump(0)
	next
}
/^[ \t]*jt[ \t]+\$?[0-9]+[ \t]+[A-Za-z_][A-Za-z0-9_]+/ {
	code[code_i++] = 7
	process_jump(1)
	next
}
/^[ \t]*jf[ \t]+\$?[0-9]+[ \t]+[A-Za-z_][A-Za-z0-9_]+/ {
	code[code_i++] = 8
	process_jump(1)
	next
}
/^[ \t]*call[ \t]+[A-Za-z_][A-Za-z0-9_]+/ {
	code[code_i++] = 17
	process_jump(0)
	next
}
# ================

/^[ \t]*[A-Za-z0-9_]+:/ { # a label
	split($1, labels, /:/)
	jump_map[labels[1]] = code_i
	next
}

/^[ \t]*$/ {
	# skip empty lines
}

# anything else
{
	printf("Invalid syntax line %d: %s\n", NR, $0) > "/dev/stderr"
	exit 1
}

function process_args(      n, arg) {
	for (n=2; n<=NF; n++) {
		arg = $n
		if (arg ~ /^\(.*\)$/) { arg = substr(arg, 2, length(arg) - 2) }
		process_reg(arg)
	}
}

function process_jump(extra_arg_count,      n, arg) {
	for (n = 2; n < 2 + extra_arg_count; n++) {
		arg = $n
		process_reg(arg)
	}
	jumps[++jump_i] = code_i
	code[code_i++] = $n # the label itself for now
}

function process_reg(arg) {
	if (arg ~ /^\$/) {
		arg = substr(arg, 2)
		if (arg > 7) {
			printf("Invalid register line %d: %s\n", NR, arg) > "/dev/stderr"
		}
		code[code_i++] = 2^15 + arg
	} else {
		if (arg > 32767) {
			printf("Invalid value line %d: %s\n", NR, arg) > "/dev/stderr"
		}
		code[code_i++] = arg
	}
}

END {
	# process jumps
	for (i = 1; i <= jump_i; i++) {
		codi = jumps[i]
		label = code[codi]
		code[codi] = jump_map[label]
	}

	# output machine code
	for (i = 0; i < code_i; i++) {
		c = code[i]
		printf("%c%c", c % 256, c / 256)
	}
}
