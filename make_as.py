#!/usr/bin/env python3

instructions = (
    # (name, n_args, mem_arg)
    ("halt", 0, 0),
    ("set", 2, 0),
    ("push", 1, 0),
    ("pop", 1, 0),
    ("eq", 3, 0),
    ("gt", 3, 0),
    ("jmp", 1, 0),
    ("jt", 2, 0),
    ("jf", 2, 0),
    ("add", 3, 0),
    ("mult", 3, 0),
    ("mod", 3, 0),
    ("and", 3, 0),
    ("or", 3, 0),
    ("not", 2, 0),
    ("rmem", 2, 2),
    ("wmem", 2, 1),
    ("call", 1, 0),
    ("ret", 0, 0),
    ("out", 1, 0),
    ("in", 1, 0),
    ("noop", 0, 0)
)

with open('as.awk') as f:
    contents = list(f) # read lines
cut_start = contents.index('# ================\n')
cut_end = contents.index('# ================\n', cut_start + 1)

as_lines = []

# all instructions can take immediate value or register arguments
for (idx, (name, n_args, mem_arg)) in enumerate(instructions):
    as_lines.extend((
        (r'/^[ \t]*%s%s/ {' '\n') % (name, ''.join(
            r'[ \t]+%s%s%s' %
            (r'\('[0:2*(i==mem_arg-1)],
             (r'\$?[0-9]+'),
             r'\)'[0:2*(i==mem_arg-1)])
            for i in range(n_args))),
        '\tcode[code_i++] = %d\n' % idx,
        ('\tprocess_args()\n' if n_args else ''),
        '\tnext\n',
        '}\n'))
# jumps and set can take a label as their last argument
for (idx, (name, n_args, _)) in enumerate(instructions):
    if name[0]=='j' or name in ('call', 'set'):
        as_lines.extend((
            (r'/^[ \t]*%s%s/ {' '\n') % (name, ''.join(
                r'[ \t]+%s' % ('[A-Za-z_][A-Za-z0-9_]+' if i==n_args-1 else r'\$?[0-9]+')
                for i in range(n_args))),
            '\tcode[code_i++] = %d\n' % idx,
            '\tprocess_jump(%d)\n' % (n_args - 1),
            '\tnext\n',
            '}\n'))

# insert the generated code, keep the = markers
contents[cut_start + 1: cut_end] = as_lines

with open('as.awk', 'w') as f:
	for l in contents:
		f.write(l)
