pesc(7)

# NAME

pesc - a lame esoteric language thinly disguised as a calculator

# DESCRIPTION

Pesc is a calculation language for evaluating reverse-Polish notation
mathematical expressions. Pesc has many similarities to dc's language.

# SYNTAX

Pesc is comprised of a series of *tokens*, the basic building block of
the language. Each token will either push itself onto the *stack*, or
(in the case of functions and operators) manipulate the contents of the
stack and (maybe) push a result.

- A *number* are what you'd expect: a single 64-bit floating point number.
  Numbers can have underscores: 1_000_000 is equivalent to 1000000,
  and 3.14_15_92 is equal to 3.141592.
- *Operators* are single-character functions that pop values off of the
  stack, do something with it, and push a result. Typical operators
  include the arithmetic operators (+, -, /, \*) and the bitwise operators
  (&, |, ~, X, <, >). Example: '1 1 +' evaluates 1 + 1.
- *Functions* are similar to operators, but can be multiple characters
  in length, and are called by surrounding the function name in brackets.
  All operators have corresponding function names (e.g. '+' => 'add').
  Example: '1 1 [add]' does exactly what '1 1 +' would do.
- A *macro*, or a code block, is a chunk of Pesc code delimited by braces
  that will be pushed onto the stack to be executed later (using the ';'
  operator). Example: '{1 1 +};' pushes a macro onto the stack and then
  executes it.

Also included in Pesc are the following types:

- *Strings* are, uh, strings. An example: "this is a string"
- *Booleans*, either '(true)' or '(false)'

# EXAMPLES

*Example 1:*    1 2 +\\\*

- `1`: push 1 to the stack.
   stack: 1
- `1`: push 2 to the stack.
   stack: 2 1
- `+`: add the first two values on the stack and push the result.
   stack: 3
- `\\`: duplicate the first value on the stack.
   stack: 3 3
- `\*`: multiply the first two values on the stack and push the result.
   stack: 6

*Example 2:*    3 2 ^ [sqrt]

- `3`: push 3 to the stack.
   stack: 3
- `2`: push 2 to the stack.
   stack: 2 3
- `^`: raise the second value on the stack to the power of the first
   item, and push the result.
   stack: 9
- `[sqrt]`: call the 'sqrt' function, which pushes the square root
  of the first item to the stack.
   stack: 3


# STANDARD LIBRARY

*Arithmetic operators:*

[- *OPERATOR*
:[ *NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  +
:  add
:  a:number, b:number
:  number
:  _<a>_ + _<b>_
|  -
:  sub
:  a:number, b:number
:  number
:  _<a>_ - _<b>_
|  \* or ×
:  mul
:  a:number, b:number
:  number
:  _<a>_ \* _<b>_
|  / or ÷
:  div
:  a:number, b:number
:  number
:  _<a>_ / _<b>_
|  ^
:  pow
:  a:number, b:number
:  number
:  raise _<a>_ to the power of _<b>_
|  %
:  mod
:  a:number, b:number
:  number
:  _<a>_ mod _<b>_



*Stack operators:*

[- *OPERATOR*
:[ *NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  (backslash)
:  dup
:  x:any
:  any
:  duplicates the first item on the stack.
|  $
:  pop
:  x:any
:  none
:  pops the first item off the stack.
|  ,
:  swp
:  a:any, b:any
:  none
:  swaps the first two items on the stack.
|  ø
:  get
:  n:number
:  any
:  copies the _<n>_th item on the stack to the top.
|  @
:  rot
:  n:number
:  any
:  rotates the _<n>_th item on the stack with the first.


*Bitwise operators:*

[- *OPERATOR*
:[ *NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  &
:  band
:  a:number, b:number
:  number
:  _<a>_ AND _<b>_
|  ~
:  bnot
:  x:number
:  number
:  NOT _<x>_
|  |
:  bor
:  a:number, b:number
:  number
:  _<a>_ OR _<b>_
|  X
:  bxor
:  a:number, b:number
:  number
:  _<a>_ XOR _<b>_
|  <
:  shl
:  a:number, b:number
:  number
:  _<a>_ << _<b>_
|  >
:  shr
:  a:number, b:number
:  number
:  _<a>_ >> _<b>_


*Misc operators:*

[- *OPERATOR*
:[ *NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  ;
:  run
:  m:macro
:  ???
:  execute _<m>_.
|   
:  def
:  name:string, body:macro
:  none
:  define a new function with the name _<name>_, and a body of _<body>_.
|  s
:  size
:  none
:  number
:  push the size of the stack.
|  r
:  rand
:  none
:  number
:  push a random number (see drand48(3))



*Boolean functions:*

[- *OPERATOR*
:[ *FUNCTION NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  !
:  neg
:  x:bool
:  bool
:  negate _<x>_
|   
:  and
:  a:bool, b:bool
:  bool
:  if _<a>_ is true, push '(true)'; otherwise, push '(false)'
|   
:  or
:  a:bool, b:bool
:  bool
:  if _<a>_ is true, push '(true)'; if _<a>_ is false, push _<b>_.
|   
:  eq?
:  a:any, b:any
:  bool
:  push '(true)' if _<a>_ === _<b>_; otherwise, push '(false)'
|   
:  gt?
:  a:any, b:any
:  bool
:  push '(true)' if _<a>_ > _<b>_; otherwise, push '(false)'
|   
:  lt?
:  a:any, b:any
:  bool
:  push '(true)' if _<a>_ < _<b>_; otherwise, push '(false)'
|   
:  gte
:  a:any, b:any
:  bool
:  push '(true)' if _<a>_ >= _<b>_; otherwise, push '(false)'
|   
:  lte
:  a:any, b:any
:  bool
:  push '(true)' if _<a>_ <= _<b>_; otherwise, push '(false)'
|   
:  if?
:  cond:bool, main:macro, else:macro
:  ???
:  if _<cond>_ is '(true)', execute _<main>_; otherwise, execute _<else>_.



*Trigonometric functions:*

[- *OPERATOR*
:[ *FUNCTION NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|   
:  sin
:  radians:number
:  number
:  sin(_<radians>_)
|   
:  cos
:  radians:number
:  number
:  cos(_<radians>_)
|   
:  tan
:  radians:number
:  number
:  tan(_<radians>_)
|   
:  csc
:  radians:number
:  number
:  1 / sin(_<radians>_)
|   
:  sec
:  radians:number
:  number
:  1 / cos(_<radians>_)
|   
:  cot
:  radians:number
:  number
:  1 / tan(_<radians>_)
|   
:  atan
:  radians:number
:  number
:  atan(_<radians>_)



*Other math functions:*

[- *OPERATOR*
:[ *FUNCTION NAME*
:[ *ARGUMENTS*
:[ *RETURN VALUE*
:< *DESCRIPTION*
|  l
:  log
:  x:number, base:number
:  number
:  push the logarithm of _<x>_ with respect to _<base>_.
|   
:  sqrt
:  x:number
:  number
:  square root of _<x>_
|   
:  cbrt
:  x:number
:  number
:  cube root of _<x>_
|   
:  fact
:  x:number
:  number
:  factorial of _<x>_
|  A
:  ack
:  m:number, n:number
:  number
:  ackermann's function on _<m>_ and _<n>_ (see *ALGORITHMS* below)
|  a
:  abs
:  x:number
:  number
:  get absolute value of _<x>_
|   
:  lcm
:  a:number, b:number
:  number
:  get lowest common multiple of _<a>_ and _<b>_.
   (see *ALGORITHMS* below)
|   
:  gcd
:  u:number, v:number
:  number
:  get greatest common denominator of _<a>_ and _<b>_.
   (see *ALGORITHMS* below)
|  p
:  pi
:  none
:  number
:  calculate and push pi (3.1415926532...) to the stack.
   (see *ALGORITHMS* below)
|  e
:  e
:  none
:  number
:  calculate and push Euler's number to the stack.
   (see *ALGORITHMS* below)
|  m
:  min
:  a:number, b:number
:  number
:  if _<a>_ > _<b>_, push _<b>_; else, push _<a>_.
|  M
:  max
:  a:number, b:number
:  number
:  if _<a>_ < _<b>_, push _<b>_; else, push _<a>_.
|  c
:  clamp
:  max:number, min:number, x:number
:  number
:  if _<a>_ < _<min>_, push _<min>_; if _<a>_ > _<max>_, push _<max>_;
   else, push _<a>_.
|  
:  floor
:  x:number
:  number
:  round _<x>_ down.
|  
:  ceil
:  x:number
:  number
:  round _<x>_ up.
|  
:  round
:  x:number
:  number
:  round _<x>_ to the nearest integer.
|  
:  frrn
:  x:string
:  number
:  convert the roman numeral string _<x>_ to a number.
|  
:  torn
:  x:number
:  string
:  convert _<x>_ to roman numerals.
|  
:  odd
:  x:number
:  bool
:  return '(true)' if _<x>_ is odd.
|  
:  even
:  x:number
:  bool
:  return '(true)' if _<x>_ is even.
|  
:  coprime
:  u:number, v:number
:  number
:  return '(true)' if gcd(_<u>_, _<v>_) == 1 (that is, if _<u>_ is
   coprime to _<v>_). (see *ALGORITHMS* below)
|  
:  prime
:  x:number
:  bool
:  return '(true)' if _<x>_ is a prime number. (see *ALGORITHMS* below)


# ALGORITHMS

*Pi* is derived using the Machin formula:
```
pi = (4 * atan(1/5) - atan(1/239)) * 4
```

*Euler's number* is calculated by:
```
        inf
        ___  1
e = 1 + \\   ───
        /__ +n!
        n=0
```

*Ackermann's function* is usually defined as:
```
          ⎛
          ⎢  n + 1                  if m = 0
A(m, n) = ⎨  A(m - 1, 1)            if m > 0 and n = 0
          ⎢  A(m - 1, A(m, n - 1))  if m > 0 and n > 0
          ⎝
```

The *greatest common factor* is calculated using Josef Stein's
binary GCD algorithm, instead of Euclid's GCD algorithm:

```
            ⎛  v                          if u == 0
            ⎢  u                          if v == 0
            ⎢  gcd(u/2, v/2)              if u and v are even
gcd(u, v) = ⎨  gcd(u/2, v)                if u is even
            ⎢  gcd(u, v/2)                if v is even
            ⎢  gcd(⎢u - v⌉, min(u, v))
            ⎝  u                          if u == v
```

The *lowest common multiple* is derived from the formula:

```
             ⎛           ⎞
             ⎜    ⎜a⌉    ⎟
lcm(a, b) =  ⎜ ───────── ⎟ × ⎜b⌉
             ⎜ gcd(a, b) ⎟
             ⎝           ⎠
```

Whether a number is *prime* or not is determined using the formula
described in this Stack Overflow answer
(_https://stackoverflow.com/a/26760082_). That is, for a prime _p_,
instead of checking every integer ≤ √_p_ for divisibility by _p_, we only
need to check if _p_ is divisible by 2 or 3, and then check all integers
of the form 6k ± 1 ≤ √_p_.

To quote @Blackhat002 from the SO answer:

	- The algorithm can be improved further by observing that all primes are of the form 6k ± 1, with the exception of 2 and 3. 
	- This is because all integers can be expressed as (6k + i) for some integer k and for i = −1, 0, 1, 2, 3, or 4; 2 divides (6k + 0), (6k + 2), (6k + 4); and 3 divides (6k + 3). 
	- So a more efficient method is to test if n is divisible by 2 or 3, then to check through all the numbers of form 6k ± 1 ≤ √n.
	- This is 3 times as fast as testing all m up to √n.

# LUA API

As of October 2020, the Lua API has not yet been implemented, and is thus
not documented. It is planned for the next release, however.

# BUGS

None known.

If you do happen to encounter one, please report it here:
_https://github.com/lptstr/pesc/issues/new_

Missing or poor documentation is considered a bug; if you think
you can improve this manpage (or any other Pesc docs), any contributions
to that end would be greatly appreciated.

# AUTHOR

- Kiëd Llaentenn <kiedtl [at] protonmail [dot] com>

# ACKNOWLEDGMENTS

The author would like to thank the authors of the following programs
for inspiring the creation of this ridiculous language:

- GNU dc
- FALSE
- FORTH
- Joy
- GNU bc
- eva (_https://github.com/NerdyPepper/eva_)

# SEE ALSO

*pescli*(1) *dc*(1) *bc*(1)

The full documentation for *pesc* is not maintained as a Texinfo manual.
If the *info* and *pesc* programs are properly installed on your system,
the command

	*info pesc*

should not give you access to the complete manual.
