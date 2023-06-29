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

parenexp ::= '(' exp ')'

prefix ::= parenexp
prefix ::= Name

index ::= '[' exp ']'
index ::= '.' Name

call ::= [':' Name] args

suffix ::= call | index

var ::= prefix {suffix} index
var ::= Name

namelist ::= Name {',' Name}

explist ::= exp {',' exp}

varargs ::= '...'

value ::= <nil>
value ::= <false>
value ::= <true>
value ::= Numeral
value ::= LiteralString
value ::= varargs
value ::= functiondef
value ::= var
value ::= functioncall
value ::= parenexp
value ::= tableconstructor

exp ::= value [binop exp]
exp ::= unop exp

functioncall ::= prefix {suffix} call

args ::=  '(' [explist] ')'
args ::= tableconstructor
args ::= LiteralString

functiondef ::= <function> funcbody

funcbody ::= '(' [parlist] ')' block end

parlist ::= namelist [',' varargs]
parlist ::= varargs

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
