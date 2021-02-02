#include<iostream>
#include<fstream>
#include<cstdint>
#include<stack>
typedef std::uint16_t word;
constexpr word max = 0x7fff;
word memory[max+1];
word regs[8];
struct wordStack: std::stack<word>
{
	using std::stack<word>::c; // the underlying container
} stack;

static word readWord(word const*p)
{
	std::uint8_t const*const x = reinterpret_cast<std::uint8_t const*>(p);
	return x[0] | (x[1] << 8);
}
static void writeWord(word *p, word w)
{
	std::uint8_t *const x = reinterpret_cast<std::uint8_t *>(p);
	x[0] = w;
	x[1] = w >> 8;
}
static word readReg(word w)
{
	if (w > max) return regs[w - max - 1];
	else return w;
}
static void writeReg(word w, word value)
{
	if (w > max) regs[w - max - 1] = value;
	else return;
}

#define SAVESTATE_BIN "savestate.bin"

static void saveState(const word pc)
{
	std::ofstream out(SAVESTATE_BIN, std::ios::binary);
	out.write(reinterpret_cast<const char*>(&memory), sizeof memory);
	for (word const&reg: regs)
	{
		out.write(reinterpret_cast<const char*>(&reg), sizeof reg);
	}
	out.write(reinterpret_cast<const char*>(&pc), sizeof pc);
	for (word const&w: stack.c)
	{
		out.write(reinterpret_cast<const char*>(&w), sizeof w);
	}
}

static bool loadState(word &pc)
{
	std::ifstream in(SAVESTATE_BIN, std::ios::binary);
	if (not in) return false;
	in.read(reinterpret_cast<char*>(&memory), sizeof memory);
	for (word &reg: regs)
	{
		in.read(reinterpret_cast<char*>(&reg), sizeof reg);
	}
	in.read(reinterpret_cast<char *>(&pc), sizeof pc);
	word w;
	while (in.read(reinterpret_cast<char*>(&w), sizeof w))
	{
		stack.push(w);
	}
	return true;
}

enum instruction
{
	i_halt,
	i_set,
	i_push,
	i_pop,
	i_eq,
	i_gt,
	i_jmp,
	i_jt,
	i_jf,
	i_add,
	i_mult,
	i_mod,
	i_and,
	i_or,
	i_not,
	i_rmem,
	i_wmem,
	i_call,
	i_ret,
	i_out,
	i_in,
	i_noop
};

int run(word pc)
{
#define NEXTWORD readWord(&memory[pc++ & max])
	for ( ;; )
	{
		switch (NEXTWORD)
		{
		case i_halt:
			return 0;
		case i_set:
			{
				const word a = NEXTWORD;
				const word b = NEXTWORD;
				writeReg(a, readReg(b));
			} break;
		case i_push:
			stack.push(readReg(NEXTWORD));
			break;
		case i_pop:
			if (not stack.empty())
			{
				const word w = stack.top();
				stack.pop();
				writeReg(NEXTWORD, w);
			}
			else
			{
				std::cerr << "Error: empty stack\n";
				return 2;
			}
			break;
		case i_eq:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b == c);
			} break;
		case i_gt:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b > c);
			} break;
		case i_jmp:
			{
				const word a = readReg(NEXTWORD);
				pc = a;
			} break;
		case i_jt:
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				if (a != 0) pc = b;
			} break;
		case i_jf:
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				if (a == 0) pc = b;
			} break;
		case i_add:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, (b + c) & max);
			} break;
		case i_mult:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, (b * c) & max);
			} break;
		case i_mod:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b % c);
			} break;
		case i_and:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b & c);
			} break;
		case i_or:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b | c);
			} break;
		case i_not:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				writeReg(a, (~b) & max);
			} break;
		case i_rmem:
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				writeReg(a, readWord(&memory[b & max]));
			} break;
		case i_wmem:
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				writeWord(&memory[a & max], b);
			} break;
		case i_call:
			{
				const word a = readReg(NEXTWORD);
				stack.push(pc);
				pc = a;
			} break;
		case i_ret:
			if (not stack.empty())
			{
				pc = stack.top();
				stack.pop();
			}
			else
			{
				return 0; // halt
			}
			break;
		case i_out:
			std::cout << static_cast<unsigned char>(readReg(NEXTWORD)) << std::flush;
			break;
		case i_in:
			{
				char c;
				if (std::cin.read(&c, 1)) // input was ok
				{
					writeReg(NEXTWORD, static_cast<unsigned char>(c));
				}
				else // probably EOF
				{
					saveState(pc - 1); // restart i_in when loading state
					return 0; // halt
				}
			} break;
		case i_noop:
			break;
		}
	}
}

int main(int argc, char *argv[])
{
	std::ios::sync_with_stdio(false);
	word pc = 0;
	if (argc < 2)
	{
		if (not loadState(pc))
		{
			std::cerr << "Need either a argument with a program or a saved state.\n";
			return 1;
		}
		// else carry on
	}
	else
	{
		std::ifstream in(argv[1], std::ios::binary);
		in.read(reinterpret_cast<char *>(memory), sizeof memory);
		if (in.gcount() == 0)
		{
			std::cerr << "Could not read the program in `" << argv[1] << "'\n";
			return 1;
		}
	}

	return run(pc);
}
