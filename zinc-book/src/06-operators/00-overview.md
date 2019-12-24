# Operators

## Path resolution

`::`

**Accepts**
1. Module or enum
2. Identifier or module

**Returns** the second operand.

## Array indexing

`[]`

**Accepts**
1. Array expression
2. Integer literal

**Returns** an array element or error.

**Note**: only integer literals can be array indexes now.

## Field access

`.`

**Accepts**
1. Tuple expression
2. Integer literal

**Returns** a tuple or structure element or error.

## Unary minus

`-`

**Accepts**
1. Integer expression

**Returns** the integer result.

## Logical NOT

`!`

**Accepts**
1. Boolean expression

**Returns** the boolean result.

## Casting

`as`

**Accepts**
1. Integer expression
2. Integer type

**Returns** the integer result.

Casting allowed:

- integers to types of greater bitlength
- enums to integers of enough bitlength

```rust,no_run,noplaypen
let a = -1; // inference
let b: u16 = a as u16; // ok, casted to the opposite sign with greater bitlength 
let c: u8 = Order::First; // casting to an integer of enough bitlength
```

## Multiplication

`*`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Division

`/`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Remainder

`%`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Addition

`+`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Subtraction

`-`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Equality

`==`

**Accepts**
1. Integer **or** boolean expression
2. Integer **or** boolean expression

**Returns** the boolean result.

## Non-equality

`!=`

**Accepts**
1. Integer **or** boolean expression
2. Integer **or** boolean expression

**Returns** the boolean result.

## Lesser or equals

`<=`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

## Greater or equals

`>=`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

## Lesser

`<`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

## Greater

`>`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

## Logical AND

`&&`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

## Logical XOR

`^^`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

## Logical OR

`||`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

## Exclusive range (TODO)

`..`

## Inclusive range (TODO)

`..=`

## Assignment

`=`

**Accepts**
1. Place expression
2. Value expression

**Returns** `()`.