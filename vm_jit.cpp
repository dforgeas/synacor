#include<initializer_list>
#include<iostream>
#include<fstream>
#include<cstdint>
#include<cstring>
#include<cstddef>
#include<stack>
#include<sys/mman.h>
typedef std::uint16_t word;
constexpr word max = 0x7fff;
extern "C"
{
word memory[max+1] = {};
word regs[8] = {};
}
static struct wordStack: std::stack<word>
{
	using std::stack<word>::c; // the underlying container
} stack;

static word readWord(word const*const p)
{
	std::uint8_t const*const x = reinterpret_cast<std::uint8_t const*>(p);
	return x[0] | (x[1] << 8);
}
static void writeWord(word *const p, const word w)
{
	std::uint8_t *const x = reinterpret_cast<std::uint8_t *>(p);
	x[0] = w;
	x[1] = w >> 8;
}
static word readReg(const word w)
{
	const word result = (w > max) ? regs[w - max - 1] : w;
	return result;
}
static void writeReg(const word w, const word value)
{
	if (w > max) regs[w - max - 1] = value;
	else return;
}

#define SAVESTATE_BIN "savestate.bin"

static void saveState(const word pc)
{
	std::ofstream out(SAVESTATE_BIN, std::ios::binary);
	out.write(reinterpret_cast<const char*>(&memory), sizeof memory);
	auto putWord = [&out](const word w)
	{
		out.put(w & 0xff).put(w >> 8);
	};
	for (word const&reg: regs)
	{
		putWord(reg);
	}
	putWord(pc);
	for (word const&w: stack.c)
	{
		putWord(w);
	}
}

static bool loadState(word &pc)
{
	std::ifstream in(SAVESTATE_BIN, std::ios::binary);
	if (not in) return false;
	in.read(reinterpret_cast<char*>(&memory), sizeof memory);
	auto getWord = [&in]() -> word
	{
		char x, y;
		if (in.get(x).get(y)) return (unsigned char)x | ((unsigned char)y << 8);
		else return 0xffff;
	};
	for (word &reg: regs)
	{
		reg = getWord();
	}
	pc = getWord();
	word w;
	while ((w = getWord()) != 0xffff)
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

static word nextProgramWord(word &pc)
{
	const auto w = readWord(&memory[pc++ & max]);
	return w;
}

extern "C"
{ // put the interface with assembly here, to disable C++ complications if ever

enum Errors
{
	// ARM immediate fields support one byte worth of non-zero bits
	// anything after 0x8008 is invalid according to the challenge
	err_halt = 0xff00
};

// Do not let exceptions pass into our machine code.
// Non-throwing functions are permitted to call potentially-throwing functions. Whenever an exception is thrown and the search for a handler encounters the outermost block of a non-throwing function, the function std::terminate [or std::unexpected (until C++17)] is called.
void putchar_impl(word w) noexcept
{
	const auto c = static_cast<unsigned char>(w);
	std::cout << c;
	// don't flush here since cout will be flushed when cin reads
}

word getchar_impl(word pc) noexcept
{
// pc can just be computed by the JIT compiler statically at the location of the i_in instruction most surely
// that is to say the machine code doesn't need to track pc everywhere.
	char c;
	if (std::cin.get(c)) // input was ok
	{
		return static_cast<unsigned char>(c);
	}
	else // probably EOF
	{
		saveState(pc - 1); // restart i_in when loading state
		return err_halt;
	}
}

void stack_push_impl(word w) noexcept
{
	stack.push(w);
}

word stack_pop_impl(void) noexcept
{
	if (not stack.empty())
	{
		const word w = stack.top();
		stack.pop();
		return w;
	}
	else
	{
		std::cerr << "Error: empty stack\n";
		return err_halt; // do we return something else? like 0xf100
	}
}

struct Callbacks;
typedef int (*machine_code_ptr)(word *memory, word *regs, Callbacks *);
static machine_code_ptr machine_code;

void write_mem_impl(word addr, word value) noexcept
{
	writeWord(&memory[addr], value);
	if (value < i_noop)
	{ // TODO: look up and compile the equivalent machine code
	}
}

static struct Callbacks
{
	void (*putchar)(word) noexcept;
	word (*getchar)(word pc) noexcept;
	void (*stack_push)(word) noexcept;
	word (*stack_pop)(void) noexcept;
	void (*write_mem)(word addr, word value) noexcept;
} callbacks = {
	putchar_impl,
	getchar_impl,
	stack_push_impl,
	stack_pop_impl,
	write_mem_impl,
};

} // extern "C"

static class WillUnmap
{
	void *ptr = 0;
	std::size_t length;
public:
	~WillUnmap()
	{
		if (ptr) munmap(ptr, length);
	}
	void set(void *p, std::size_t len)
	{
		ptr = p;
		length = len;
	}
} willUnmap;

constexpr int INSTR_PER_WORD = 4; // a word is either an instruction or an argument, so instructions with arguments get more space
constexpr auto CODE_SIZE = (max+1) * INSTR_PER_WORD;
constexpr auto CODE_SIZE_IN_BYTES = CODE_SIZE * sizeof(std::uint32_t);

namespace arm
{
	typedef std::uint32_t instr;
	constexpr int fp = 11, ip = 12, sp = 13, lr = 14, pc = 15; // TODO: make an enum
	constexpr instr movi(int rd, int val) // TODO: rename to move and use the enum to overload
	{
		if (val < 0 or val > 0xff)
			throw "invalid value in movi";
		return 0xe3a0'0000 | rd << 12 | val;
	}
	constexpr instr orri_hi(int rd, int val)
	{
		if (val < 0 or val > 0xff)
			throw "invalid value in orri_hi";
		return 0xe380'0c00 | rd << 12 | rd << 16 | val;
	}
	constexpr instr movr(int rd, int rs) { return 0xe1a0'0000 | rd << 12 | rs; }
	constexpr instr nop() { return movr(0, 0); }
	enum condition: instr
	{
		EQ, NE, HS, LO, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL
	};
	static_assert(GT == 0b1100 and  AL == 0b1110);
	enum operation: instr
	{
		AND, EOR, SUB, RSB, ADD, ADC, SBC, RSC, TST, TEQ, CMP, CMN, ORR, MOV, BIC, MVN
	};
	static_assert(ADD == 0b0100 and CMP == 0b1010 and ORR == 0b1100 and MVN == 0b1111);
	constexpr instr S = 1 << 20, I = 1 << 25;
	constexpr instr lsl(int shift)
	{
		return shift << 7 | 0b000 << 4;
	}
	constexpr instr op(condition cond, operation oper, int rd, int rn, int rs)
	{
		return cond << 28 | oper << 21 | rn << 16 | rd << 12 | rs
			| (oper >= TST and oper <= CMN) << 20; // force S for them
	}
	constexpr instr smulbb(int rd, int rs, int rm)
	{
		return 0xe160'0080 | rd << 16 | rs << 8 | rm;
	}
	constexpr instr push(std::initializer_list<int> regs)
	{
		instr i = 0xe92d'0000;
		for (int r: regs) i |= 1 << r;
		return i;
	}
	constexpr instr pop(std::initializer_list<int> regs)
	{
		instr i = 0xe8bd'0000;
		for (int r: regs) i |= 1 << r;
		return i;
	}
	constexpr instr ldr(int rd, int rbase, int off_b)
	{
		instr i = 0xe590'0000 | rbase << 16 | rd << 12;
		if (off_b < 0)
		{
			i &= ~(1 << 23); // clear the 'up=add' bit to mean 'down=substract'
			off_b = -off_b;
		}
		if (off_b >= 0x1000) // just 12 bits are available
			throw "byte offset overflow in ldr";
		return i | off_b;
	}
	constexpr instr ldrpc(int rd, int off_b)
	{
		return ldr(rd, pc, off_b - 8); // undo the pipeline delay on pc
	}
	constexpr instr ldrh(int rd, int rbase, int off_b)
	{
		// L=1, S=0, H=1, W=0, U=1
		instr i = 0xe1d0'00b0 | rd << 12 | rbase << 16;
		if (off_b < 0)
		{
			i &= ~(1 << 23); // clear the 'up=add' bit to mean 'down=substract'
			off_b = -off_b;
		}
		if (off_b >= 0x100) // just 8 bits are available
			throw "byte offset overflow in ldrh";
		return i | (off_b & 0xf0) << 4 | (off_b & 0xf);
	}
	constexpr instr strh(int rd, int rbase, int off_b)
	{
		// L=0
		const auto L = S;
		return ldrh(rd, rbase, off_b) & ~L; // clear the L bit
	}
	constexpr instr blx(int rm) { return 0xe12f'ff30 | rm; }
	constexpr instr bx(condition cond, int rm) { return 0x012f'ff10 | cond << 28 | rm; }
	constexpr instr b(condition cond, int off_i)
	{
		off_i -= 2; // undo the pipeline delay on pc
		if ((off_i & 0xff00'0000) != 0xff00'0000 and (off_i & 0xff00'0000) != 0)
			throw "instruction offset overflow in branch";
		return 0x0a00'0000 | cond << 28 | (off_i & 0xff'ffff);
	}
}

static int run(word pc) noexcept
try
{
	void *const code = mmap(0, CODE_SIZE_IN_BYTES,
		PROT_READ | PROT_WRITE | PROT_EXEC,
		MAP_PRIVATE | MAP_ANONYMOUS,
		-1, 0);
	if (code == MAP_FAILED)
	{
		std::cerr << "Failed allocating memory for machine code: " << std::strerror(errno) << '\n';
		return -1;
	}
	willUnmap.set(code, CODE_SIZE_IN_BYTES);
	machine_code = reinterpret_cast<machine_code_ptr>(code);

	arm::instr *code_p = static_cast<arm::instr *>(code);
	constexpr arm::instr nop = arm::nop();
	for (int i = 0; i < CODE_SIZE; i++)
		code_p[i] = nop;

	if (memory[0] != i_noop or memory[1] != i_noop)
	{
		std::cerr << "Broken expectation: the memory doesn't start with 2 noops\n";
		return -2;
	}
	// now we can write our function prologue in the place of the first and second noops
	// 4, 5, 6 are the machine_code arguments: memory, regs, &callbacks
	// 7 is the machine_code pointer
	// 8 is max = 0x7fff
	constexpr arm::instr push = arm::push({4, 5, 6, 7, 8, arm::lr});
	*code_p++ = push;
	const int rmemory = 0 + 4, rregs = 1 + 4, rcallbacks = 2 + 4, rmachine_code = 3 + 4, rmax = 4 + 4;
	// get the machine_code pointer by pointing at the push instruction
	constexpr arm::instr get_machine_code = arm::op(arm::AL, arm::SUB, rmachine_code, arm::pc, 8 + 4) | arm::I;
	*code_p++ = get_machine_code;
	// save the arguments in callee-saved registers
	#if 0
	for (int x: {0, 1, 2})
	{
		*code_p++ = arm::movr(x + 4, x);
	}
	#else // do it in 2 instructions but with the stack
	constexpr arm::instr push_args = arm::push({0, 1, 2});
	constexpr arm::instr pop_args = arm::pop({rmemory, rregs, rcallbacks});
	*code_p++ = push_args;
	*code_p++ = pop_args;
	#endif
	*code_p++ = arm::movi(rmax, 0xff);
	*code_p++ = arm::orri_hi(rmax, 0x7f);

	// function epilogue
	constexpr arm::instr ret = arm::pop({4, 5, 6, 7, 8, arm::pc});

	auto readReg = [&code_p](const int dst, const word src)
	{
		if (src > max)
		{
			*code_p++ = arm::ldrh(dst, rregs, (src - max - 1) * sizeof src);
		}
		else
		{
			*code_p++ = arm::movi(dst, src & 0xff);
			const int high = src >> 8;
			if (high) *code_p++ = arm::orri_hi(dst, high);
		}
	};
	auto writeReg = [&code_p](const word dst, const int src)
	{
		if (dst > max)
		{
			*code_p++ = arm::strh(src, rregs, (dst - max - 1) * sizeof dst);
		}
	};
	auto condJump = [&code_p, readReg](arm::condition cond, int extra, int pc, const word w)
	{
		if (w > max)
		{
			readReg(arm::ip, w);
			static_assert(INSTR_PER_WORD * sizeof(arm::instr) == 16); // confirm lsl 4 works
			constexpr arm::instr a1 = arm::op(arm::AL, arm::ADD, arm::ip, rmachine_code, arm::ip) | arm::lsl(4);
			*code_p++ = a1;
			*code_p++ = arm::bx(cond, arm::ip);
		}
		else
		{
			const int off_i = (w - pc) * INSTR_PER_WORD - extra;
			*code_p++ = arm::b(cond, off_i);
		}
	};
	auto ternary = [&code_p, readReg, writeReg](int &i, arm::instr oper(int, int, int), bool mask)
	{
		const word a = memory[++i];
		const word b = memory[++i];
		const word c = memory[++i];
		readReg(0, b);
		readReg(1, c);
		*code_p++ = oper(1, 0, 1);
		if (mask) *code_p++ = arm::op(arm::AL, arm::AND, 1, 1, rmax);
		writeReg(a, 1);
	};

	for (int i = 2; i <= max; i++)
	{
		const word w = memory[i];
		code_p = static_cast<arm::instr *>(code) + INSTR_PER_WORD * i;
		switch (w)
		{
			case i_halt: // 0 arguments: code size is INSTR_PER_WORD
			{
				constexpr arm::instr m = arm::movi(0, 0); // r0 = 0
				*code_p++ = m;
				*code_p++ = ret;
				break;
			}
			case i_noop: // 0 arguments
				break;
			case i_in: // 1 argument: code size is 2 * INSTR_PER_WORD
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, getchar));
				*code_p++ = l1;
				// pass the program counter as the argument (in order to savestate on EOF)
				*code_p++ = arm::movi(0, i & 0xff);
				const int high = i >> 8;
				if (high) *code_p++ = arm::orri_hi(0, i >> 8);
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = b1;
				// halt if err_halt was returned
				constexpr arm::instr c1 = arm::op(arm::AL, arm::CMP, 0, 0, err_halt >> 8 | 0xc'00) | arm::I;
				*code_p++ = c1;
				constexpr arm::instr r1 = ret & 0xfff'ffff | arm::EQ;
				*code_p++ = r1;
				break;
			}
			case i_out: // 1 argument: code size is 2 * INSTR_PER_WORD
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, putchar));
				*code_p++ = l1;
				const word w = memory[++i];
				readReg(0, w);
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = b1;
				break;
			}
			case i_jmp: // 1 argument: code size is 2 * INSTR_PER_WORD
			{
				const auto pc = i;
				const word w = memory[++i];
				const int extra = 0;
				condJump(arm::AL, extra, pc, w);
				break;
			}
			case i_jt: // 2 arguments: code size is 3 * INSTR_PER_WORD
			{
				const auto pc = i;
				const word a = memory[++i];
				const word b = memory[++i];
				readReg(0, a);
				// reuse the register encoding but actually make it read it as an immediate zero value:
				constexpr arm::instr c1 = arm::op(arm::AL, arm::CMP, 0, 0, 0) | arm::I;
				*code_p++ = c1;
				const int extra = (a <= max) + 2;
				condJump(arm::NE, extra, pc, b);
				break;
			}
			case i_jf: // 2 arguments: code size is 3 * INSTR_PER_WORD
			{
				const auto pc = i;
				const word a = memory[++i];
				const word b = memory[++i];
				readReg(0, a);
				// reuse the register encoding but actually make it read it as an immediate zero value:
				constexpr arm::instr c1 = arm::op(arm::AL, arm::CMP, 0, 0, 0) | arm::I;
				*code_p++ = c1;
				const int extra = (a <= max) + 2;
				condJump(arm::EQ, extra, pc, b);
				break;
			}
			case i_set: // 2 arguments: code size is 3 * INSTR_PER_WORD
			{
				const word a = memory[++i];
				const word b = memory[++i];
				readReg(0, b);
				writeReg(a, 0);
				break;
			}
			case i_add: // 3 arguments
			case i_mult: // 3 arguments
			case i_and: // 3 arguments
			case i_or: // 3 arguments
			{
				switch (w)
				{
				case i_add: ternary(i, [](int a, int b, int c) {
						return arm::op(arm::AL, arm::ADD, a, b, c);
					}, true); break;
				case i_and: ternary(i, [](int a, int b, int c) {
						return arm::op(arm::AL, arm::AND, a, b, c);
					}, false); break;
				case i_or: ternary(i, [](int a, int b, int c) {
						return arm::op(arm::AL, arm::ORR, a, b, c);
					}, false); break;
				case i_mult: ternary(i, [](int a, int b, int c) {
						return arm::smulbb(a, b, c);
					}, true); break;
				}
				break;
			}
			case i_eq: // 3 arguments
			case i_gt: // 3 arguments
			{
				const word a = memory[++i];
				const word b = memory[++i];
				const word c = memory[++i];
				readReg(0, b);
				readReg(1, c);
				*code_p++ = arm::op(arm::AL, arm::CMP, 0, 0, 1);
				auto cond0 = arm::NE, cond1 = arm::EQ;
				if (w == i_gt) cond0 = arm::LS, cond1 = arm::HI;
				*code_p++ = arm::op(cond0, arm::MOV, 0, 0, 0) | arm::I;
				*code_p++ = arm::op(cond1, arm::MOV, 0, 0, 1) | arm::I;
				writeReg(a, 0);
				break;
			}
			case i_push: // 1 argument
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, stack_push));
				*code_p++ = l1;
				const word a = memory[++i];
				readReg(0, a);
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = b1;
				break;
			}
			case i_pop: // 1 argument
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, stack_pop));
				*code_p++ = l1;
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = b1;
				const word a = memory[++i];
				writeReg(a, 0);
				// halt if err_halt was returned
				constexpr arm::instr c1 = arm::op(arm::AL, arm::CMP, 0, 0, err_halt >> 8 | 0xc'00) | arm::I;
				*code_p++ = c1;
				constexpr arm::instr r1 = ret & 0xfff'ffff | arm::EQ;
				*code_p++ = r1;
				break;
			}
			default: // special halt
				*code_p++ = arm::movi(0, 0xf7);`
				*code_p++ = ret;
				break;
		}
	}
	__builtin___clear_cache(code, (char*)code + CODE_SIZE_IN_BYTES);
#if 0
	{
		std::ofstream machine_code_out("machine_code.bin", std::ios::binary);
		machine_code_out.write((const char*)code, CODE_SIZE_IN_BYTES);
	}
#endif
	return machine_code(memory, regs, &callbacks);
#if 0
#define NEXTWORD nextProgramWord(pc)
	auto ternary = [&pc] (auto&& oper) {
		const word a = NEXTWORD;
		const word b = readReg(NEXTWORD);
		const word c = readReg(NEXTWORD);
		writeReg(a, oper(b, c));
	};
	for ( ;; )
	{
		switch (NEXTWORD)
		{
		case i_halt:
			
			break;
		case i_set:
			{
				const word a = NEXTWORD;
				const word b = NEXTWORD;
				writeReg(a, readReg(b));
			} break;
		case i_push:
			break;
		case i_pop:
			break;
		case i_eq:
			{
				ternary([](word b, word c){ return b == c; });
			} break;
		case i_gt:
			{
				ternary([](word b, word c){ return b > c; });
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
				ternary([](word b, word c){ return (b + c) & max; });
			} break;
		case i_mult:
			{
				ternary([](word b, word c){ return (b * c) & max; });
			} break;
		case i_mod:
			{
				ternary([](word b, word c){ return b % c; });
			} break;
		case i_and:
			{
				ternary([](word b, word c){ return b & c; });
			} break;
		case i_or:
			{
				ternary([](word b, word c){ return b | c; });
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
			{
			} break;
		case i_in:
			{
			} break;
		case i_noop:
			break;
		}
	}
#endif
}
catch (const char *err)
{
	std::cerr << err << '\n';
	return -2;
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
