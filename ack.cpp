#include<iostream>
#include<unordered_map>
#include<utility>
int x;
struct hash
{
	std::size_t operator()(const std::pair<short, short> k) const
	{
		return k.first * 05 ^ k.second * 0200001;
	}
};
std::unordered_map<std::pair<short, short>, short, hash> mem;
int ack(const int m, const int n)
{
	const auto it = mem.find(std::make_pair(m, n));
	if (it != mem.end()) return it->second;
	int r;
	if (m == 0) r = (n+1) & 0x7fff;
	else if (n == 0) r = ack(m-1, x);
	else r = ack(m-1, ack(m, n-1));
	mem[std::make_pair(m, n)] = r;
	return r;
}
int main()
{
	std::ios::sync_with_stdio(false);
	for (x = 2; x < 0x8000; x++)
	{
		std::cout << x << std::flush;
		mem.clear();
		const int a = ack(4, 1);
		std::cout << " -> " << a;
		if (a == 6) std::cout << " OK";
		std::cout << '\n';
	}
}
