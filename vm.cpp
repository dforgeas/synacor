#include<iostream>
#include<fstream>
#include<cstdint>
#include<stack>
typedef std::uint16_t word;
constexpr word max = 0x7fff;
word memory[max+1];
word regs[8];
std::stack<word> stack;
word readWord(word const*p)
{
	std::uint8_t const*const x = reinterpret_cast<std::uint8_t const*>(p);
	return x[0] | (x[1] << 8);
}
void writeWord(word *p, word w)
{
	std::uint8_t *const x = reinterpret_cast<std::uint8_t *>(p);
	x[0] = w;
	x[1] = w >> 8;
}
word readReg(word w)
{
	if (w > max) return regs[w - max - 1];
	else return w;
}
void writeReg(word w, word value)
{
	if (w > max) regs[w - max - 1] = value;
	else return;
}
#define NEXTWORD readWord(&memory[pc++ & max])
int main(int argc, char *argv[])
{
	std::ios::sync_with_stdio(false);
	if (argc < 2) return 1;
	std::ifstream in(argv[1], std::ios::binary);
	in.read(reinterpret_cast<char *>(memory), sizeof memory);
	word pc = 0;
	for ( ;; )
	{
		switch (NEXTWORD)
		{
		case 0: // halt
			return 0;
		case 1: // set
			{
				const word a = NEXTWORD;
				const word b = NEXTWORD;
				writeReg(a, readReg(b));
			} break;
		case 2: // push
			stack.push(readReg(NEXTWORD));
			break;
		case 3: // pop
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
		case 4: // eq
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b == c);
			} break;
		case 5: // gt
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b > c);
			} break;
		case 6: // jmp
			{
				const word a = readReg(NEXTWORD);
				pc = a;
			} break;
		case 7: // jt
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				if (a != 0) pc = b;
			} break;
		case 8: // jf
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				if (a == 0) pc = b;
			} break;
		case 9: // add
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, (b + c) & max);
			} break;
		case 10: // mult
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, (b * c) & max);
			} break;
		case 11: // mod
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b % c);
			} break;
		case 12: // and
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b & c);
			} break;
		case 13: // or
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				const word c = readReg(NEXTWORD);
				writeReg(a, b | c);
			} break;
		case 14: // not
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				writeReg(a, (~b) & max);
			} break;
		case 15: // rmem
			{
				const word a = NEXTWORD;
				const word b = readReg(NEXTWORD);
				writeReg(a, readWord(&memory[b & max]));
			} break;
		case 16: // wmem
			{
				const word a = readReg(NEXTWORD);
				const word b = readReg(NEXTWORD);
				writeWord(&memory[a & max], b);
			} break;
		case 17: // call
			{
				const word a = readReg(NEXTWORD);
				stack.push(pc);
				pc = a;
			} break;
		case 18: // ret
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
		case 19: // out
			std::cout << static_cast<unsigned char>(readReg(NEXTWORD)) << std::flush;
			break;
		case 20: // in
			{
				char c = 0;
				std::cin.read(&c, 1);
				writeReg(NEXTWORD, static_cast<unsigned char>(c));
			} break;
		case 21: // noop
			break;
		}
	}
}
