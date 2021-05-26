#!/usr/bin/awk -f
BEGIN {
	dir_map["N"] = "north"
	dir_map["S"] = "south"
	dir_map["E"] = "east"
	dir_map["W"] = "west"
}

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

	if (weight == end_weight) ok = "ok"
	else ok = ""
	if (end_i == i && end_j == j && weight == end_weight) {
		ok = ok " OK"
		for (z = 1; z <= length(dir); ++z) {
			print dir_map[substr(dir, z, 1)]
		}
	}
	if (ok != "") {
		print dir, "->", txt, "->", weight, ok
	}
	if (end_i != i || end_j != j) {
		if (i > start_i && (i-1 != start_i || j != start_j)) go(i - 1, j, weight, oper, dir "W", txt)
		if (i < end_i) go(i + 1, j, weight, oper, dir "E", txt)
		if (j > end_j) go(i, j - 1, weight, oper, dir "N", txt)
		if (j < start_j && (i != start_i || j-1 != start_j)) go(i, j + 1, weight, oper, dir "S", txt)
	} # visiting the last room is only allowed once, stop the search
}
