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
	for (c=1;c<=n_coins;c++)
	for (d=1;d<=n_coins;d++)
	for (e=1;e<=n_coins;e++) {
		if (coins[a] + coins[b] * coins[c]^2 + coins[d]^3 - coins[e] == 399) {
			print names[a], names[b], names[c], names[d], names[e]
		}
	}
}
