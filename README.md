# ppop

ppop is an esoteric programming language for pipe-dream parallelism.

## syntax

ppop has almost minimal syntax.

- the script must be a vaild UTF8 text file.
- the number of graphemes in the script must be a multiple of 4.

## semantics

the semantics of ppop is very similar to machine languages. every 4 unicode grapheme tuple is treated as an instruction. the difference is that all instruction is executed not once but . each of them is translated as define (`L1`, `L2`, `OP`, `LO`) as the tuple.

### agent and pipe

all 

- if `OP` is a kind of `IOP`, then 

## list of operator

### load immediate

| symbol | type |
| --- | --- |
|`b`| `bool` |
|`x`| `u8` |
|`u`| `u64` |
|`i`| `i64` |
|`f`| `float` |
|`U`| `usize` |
|`I`| `isize` |
|`S`| `String` |

### arithmetics

- `+`
- `-`
- `*`
- `/`
- `%`

### stdio

- `P` print to stdout
- `p` print to stderr
