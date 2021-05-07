#!/usr/bin/awk -f

BEGIN {
	ord_index = ""
	# initialize our character code function
	for (i = 1; i < 256; i+=4) ord_index = sprintf("%s%c%c%c%c", ord_index, i, 1+i, 2+i, 3+i) # not found is 0
}

{
	for (i = 1; i <= length($0); ++i) {
		printf("out %d\n", index(ord_index, substr($0, i, 1)))
	}
	print ""
}
