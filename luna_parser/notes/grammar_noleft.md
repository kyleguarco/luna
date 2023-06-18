# Lua BNF Grammar

This grammar has the left-recursion removed.
Also see http://lua-users.org/lists/lua-l/2010-12/msg00699.html

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

braceexp ::= '(' exp ')'

index ::= '[' exp ']' | '.' Name

var ::= Name
var ::= var index
var ::= functioncall index
var ::= braceexp index

namelist ::= Name {',' Name}

explist ::= exp {',' exp}

exp ::= <nil>
exp ::= <false>
exp ::= <true>
exp ::= Numeral
exp ::= LiteralString
exp ::= '...'
exp ::= functiondef
exp ::= (var | functioncall | braceexp)
exp ::= tableconstructor
exp ::= exp binop exp
exp ::= unop exp

<!-- prefixexp ::= var | functioncall | braceexp -->

calltype ::= [':' Name] args

functioncall ::= var calltype
functioncall ::= functioncall calltype
functioncall ::= braceexp calltype

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
