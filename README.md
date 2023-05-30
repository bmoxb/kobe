# Kobe

## Features

* Simple, Lua-inspired syntax.
* Static typing.
* Compiles to WASM.

## Grammar

Note that this grammar does not describe operator precedence.

```
program ::= {function}

function ::= 'fn' '(' [params] ')' ['->' ident] '\n' {stat} 'end'
params ::= param {',' param}
param ::= ident ':' ident

stat ::= '\n'
       | expr '\n'
       | 'let' ident ':' ident ['=' expr] '\n'
       | ident '=' expr '\n'
       | ident '+=' expr '\n'
       | ident '-=' expr '\n'
       | ident '*=' expr '\n'
       | ident '/=' expr '\n'
       | 'return' expr '\n'
       | 'if' expr 'then' {stat} ['else' {stat}] 'end'
       | 'while' expr 'do' {stat} 'end'
       | 'for' ident 'in' expr 'do' {stat} 'end'

expr ::= int | float | char | string
       | '[' [exprs] ']'
       | '(' expr ')'
       | expr binop expr
       | unop expr

exprs ::= expr {',' expr}

binop ::= '==' | '!=' | 'and' | 'or' | '<' | '>' | '<=' '>=' | '+' | '-' | '*' | '/'

unop ::= '-' | 'not'
```
