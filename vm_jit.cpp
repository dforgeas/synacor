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
static word memory[max+1];
static word regs[8];
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

enum Errors
{
	// ARM immediate fields support one byte worth of non-zero bits
	// anything after 0x8008 is invalid according to the challenge
	err_halt = 0xff00
};


// Do not let exceptions pass into our machine code.
// Non-throwing functions are permitted to call potentially-throwing functions. Whenever an exception is thrown and the search for a handler encounters the outermost block of a non-throwing function, the function std::terminate [or std::unexpected (until C++17)] is called.
extern "C" void putchar_impl(word w) noexcept
{
	const auto c = static_cast<unsigned char>(w);
	std::cout << c;
	// don't flush here since cout will be flushed when cin reads
}

extern "C" word getchar_impl(word pc) noexcept
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

extern "C" void stack_push_impl(word w) noexcept
{
	stack.push(w);
}

extern "C" word stack_pop_impl(void) noexcept
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

extern "C" void write_mem_impl(word addr, word value) noexcept
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

constexpr int INSTR_PER_WORD = 6; // a word is either an instruction or an argument, so instructions with arguments get more space
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
		return 0xe3a00000 | rd << 12 | val;
	}
	constexpr instr movr(int rd, int rs) { return 0xe1a00000 | rd << 12 | rs; }
	constexpr instr nop() { return movr(0, 0); }
	constexpr instr push(std::initializer_list<int> regs)
	{
		instr i = 0xe92d0000;
		for (int r: regs) i |= 1 << r;
		return i;
	}
	constexpr instr pop(std::initializer_list<int> regs)
	{
		instr i = 0xe8bd0000;
		for (int r: regs) i |= 1 << r;
		return i;
	}
	constexpr instr ldr(int rd, int rbase, int off)
	{
		instr i = 0xe5900000 | rbase << 16;
		if (off < 0)
		{
			i &= ~(1 << 23); // clear the 'up=add' bit to mean 'down=substract'
			off = -off;
		}
		if (off >= 0x1000) // just 12 bits are available
			throw "offset overflow in ldr";

		return i | rd << 12 | off;
	}
	constexpr instr ldrpc(int rd, int off)
	{
		return ldr(rd, pc, off - 8); // undo the pipeline delay on pc
	}
	constexpr instr blx(int rm) { return 0xe12fff30 | rm; }
}

static int run(word pc)
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

	if (memory[0] != i_noop)
	{
		std::cerr << "Broken expectation: the memory doesn't start with a noop\n";
		return -2;
	}
	// now we can write our function prologue in the place of the first noop
	// function epilogue
	constexpr arm::instr push = arm::push({4, 5, 6, 7, 8, arm::lr});
	*code_p++ = push;
	// save the arguments in callee-saved registers
	for (int x: {0, 1, 2})
	{
		*code_p++ = arm::movr(x + 4, x);
	}
	constexpr int rcallbacks = 2 + 4;
	constexpr arm::instr ret = arm::pop({4, 5, 6, 7, 8, arm::pc});

	for (int i = 0; i <= max; i++)
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
			case i_in: // 1 argument: code size is 2 * INSTR_PER_WORD
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, getchar));
				constexpr arm::instr m1 = arm::movi(0, 0); // replace with actual program counter
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = l1;
				*code_p++ = m1;
				*code_p++ = b1;
				break;
			}
			case i_out: // 1 argument: code size is 2 * INSTR_PER_WORD
			{
				constexpr arm::instr l1 = arm::ldr(arm::ip, rcallbacks, offsetof(Callbacks, putchar));
				constexpr arm::instr m1 = arm::movi(0, '!'); // replace with readReg
				constexpr arm::instr b1 = arm::blx(arm::ip);
				*code_p++ = l1;
				const word w = memory[++i];
				*code_p++ = arm::movi(0, static_cast<unsigned char>(w)); // replace with readReg
				*code_p++ = b1;
				break;
			}
		}
	}
	__builtin___clear_cache(code, (char*)code + CODE_SIZE_IN_BYTES);
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
