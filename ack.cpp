#include<iostream>
#include<unordered_map>
#include<utility>
#include<future>
#include<mutex>
struct hash
{
	std::size_t operator()(const std::pair<short, short> k) const
	{
		return k.first * 05 ^ k.second * 0200001;
	}
};
class MemAck
{
	std::unordered_map<std::pair<short, short>, short, hash> mem;
	short x;
public:
	void reset(const short new_x)
	{
		mem.clear();
		x = new_x;
	}
	short ack(const short m, const short n)
	{
		const auto it = mem.find({m, n});
		if (it != mem.end()) return it->second;
		short r;
		if (m == 0) r = (n+1) & 0x7fff;
		else if (n == 0) r = ack(m-1, x);
		else r = ack(m-1, ack(m, n-1));
		mem[{m, n}] = r;
		return r;
	}
};
constexpr int num_threads = 4;
std::mutex cout_mutex;
void search(const int i)
{
	MemAck memack;
	for (int x = 2 + i; x < 0x8000; x += num_threads)
	{
		memack.reset(x);
		const short a = memack.ack(4, 1);
		std::lock_guard<std::mutex> guard(cout_mutex);
		std::cout << x << " -> " << a;
		if (a == 6) std::cout << " OK";
		std::cout << std::endl;
	}
}
int main()
{
	std::ios::sync_with_stdio(false);
	std::future<void> threads[num_threads];
	for (int nt = 0; nt < num_threads; nt++)
	{
		threads[nt] = std::async(std::launch::async, &search, nt);
	}
	for (auto &f: threads)
	{
		f.get();
	}
}
