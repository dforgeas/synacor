#!/usr/bin/awk -f
# parses vault_lock.txt
{
	i = 1
	j++
	for (f = 1; f <= NF; f++) {
		if ($f == "|") continue
		if ($f ~ /(.+)=(.+)/) {
			weight = $f
			sub(/(.+)=/, "", weight)
			print "weight", weight
			grid[i, j] = weight
			if (substr($f, 1, 3) == "orb") {
				start_i = i
				start_j = j
			} else if (substr($f, 1, 5) == "vault") {
				end_i = i - 1
				end_j = j
				end_weight = weight
			}
		} else {
			grid[i, j] = $f
		}
		i++
	}
}

# now solve the puzzle
END {
	print "start_i", start_i, " start_j", start_j, " end_i", end_i, " end_j", end_j
	weight = grid[start_i, start_j]
	grid[start_i, start_j] = -999
	go(start_i, start_j, weight, "", "", weight)
}

function go(i, j, weight, oper, dir, txt,    cell) {
	if (length(dir) > 12) return
	cell = grid[i, j]
	if (oper != "") {
		# we've left the start
		if (oper == "-") weight -= cell
		else if (oper == "+") weight += cell
		else if (oper == "*") weight *= cell
		txt = txt " " oper cell
		oper = ""
	} else if (cell ~ /^[-+*]$/) {
		oper = cell
	}

	#if (end_i == i && end_j == j) {
		if (weight == end_weight) ok = "ok"
		else ok = ""
		if (end_i == i && end_j == j && weight == end_weight) ok = ok " OK"
		if (ok != "") {
			print dir, "->", txt, "->", weight, ok
		}
	#} else {
		if (i > 1 && (i-1 != start_i || j != start_j)) go(i - 1, j, weight, oper, dir "W", txt)
		if (i < 4) go(i + 1, j, weight, oper, dir "E", txt)
		if (j > 1 && (i != start_i || j-1 != start_j)) go(i, j - 1, weight, oper, dir "N", txt)
		if (j < 4) go(i, j + 1, weight, oper, dir "S", txt)
	#}
}
