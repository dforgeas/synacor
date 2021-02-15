#include <iostream>
#include <iomanip>
#include <fstream>
#include <string>

struct i_def
{
	const char *name;
	int n_args;
	int mem_arg;
} defs[] =
{
	{"halt", 0, 0},
	{"set", 2, 0},
	{"push", 1, 0},
	{"pop", 1, 0},
	{"eq", 3, 0},
	{"gt", 3, 0},
	{"jmp", 1, 0},
	{"jt", 2, 0},
	{"jf", 2, 0},
	{"add", 3, 0},
	{"mult", 3, 0},
	{"mod", 3, 0},
	{"and", 3, 0},
	{"or", 3, 0},
	{"not", 2, 0},
	{"rmem", 2, 2},
	{"wmem", 2, 1},
	{"call", 1, 0},
	{"ret", 0, 0},
	{"out", 1, 0},
	{"in", 1, 0},
	{"noop", 0, 0}
};
constexpr auto n_defs = sizeof defs / sizeof *defs;

static std::string reg(unsigned char x, unsigned char y)
{
	if (0x80 & y) return '$' + std::to_string(x);
	else return std::to_string(x | (y << 8));
}

int main(int argc, char *argv[])
{
	std::ios::sync_with_stdio(false);
	using std::setw;
	if (argc < 2)
	{
		std::cerr << "Missing filename argument.\n";
		return 1;
	}
	std::ifstream in(argv[1], std::ios::binary);
	char x[2];
	for (int offset = 0; in.read(x, sizeof x); offset++)
	{
		const unsigned char x0 = x[0];
		if (x[1] != 0 or x0 >= n_defs) continue; // not an instruction
		const auto &def = defs[x0];
		std::cout << setw(8) << offset << ": " << def.name;
		for (int i = 0; i < def.n_args; i++)
		{
			std::cout << ' ';
			if (i + 1 == def.mem_arg) std::cout << '(';
			if (not in.read(x, sizeof x)) return 2;
			offset++, std::cout << reg(x[0], x[1]);
			if (i + 1 == def.mem_arg) std::cout << ')';
		}
		// now parse and print the optional comments
		while (in.peek() == 0xff)
		{
			in.get(); // 0xff
			if (in.peek() == 0xfe)
			{
				in.get(); // 0xfe, making a BOM: 0xFEFF
				std::string comment;
				std::getline(in, comment, '\0');
				std::cout << " # " << comment;
			}
			else
			{
				in.unget(); // put back 0xff
				break; // exit the loop: don't try again
			}
		}
		std::cout << '\n';
	}
}
