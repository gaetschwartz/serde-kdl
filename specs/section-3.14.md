### 3.14. Number

Numbers in KDL represent numerical Values (Section 3.7). There is no logical distinction in KDL
between real numbers, integers, and floating point numbers. It's up to
individual implementations to determine how to represent KDL numbers.

There are five syntaxes for Numbers: Keywords, Decimal, Hexadecimal, Octal, and Binary.

- All non-Keyword (Section 3.14.1) numbers may optionally start with one of `-` or `+`, which determine whether they'll be positive or negative.

- Binary numbers start with `0b` and only allow `0` and `1` as digits, which may be separated by `_`. They represent numbers in radix 2.

- Octal numbers start with `0o` and only allow digits between `0` and `7`, which may be separated by `_`. They represent numbers in radix 8.

- Hexadecimal numbers start with `0x` and allow digits between `0` and `9`, as well as letters `A` through `F`, in either lower or upper case, which may be separated by `_`. They represent numbers in radix 16.

- Decimal numbers are a bit more special:
  - They have no radix prefix.
  - They use digits `0` through `9`, which may be separated by `_`.
  - They may optionally include a decimal separator `.`, followed by more digits, which may again be separated by `_`.
  - They may optionally be followed by `E` or `e`, an optional `-` or `+`, and more digits, to represent an exponent value.

Note that, similar to JSON and some other languages,
numbers without an integer digit (such as `.1`) are illegal.
They must be written with at least one integer digit, like `0.1`.
(These patterns are also disallowed from Identifier Strings (Section 3.10), to avoid confusion.)

#### 3.14.1. Keyword Numbers

There are three special "keyword" numbers included in KDL to accomodate the
widespread use of [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floats:

- `#inf` - floating point positive infinity.
- `#-inf` - floating point negative infinity.
- `#nan` - floating point NaN/Not a Number.

To go along with this and prevent foot guns, the bare Identifier
Strings (Section 3.10) `inf`, `-inf`, and `nan` are considered illegal
identifiers and should yield a syntax error.

The existence of these keywords does not imply that any numbers be represented
as IEEE 754 floats. These are simply for clarity and convenience for any
implementation that chooses to represent their numbers in this way.