#!/usr/bin/awk -f
BEGIN {
	coins[1]=2
	coins[2]=3
	coins[3]=5
	coins[4]=7
	coins[5]=9
	n_coins=5
	split("red corroded shiny concave blue", names)
	
	for (a=1;a<=n_coins;a++)
	for (b=1;b<=n_coins;b++)
	if (b != a)
	for (c=1;c<=n_coins;c++)
	if (c != a && c != b)
	for (d=1;d<=n_coins;d++)
	if (d != a && d != b && d != c)
	for (e=1;e<=n_coins;e++)
	if (e != a && e != b && e != c && e != d) {
		if (coins[a] + coins[b] * coins[c]^2 + coins[d]^3 - coins[e] == 399) {
			print names[a], names[b], names[c], names[d], names[e]
			split(sprintf("%s %s %s %s %s", names[a], names[b], names[c], names[d], names[e]), solution)
			for (s = 1; s <= n_coins; ++s) print "use", solution[s], "coin"
		}
	}
}
