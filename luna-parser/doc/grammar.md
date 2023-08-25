# Lua BNF Grammar

Original grammar as seen in https://www.lua.org/manual/5.4/manual.html#9

Notation:
* `{}` zero or more of
* `[]` optional
* `<>` keywords
* `''` terminals

```
chunk ::= block

block ::= {stat} [retstat]

stat ::= ';'
stat ::= varlist '=' explist
stat ::= functioncall
stat ::= label
stat ::= <break>
stat ::= <goto> Name
stat ::= <do> block <end>
stat ::= <while> exp <do> block <end>
stat ::= <repeat> block <until> exp
stat ::= <if> exp <then> block {<elseif> exp <then> block} [<else> block] <end>
stat ::= <for> Name '=' exp ',' exp [',' exp] <do> block <end>
stat ::= <for> namelist <in> explist <do> block <end>
stat ::= <function> funcname funcbody
stat ::= <local> <function> Name funcbody
stat ::= <local> attnamelist ['=' explist]

attnamelist ::=  Name attrib {',' Name attrib}

attrib ::= ['<' Name '>']

retstat ::= <return> [explist] [';']

label ::= '::' Name '::'

funcname ::= Name {'.' Name} [':' Name]

varlist ::= var {',' var}

var ::= Name
var ::= prefixexp '[' exp ']'
var ::= prefixexp '.' Name

namelist ::= Name {',' Name}

explist ::= exp {',' exp}

exp ::= <nil>
exp ::= <false>
exp ::= <true>
exp ::= Numeral
exp ::= LiteralString
exp ::= '...'
exp ::= functiondef
exp ::= prefixexp
exp ::= tableconstructor
exp ::= exp binop exp
exp ::= unop exp

prefixexp ::= var
prefixexp ::= functioncall
prefixexp ::= '(' exp ')'

functioncall ::= prefixexp args
functioncall ::= prefixexp ':' Name args

args ::=  '(' [explist] ')'
args ::= tableconstructor
args ::= LiteralString

functiondef ::= <function> funcbody

funcbody ::= '(' [parlist] ')' block end

parlist ::= namelist [',' '...']
parlist ::= '...'

tableconstructor ::= '{' [fieldlist] '}'

fieldlist ::= field {fieldsep field} [fieldsep]

field ::= '[' exp ']' '=' exp
field ::= Name '=' exp
field ::= exp

fieldsep ::= ',' | ';'

binop ::=  '+' | '-' | '*' | '/' | '//' | '^' | '%' |
		'&' | '~' | '|' | '>>' | '<<' | '..' |
		'<' | '<=' | '>' | '>=' | '==' | '~=' |
		<and> | <or>

unop ::= '-' | <not> | '#' | '~'
```
