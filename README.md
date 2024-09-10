# ppop

ppop is an esoteric programming language for pipe-dream parallelism.

## syntax

ppop has some syntaxes (restriction), since the implementation is WIP.

- the script must be a vaild UTF8 text file.
- the number of graphemes in the script must be a multiple of 4. let the tuple be (`L1`, `L2`, `OP`, `LO`).
- `L1`, `L2`, `LO` is restricted by `OP`:

  - if `OP` is an **I Operator**, then `L1`, `L2`, `LO` must be pipes with proper types deducted from the script.
  - if `OP` is an **Y Operator**, then `L1`, `LO` must be pipes with proper types deducted from the script, and `L2` must be an acceptable immediate.

- pipes `#` & `@` must be typed as `usize` & `String`.

these syntaxes must be removed in future: any vaild UTF8 text file will be executable.

## semantics

***this section is WIP***

the semantics of ppop is very similar to machine languages. every 4 unicode grapheme tuple is treated as an instruction. the difference is that all instruction is executed not once but . each of them is translated as define  as the tuple.

### agent and pipe

all 

- if `OP` is a kind of `IOP`, then 

## list of operator

- uppercase ASCII alphabet: pipe
- lowercase ASCII alphabet: immediate
- ASCII non-alphabetnumeral symbol: operator

### load immediate

syntax: `Am#B` where

- `A`: `()`
- `B`: `T`

| symbol(`#`) | type(`T`) |
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

syntax: `AB#C` where

- `A`: `T`
- `B`: `T`
- `C`: `T`

### stdio

- `P` print to stdout
- `p` print to stderr

## examples

this script prints "Hello World\n", with EOL at the end:

```ppop
##~!!HSH!eSe!lSl!oSo! S !WSW!rSr!dSd!
SRHe+00l+11l+22o+33 +44W+55o+66r+77l+88d+99R+xxxP
```

with illegal comments, escape and pipe renaming, the detail of the script is explained below:

```ppop-ill
##~!  #: usize !: ()    `argc` is sent to `#` by system and is the only element of `#`, so we can utilize it as init signal by convert it into `()`
!HSH  !: () -> H:String
!eSe  !: () -> e:String
!lSl  !: () -> l:String
!oSo  !: () -> o:String
!_S_  !: () -> _:String    each pipe store the character to build `Hello world`
!WSW  !: () -> W:String
!rSr  !: () -> r:String
!dSd  !: () -> d:String
!\nSR !: () -> R:String
He+0  H: String, e: String -> 0:String
0l+1  0: String, l: String -> 1:String
1l+2  1: String, l: String -> 2:String
2o+3  2: String, o: String -> 3:String
3_+4  3: String, _: String -> 4:String
4W+5  4: String, W: String -> 5:String    concat
5o+6  5: String, o: String -> 8:String
6r+7  6: String, r: String -> 7:String
7l+8  7: String, l: String -> 8:String
8d+9  8: String, d: String -> 9:String
9R+x  9: String, R: String -> x:String
xxP\n x: String -> \n: String    print
```
