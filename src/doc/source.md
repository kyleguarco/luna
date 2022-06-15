# Luna

`luna` is a rust library intended solely for learning about how compilers and interpreters work. This isn't some new language, but is instead an implementation of the lua programming language written in rust.

---
# Lua 5.4.4

This project aims to re-create all the functionality that Lua 5.4.4 provides. Below are the functions provided by all of the header files present in the current library.

*This documentation is based on this [lua-users](http://lua-users.org/wiki/LuaSource) article*

<details open>
<summary><b>Sections</b> (Click to expand)</summary>

 - [Utilities](#utilities)
 - [Data Types](#basic-data-types)
 - [Parsing and CodeGen](#parsing-and-code-generation)
 - [The LVM](#bytecode-execution-the-lvm)

</details>

## Utilities
### Debug Interface (ldebug.h)
```c
#include "lstate.h"

int luaG_getfuncline (const Proto *f, int pc);
const char *luaG_findlocal (lua_State *L, CallInfo *ci, int n, StkId *pos);
l_noret luaG_typeerror (lua_State *L, const TValue *o, const char *opname);
l_noret luaG_callerror (lua_State *L, const TValue *o);
l_noret luaG_forerror (lua_State *L, const TValue *o, const char *what);
l_noret luaG_concaterror (lua_State *L, const TValue *p1, const TValue *p2);
l_noret luaG_opinterror (lua_State *L, const TValue *p1, const TValue *p2, const char *msg);
l_noret luaG_tointerror (lua_State *L, const TValue *p1, const TValue *p2);
l_noret luaG_ordererror (lua_State *L, const TValue *p1, const TValue *p2);
l_noret luaG_runerror (lua_State *L, const char *fmt, ...);
const char *luaG_addinfo (lua_State *L, const char *msg, TString *src, int line);
l_noret luaG_errormsg (lua_State *L);
int luaG_traceexec (lua_State *L, const Instruction *pc);
```

### Buffered Input Streams (lzio.h)
```c
#include "lua.h"
#include "lmem.h"

typedef struct Mbuffer {
  char *buffer;
  size_t n;
  size_t buffsize;
} Mbuffer;

struct Zio {
  size_t n;		/* bytes still unread */
  const char *p;	/* current position in buffer */
  lua_Reader reader;	/* reader function */
  void *data;		/* additional data */
  lua_State *L;		/* Lua state (for reader) */
};

typedef struct Zio ZIO;

void luaZ_init (lua_State *L, ZIO *z, lua_Reader reader, void *data);
size_t luaZ_read (ZIO* z, void *b, size_t n); /* read next n bytes */
int luaZ_fill (ZIO *z);
```

### Memory Manager Interface (lmem.h)
```c
#include <stddef.h>

#include "llimits.h"
#include "lua.h"

l_noret luaM_toobig (lua_State *L);
void *luaM_realloc_ (lua_State *L, void *block, size_t oldsize, size_t size);
void *luaM_saferealloc_ (lua_State *L, void *block, size_t oldsize, size_t size);
void luaM_free_ (lua_State *L, void *block, size_t osize);
void *luaM_growaux_ (lua_State *L, void *block, int nelems, int *size, int size_elem, int limit, const char *what);
void *luaM_shrinkvector_ (lua_State *L, void *block, int *nelem, int final_n, int size_elem);
void *luaM_malloc_ (lua_State *L, size_t size, int tag);
```

### Incremental Garbage Collector (lgc.h)
```c
#include "lobject.h"
#include "lstate.h"

void luaC_fix (lua_State *L, GCObject *o);
void luaC_freeallobjects (lua_State *L);
void luaC_step (lua_State *L);
void luaC_runtilstate (lua_State *L, int statesmask);
void luaC_fullgc (lua_State *L, int isemergency);
GCObject *luaC_newobj (lua_State *L, int tt, size_t sz);
void luaC_barrier_ (lua_State *L, GCObject *o, GCObject *v);
void luaC_barrierback_ (lua_State *L, GCObject *o);
void luaC_checkfinalizer (lua_State *L, GCObject *o, Table *mt);
void luaC_changemode (lua_State *L, int newmode);
```

## Basic Data Types
### Lua Objects (lobject.h)
```c
#include <stdarg.h>

#include "llimits.h"
#include "lua.h"

typedef union Value {
  struct GCObject *gc;    /* collectable objects */
  void *p;         /* light userdata */
  lua_CFunction f; /* light C functions */
  lua_Integer i;   /* integer numbers */
  lua_Number n;    /* float numbers */
} Value;

typedef struct TValue {
  TValuefields;
} TValue;

typedef union StackValue {
  TValue val;
  struct {
    TValuefields;
    unsigned short delta;
  } tbclist;
} StackValue;

/* index to stack elements */
typedef StackValue *StkId;

/*
** Common Header for all collectable objects (in macro form, to be
** included in other objects)
*/
#define CommonHeader	struct GCObject *next; lu_byte tt; lu_byte marked


/* Common type for all collectable objects */
typedef struct GCObject {
  CommonHeader;
} GCObject;

/*
** Header for a string value.
*/
typedef struct TString {
  CommonHeader;
  lu_byte extra;  /* reserved words for short strings; "has hash" for longs */
  lu_byte shrlen;  /* length for short strings */
  unsigned int hash;
  union {
    size_t lnglen;  /* length for long strings */
    struct TString *hnext;  /* linked list for hash table */
  } u;
  char contents[1];
} TString;

/* Ensures that addresses after this type are always fully aligned. */
typedef union UValue {
  TValue uv;
  LUAI_MAXALIGN;  /* ensures maximum alignment for udata bytes */
} UValue;


/*
** Header for userdata with user values;
** memory area follows the end of this structure.
*/
typedef struct Udata {
  CommonHeader;
  unsigned short nuvalue;  /* number of user values */
  size_t len;  /* number of bytes */
  struct Table *metatable;
  GCObject *gclist;
  UValue uv[1];  /* user values */
} Udata;

/*
** Header for userdata with no user values. These userdata do not need
** to be gray during GC, and therefore do not need a 'gclist' field.
** To simplify, the code always use 'Udata' for both kinds of userdata,
** making sure it never accesses 'gclist' on userdata with no user values.
** This structure here is used only to compute the correct size for
** this representation. (The 'bindata' field in its end ensures correct
** alignment for binary data following this header.)
*/
typedef struct Udata0 {
  CommonHeader;
  unsigned short nuvalue;  /* number of user values */
  size_t len;  /* number of bytes */
  struct Table *metatable;
  union {LUAI_MAXALIGN;} bindata;
} Udata0;

/*
** Description of an upvalue for function prototypes
*/
typedef struct Upvaldesc {
  TString *name;  /* upvalue name (for debug information) */
  lu_byte instack;  /* whether it is in stack (register) */
  lu_byte idx;  /* index of upvalue (in stack or in outer function's list) */
  lu_byte kind;  /* kind of corresponding variable */
} Upvaldesc;

/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
typedef struct LocVar {
  TString *varname;
  int startpc;  /* first point where variable is active */
  int endpc;    /* first point where variable is dead */
} LocVar;

/*
** Associates the absolute line source for a given instruction ('pc').
** The array 'lineinfo' gives, for each instruction, the difference in
** lines from the previous instruction. When that difference does not
** fit into a byte, Lua saves the absolute line for that instruction.
** (Lua also saves the absolute line periodically, to speed up the
** computation of a line number: we can use binary search in the
** absolute-line array, but we must traverse the 'lineinfo' array
** linearly to compute a line.)
*/
typedef struct AbsLineInfo {
  int pc;
  int line;
} AbsLineInfo;

/*
** Function Prototypes
*/
typedef struct Proto {
  CommonHeader;
  lu_byte numparams;  /* number of fixed (named) parameters */
  lu_byte is_vararg;
  lu_byte maxstacksize;  /* number of registers needed by this function */
  int sizeupvalues;  /* size of 'upvalues' */
  int sizek;  /* size of 'k' */
  int sizecode;
  int sizelineinfo;
  int sizep;  /* size of 'p' */
  int sizelocvars;
  int sizeabslineinfo;  /* size of 'abslineinfo' */
  int linedefined;  /* debug information  */
  int lastlinedefined;  /* debug information  */
  TValue *k;  /* constants used by the function */
  Instruction *code;  /* opcodes */
  struct Proto **p;  /* functions defined inside the function */
  Upvaldesc *upvalues;  /* upvalue information */
  ls_byte *lineinfo;  /* information about source lines (debug information) */
  AbsLineInfo *abslineinfo;  /* idem */
  LocVar *locvars;  /* information about local variables (debug information) */
  TString  *source;  /* used for debug information */
  GCObject *gclist;
} Proto;

/*
** Upvalues for Lua closures
*/
typedef struct UpVal {
  CommonHeader;
  lu_byte tbc;  /* true if it represents a to-be-closed variable */
  TValue *v;  /* points to stack or to its own value */
  union {
    struct {  /* (when open) */
      struct UpVal *next;  /* linked list */
      struct UpVal **previous;
    } open;
    TValue value;  /* the value (when closed) */
  } u;
} UpVal;

#define ClosureHeader CommonHeader; lu_byte nupvalues; GCObject *gclist

typedef struct CClosure {
  ClosureHeader;
  lua_CFunction f;
  TValue upvalue[1];  /* list of upvalues */
} CClosure;


typedef struct LClosure {
  ClosureHeader;
  struct Proto *p;
  UpVal *upvals[1];  /* list of upvalues */
} LClosure;


typedef union Closure {
  CClosure c;
  LClosure l;
} Closure;

/*
** Nodes for Hash tables: A pack of two TValue's (key-value pairs)
** plus a 'next' field to link colliding entries. The distribution
** of the key's fields ('key_tt' and 'key_val') not forming a proper
** 'TValue' allows for a smaller size for 'Node' both in 4-byte
** and 8-byte alignments.
*/
typedef union Node {
  struct NodeKey {
    TValuefields;  /* fields for value */
    lu_byte key_tt;  /* key type */
    int next;  /* for chaining */
    Value key_val;  /* key value */
  } u;
  TValue i_val;  /* direct access to node's value as a proper 'TValue' */
} Node;

typedef struct Table {
  CommonHeader;
  lu_byte flags;  /* 1<<p means tagmethod(p) is not present */
  lu_byte lsizenode;  /* log2 of size of 'node' array */
  unsigned int alimit;  /* "limit" of 'array' array */
  TValue *array;  /* array part */
  Node *node;
  Node *lastfree;  /* any free position is before this position */
  struct Table *metatable;
  GCObject *gclist;
} Table;

int luaO_utf8esc (char *buff, unsigned long x);
int luaO_ceillog2 (unsigned int x);
int luaO_rawarith (lua_State *L, int op, const TValue *p1, const TValue *p2, TValue *res);
void luaO_arith (lua_State *L, int op, const TValue *p1, const TValue *p2, StkId res);
size_t luaO_str2num (const char *s, TValue *o);
int luaO_hexavalue (int c);
void luaO_tostring (lua_State *L, TValue *obj);
const char *luaO_pushvfstring (lua_State *L, const char *fmt, va_list argp);
const char *luaO_pushfstring (lua_State *L, const char *fmt, ...);
void luaO_chunkid (char *out, const char *source, size_t srclen);
```

### Global and Local State Objects (lstate.h)
```c
#include "lua.h"

#include "lobject.h"
#include "ltm.h"
#include "lzio.h"

typedef struct stringtable {
  TString **hash;
  int nuse;  /* number of elements */
  int size;
} stringtable;

typedef struct CallInfo {
  StkId func;  /* function index in the stack */
  StkId	top;  /* top for this function */
  struct CallInfo *previous, *next;  /* dynamic call link */
  union {
    struct {  /* only for Lua functions */
      const Instruction *savedpc;
      volatile l_signalT trap;
      int nextraargs;  /* # of extra arguments in vararg functions */
    } l;
    struct {  /* only for C functions */
      lua_KFunction k;  /* continuation in case of yields */
      ptrdiff_t old_errfunc;
      lua_KContext ctx;  /* context info. in case of yields */
    } c;
  } u;
  union {
    int funcidx;  /* called-function index */
    int nyield;  /* number of values yielded */
    int nres;  /* number of values returned */
    struct {  /* info about transferred values (for call/return hooks) */
      unsigned short ftransfer;  /* offset of first value transferred */
      unsigned short ntransfer;  /* number of values transferred */
    } transferinfo;
  } u2;
  short nresults;  /* expected number of results from this function */
  unsigned short callstatus;
} CallInfo;

typedef struct global_State {
  lua_Alloc frealloc;  /* function to reallocate memory */
  void *ud;         /* auxiliary data to 'frealloc' */
  l_mem totalbytes;  /* number of bytes currently allocated - GCdebt */
  l_mem GCdebt;  /* bytes allocated not yet compensated by the collector */
  lu_mem GCestimate;  /* an estimate of the non-garbage memory in use */
  lu_mem lastatomic;  /* see function 'genstep' in file 'lgc.c' */
  stringtable strt;  /* hash table for strings */
  TValue l_registry;
  TValue nilvalue;  /* a nil value */
  unsigned int seed;  /* randomized seed for hashes */
  lu_byte currentwhite;
  lu_byte gcstate;  /* state of garbage collector */
  lu_byte gckind;  /* kind of GC running */
  lu_byte gcstopem;  /* stops emergency collections */
  lu_byte genminormul;  /* control for minor generational collections */
  lu_byte genmajormul;  /* control for major generational collections */
  lu_byte gcstp;  /* control whether GC is running */
  lu_byte gcemergency;  /* true if this is an emergency collection */
  lu_byte gcpause;  /* size of pause between successive GCs */
  lu_byte gcstepmul;  /* GC "speed" */
  lu_byte gcstepsize;  /* (log2 of) GC granularity */
  GCObject *allgc;  /* list of all collectable objects */
  GCObject **sweepgc;  /* current position of sweep in list */
  GCObject *finobj;  /* list of collectable objects with finalizers */
  GCObject *gray;  /* list of gray objects */
  GCObject *grayagain;  /* list of objects to be traversed atomically */
  GCObject *weak;  /* list of tables with weak values */
  GCObject *ephemeron;  /* list of ephemeron tables (weak keys) */
  GCObject *allweak;  /* list of all-weak tables */
  GCObject *tobefnz;  /* list of userdata to be GC */
  GCObject *fixedgc;  /* list of objects not to be collected */
  /* fields for generational collector */
  GCObject *survival;  /* start of objects that survived one GC cycle */
  GCObject *old1;  /* start of old1 objects */
  GCObject *reallyold;  /* objects more than one cycle old ("really old") */
  GCObject *firstold1;  /* first OLD1 object in the list (if any) */
  GCObject *finobjsur;  /* list of survival objects with finalizers */
  GCObject *finobjold1;  /* list of old1 objects with finalizers */
  GCObject *finobjrold;  /* list of really old objects with finalizers */
  struct lua_State *twups;  /* list of threads with open upvalues */
  lua_CFunction panic;  /* to be called in unprotected errors */
  struct lua_State *mainthread;
  TString *memerrmsg;  /* message for memory-allocation errors */
  TString *tmname[TM_N];  /* array with tag-method names */
  struct Table *mt[LUA_NUMTAGS];  /* metatables for basic types */
  TString *strcache[STRCACHE_N][STRCACHE_M];  /* cache for strings in API */
  lua_WarnFunction warnf;  /* warning function */
  void *ud_warn;         /* auxiliary data to 'warnf' */
} global_State;

struct lua_State {
  CommonHeader;
  lu_byte status;
  lu_byte allowhook;
  unsigned short nci;  /* number of items in 'ci' list */
  StkId top;  /* first free slot in the stack */
  global_State *l_G;
  CallInfo *ci;  /* call info for current function */
  StkId stack_last;  /* end of stack (last element + 1) */
  StkId stack;  /* stack base */
  UpVal *openupval;  /* list of open upvalues in this stack */
  StkId tbclist;  /* list of to-be-closed variables */
  GCObject *gclist;
  struct lua_State *twups;  /* list of threads with open upvalues */
  struct lua_longjmp *errorJmp;  /* current error recover point */
  CallInfo base_ci;  /* CallInfo for first level (C calling Lua) */
  volatile lua_Hook hook;
  ptrdiff_t errfunc;  /* current error handling function (stack index) */
  l_uint32 nCcalls;  /* number of nested (non-yieldable | C)  calls */
  int oldpc;  /* last pc traced */
  int basehookcount;
  int hookcount;
  volatile l_signalT hookmask;
};

union GCUnion {
  GCObject gc;  /* common header */
  struct TString ts;
  struct Udata u;
  union Closure cl;
  struct Table h;
  struct Proto p;
  struct lua_State th;  /* thread */
  struct UpVal upv;
};

void luaE_setdebt (global_State *g, l_mem debt);
void luaE_freethread (lua_State *L, lua_State *L1);
CallInfo *luaE_extendCI (lua_State *L);
void luaE_freeCI (lua_State *L);
void luaE_shrinkCI (lua_State *L);
void luaE_checkcstack (lua_State *L);
void luaE_incCstack (lua_State *L);
void luaE_warning (lua_State *L, const char *msg, int tocont);
void luaE_warnerror (lua_State *L, const char *where);
int luaE_resetthread (lua_State *L, int status);
```

### Strings (lstring.h)
```c
#include "lgc.h"
#include "lobject.h"
#include "lstate.h"

unsigned int luaS_hash (const char *str, size_t l, unsigned int seed);
unsigned int luaS_hashlongstr (TString *ts);
int luaS_eqlngstr (TString *a, TString *b);
void luaS_resize (lua_State *L, int newsize);
void luaS_clearcache (global_State *g);
void luaS_init (lua_State *L);
void luaS_remove (lua_State *L, TString *ts);
Udata *luaS_newudata (lua_State *L, size_t s, int nuvalue);
TString *luaS_newlstr (lua_State *L, const char *str, size_t l);
TString *luaS_new (lua_State *L, const char *str);
TString *luaS_createlngstrobj (lua_State *L, size_t l);
```

### Prototype and Closure Utilities (lfunc.h)
```c
#include "lobject.h"

Proto *luaF_newproto (lua_State *L);
CClosure *luaF_newCclosure (lua_State *L, int nupvals);
LClosure *luaF_newLclosure (lua_State *L, int nupvals);
void luaF_initupvals (lua_State *L, LClosure *cl);
UpVal *luaF_findupval (lua_State *L, StkId level);
void luaF_newtbcupval (lua_State *L, StkId level);
void luaF_closeupval (lua_State *L, StkId level);
void luaF_close (lua_State *L, StkId level, int status, int yy);
void luaF_unlinkupval (UpVal *uv);
void luaF_freeproto (lua_State *L, Proto *f);
const char *luaF_getlocalname (const Proto *func, int local_number, int pc);
```

### Tables (ltable.h)
```c
#include "lobject.h"

const TValue *luaH_getint (Table *t, lua_Integer key);
void luaH_setint (lua_State *L, Table *t, lua_Integer key, TValue *value);
const TValue *luaH_getshortstr (Table *t, TString *key);
const TValue *luaH_getstr (Table *t, TString *key);
const TValue *luaH_get (Table *t, const TValue *key);
void luaH_newkey (lua_State *L, Table *t, const TValue *key, TValue *value);
void luaH_set (lua_State *L, Table *t, const TValue *key, TValue *value);
void luaH_finishset (lua_State *L, Table *t, const TValue *key, const TValue *slot, TValue *value);
Table *luaH_new (lua_State *L);
void luaH_resize (lua_State *L, Table *t, unsigned int nasize, unsigned int nhsize);
void luaH_resizearray (lua_State *L, Table *t, unsigned int nasize);
void luaH_free (lua_State *L, Table *t);
int luaH_next (lua_State *L, Table *t, StkId key);
lua_Unsigned luaH_getn (Table *t);
unsigned int luaH_realasize (const Table *t);

// If debug is enabled...
Node *luaH_mainposition (const Table *t, const TValue *key);
int luaH_isdummy (const Table *t);
```

## Parsing and Code Generation
### Code Generator (lcode.h)
```c
#include "llex.h"
#include "lobject.h"
#include "lopcodes.h"
#include "lparser.h"

typedef enum BinOpr {
  /* arithmetic operators */
  OPR_ADD, OPR_SUB, OPR_MUL, OPR_MOD, OPR_POW,
  OPR_DIV, OPR_IDIV,
  /* bitwise operators */
  OPR_BAND, OPR_BOR, OPR_BXOR,
  OPR_SHL, OPR_SHR,
  /* string operator */
  OPR_CONCAT,
  /* comparison operators */
  OPR_EQ, OPR_LT, OPR_LE,
  OPR_NE, OPR_GT, OPR_GE,
  /* logical operators */
  OPR_AND, OPR_OR,
  OPR_NOBINOPR
} BinOpr;

typedef enum UnOpr { OPR_MINUS, OPR_BNOT, OPR_NOT, OPR_LEN, OPR_NOUNOPR } UnOpr;

int luaK_code (FuncState *fs, Instruction i);
int luaK_codeABx (FuncState *fs, OpCode o, int A, unsigned int Bx);
int luaK_codeAsBx (FuncState *fs, OpCode o, int A, int Bx);
int luaK_codeABCk (FuncState *fs, OpCode o, int A, int B, int C, int k);
int luaK_isKint (expdesc *e);
int luaK_exp2const (FuncState *fs, const expdesc *e, TValue *v);
void luaK_fixline (FuncState *fs, int line);
void luaK_nil (FuncState *fs, int from, int n);
void luaK_reserveregs (FuncState *fs, int n);
void luaK_checkstack (FuncState *fs, int n);
void luaK_int (FuncState *fs, int reg, lua_Integer n);
void luaK_dischargevars (FuncState *fs, expdesc *e);
int luaK_exp2anyreg (FuncState *fs, expdesc *e);
void luaK_exp2anyregup (FuncState *fs, expdesc *e);
void luaK_exp2nextreg (FuncState *fs, expdesc *e);
void luaK_exp2val (FuncState *fs, expdesc *e);
int luaK_exp2RK (FuncState *fs, expdesc *e);
void luaK_self (FuncState *fs, expdesc *e, expdesc *key);
void luaK_indexed (FuncState *fs, expdesc *t, expdesc *k);
void luaK_goiftrue (FuncState *fs, expdesc *e);
void luaK_goiffalse (FuncState *fs, expdesc *e);
void luaK_storevar (FuncState *fs, expdesc *var, expdesc *e);
void luaK_setreturns (FuncState *fs, expdesc *e, int nresults);
void luaK_setoneret (FuncState *fs, expdesc *e);
int luaK_jump (FuncState *fs);
void luaK_ret (FuncState *fs, int first, int nret);
void luaK_patchlist (FuncState *fs, int list, int target);
void luaK_patchtohere (FuncState *fs, int list);
void luaK_concat (FuncState *fs, int *l1, int l2);
int luaK_getlabel (FuncState *fs);
void luaK_prefix (FuncState *fs, UnOpr op, expdesc *v, int line);
void luaK_infix (FuncState *fs, BinOpr op, expdesc *v);
void luaK_posfix (FuncState *fs, BinOpr op, expdesc *v1, expdesc *v2, int line);
void luaK_settablesize (FuncState *fs, int pc, int ra, int asize, int hsize);
void luaK_setlist (FuncState *fs, int base, int nelems, int tostore);
void luaK_finish (FuncState *fs);
l_noret luaK_semerror (LexState *ls, const char *msg);
```

### Lexical Analyzer (Tokenization of Strings) (llex.h)
```c
#include <limits.h>

#include "lobject.h"
#include "lzio.h"

enum RESERVED {
  /* terminal symbols denoted by reserved words */
  TK_AND = FIRST_RESERVED, TK_BREAK,
  TK_DO, TK_ELSE, TK_ELSEIF, TK_END, TK_FALSE, TK_FOR, TK_FUNCTION,
  TK_GOTO, TK_IF, TK_IN, TK_LOCAL, TK_NIL, TK_NOT, TK_OR, TK_REPEAT,
  TK_RETURN, TK_THEN, TK_TRUE, TK_UNTIL, TK_WHILE,
  /* other terminal symbols */
  TK_IDIV, TK_CONCAT, TK_DOTS, TK_EQ, TK_GE, TK_LE, TK_NE,
  TK_SHL, TK_SHR,
  TK_DBCOLON, TK_EOS,
  TK_FLT, TK_INT, TK_NAME, TK_STRING
};

typedef union {
  lua_Number r;
  lua_Integer i;
  TString *ts;
} SemInfo;  /* semantics information */


typedef struct Token {
  int token;
  SemInfo seminfo;
} Token;


/* state of the lexer plus state of the parser when shared by all
   functions */
typedef struct LexState {
  int current;  /* current character (charint) */
  int linenumber;  /* input line counter */
  int lastline;  /* line of last token 'consumed' */
  Token t;  /* current token */
  Token lookahead;  /* look ahead token */
  struct FuncState *fs;  /* current function (parser) */
  struct lua_State *L;
  ZIO *z;  /* input stream */
  Mbuffer *buff;  /* buffer for tokens */
  Table *h;  /* to avoid collection/reuse strings */
  struct Dyndata *dyd;  /* dynamic structures used by the parser */
  TString *source;  /* current source name */
  TString *envn;  /* environment variable name */
} LexState;

void luaX_init (lua_State *L);
void luaX_setinput (lua_State *L, LexState *ls, ZIO *z, TString *source, int firstchar);
TString *luaX_newstring (LexState *ls, const char *str, size_t l);
void luaX_next (LexState *ls);
int luaX_lookahead (LexState *ls);
l_noret luaX_syntaxerror (LexState *ls, const char *s);
const char *luaX_token2str (LexState *ls, int token);
```

### Parser (lparser.h)
```c
#include "llimits.h"
#include "lobject.h"
#include "lzio.h"

/* kinds of variables/expressions */
typedef enum {
  VVOID,  /* when 'expdesc' describes the last expression of a list,
             this kind means an empty list (so, no expression) */
  VNIL,  /* constant nil */
  VTRUE,  /* constant true */
  VFALSE,  /* constant false */
  VK,  /* constant in 'k'; info = index of constant in 'k' */
  VKFLT,  /* floating constant; nval = numerical float value */
  VKINT,  /* integer constant; ival = numerical integer value */
  VKSTR,  /* string constant; strval = TString address;
             (string is fixed by the lexer) */
  VNONRELOC,  /* expression has its value in a fixed register;
                 info = result register */
  VLOCAL,  /* local variable; var.ridx = register index;
              var.vidx = relative index in 'actvar.arr'  */
  VUPVAL,  /* upvalue variable; info = index of upvalue in 'upvalues' */
  VCONST,  /* compile-time <const> variable;
              info = absolute index in 'actvar.arr'  */
  VINDEXED,  /* indexed variable;
                ind.t = table register;
                ind.idx = key's R index */
  VINDEXUP,  /* indexed upvalue;
                ind.t = table upvalue;
                ind.idx = key's K index */
  VINDEXI, /* indexed variable with constant integer;
                ind.t = table register;
                ind.idx = key's value */
  VINDEXSTR, /* indexed variable with literal string;
                ind.t = table register;
                ind.idx = key's K index */
  VJMP,  /* expression is a test/comparison;
            info = pc of corresponding jump instruction */
  VRELOC,  /* expression can put result in any register;
              info = instruction pc */
  VCALL,  /* expression is a function call; info = instruction pc */
  VVARARG  /* vararg expression; info = instruction pc */
} expkind;

typedef struct expdesc {
  expkind k;
  union {
    lua_Integer ival;    /* for VKINT */
    lua_Number nval;  /* for VKFLT */
    TString *strval;  /* for VKSTR */
    int info;  /* for generic use */
    struct {  /* for indexed variables */
      short idx;  /* index (R or "long" K) */
      lu_byte t;  /* table (register or upvalue) */
    } ind;
    struct {  /* for local variables */
      lu_byte ridx;  /* register holding the variable */
      unsigned short vidx;  /* compiler index (in 'actvar.arr')  */
    } var;
  } u;
  int t;  /* patch list of 'exit when true' */
  int f;  /* patch list of 'exit when false' */
} expdesc;

/* kinds of variables */
#define VDKREG		0   /* regular */
#define RDKCONST	1   /* constant */
#define RDKTOCLOSE	2   /* to-be-closed */
#define RDKCTC		3   /* compile-time constant */

/* description of an active local variable */
typedef union Vardesc {
  struct {
    TValuefields;  /* constant value (if it is a compile-time constant) */
    lu_byte kind;
    lu_byte ridx;  /* register holding the variable */
    short pidx;  /* index of the variable in the Proto's 'locvars' array */
    TString *name;  /* variable name */
  } vd;
  TValue k;  /* constant value (if any) */
} Vardesc;

/* description of pending goto statements and label statements */
typedef struct Labeldesc {
  TString *name;  /* label identifier */
  int pc;  /* position in code */
  int line;  /* line where it appeared */
  lu_byte nactvar;  /* number of active variables in that position */
  lu_byte close;  /* goto that escapes upvalues */
} Labeldesc;


/* list of labels or gotos */
typedef struct Labellist {
  Labeldesc *arr;  /* array */
  int n;  /* number of entries in use */
  int size;  /* array size */
} Labellist;

/* dynamic structures used by the parser */
typedef struct Dyndata {
  struct {  /* list of all active local variables */
    Vardesc *arr;
    int n;
    int size;
  } actvar;
  Labellist gt;  /* list of pending gotos */
  Labellist label;   /* list of active labels */
} Dyndata;

/* control of blocks */
struct BlockCnt;  /* defined in lparser.c */

/* state needed to generate code for a given function */
typedef struct FuncState {
  Proto *f;  /* current function header */
  struct FuncState *prev;  /* enclosing function */
  struct LexState *ls;  /* lexical state */
  struct BlockCnt *bl;  /* chain of current blocks */
  int pc;  /* next position to code (equivalent to 'ncode') */
  int lasttarget;   /* 'label' of last 'jump label' */
  int previousline;  /* last line that was saved in 'lineinfo' */
  int nk;  /* number of elements in 'k' */
  int np;  /* number of elements in 'p' */
  int nabslineinfo;  /* number of elements in 'abslineinfo' */
  int firstlocal;  /* index of first local var (in Dyndata array) */
  int firstlabel;  /* index of first label (in 'dyd->label->arr') */
  short ndebugvars;  /* number of elements in 'f->locvars' */
  lu_byte nactvar;  /* number of active local variables */
  lu_byte nups;  /* number of upvalues */
  lu_byte freereg;  /* first free register */
  lu_byte iwthabs;  /* instructions issued since last absolute line info */
  lu_byte needclose;  /* function needs to close upvalues when returning */
} FuncState;

int luaY_nvarstack (FuncState *fs);
LClosure *luaY_parser (lua_State *L, ZIO *z, Mbuffer *buff, Dyndata *dyd, const char *name, int firstchar);
```

### Precompiled Chunk Utilities (lundump.h and ldump.c)
```c
#include "llimits.h"
#include "lobject.h"
#include "lzio.h"

/* load one chunk; from lundump.c */
LClosure* luaU_undump (lua_State* L, ZIO* Z, const char* name);
/* dump one chunk; from ldump.c */
int luaU_dump (lua_State* L, const Proto* f, lua_Writer w, void* data, int strip);
```

## Bytecode Execution (The LVM)
### Opcode Definitions (lopcode.h)
```c
#include "llimits.h"

typedef enum {
/*----------------------------------------------------------------------
  name		args	description
------------------------------------------------------------------------*/
OP_MOVE,/*	A B	R[A] := R[B]					*/
OP_LOADI,/*	A sBx	R[A] := sBx					*/
OP_LOADF,/*	A sBx	R[A] := (lua_Number)sBx				*/
OP_LOADK,/*	A Bx	R[A] := K[Bx]					*/
OP_LOADKX,/*	A	R[A] := K[extra arg]				*/
OP_LOADFALSE,/*	A	R[A] := false					*/
OP_LFALSESKIP,/*A	R[A] := false; pc++	(*)			*/
OP_LOADTRUE,/*	A	R[A] := true					*/
OP_LOADNIL,/*	A B	R[A], R[A+1], ..., R[A+B] := nil		*/
OP_GETUPVAL,/*	A B	R[A] := UpValue[B]				*/
OP_SETUPVAL,/*	A B	UpValue[B] := R[A]				*/

OP_GETTABUP,/*	A B C	R[A] := UpValue[B][K[C]:string]			*/
OP_GETTABLE,/*	A B C	R[A] := R[B][R[C]]				*/
OP_GETI,/*	A B C	R[A] := R[B][C]					*/
OP_GETFIELD,/*	A B C	R[A] := R[B][K[C]:string]			*/

OP_SETTABUP,/*	A B C	UpValue[A][K[B]:string] := RK(C)		*/
OP_SETTABLE,/*	A B C	R[A][R[B]] := RK(C)				*/
OP_SETI,/*	A B C	R[A][B] := RK(C)				*/
OP_SETFIELD,/*	A B C	R[A][K[B]:string] := RK(C)			*/

OP_NEWTABLE,/*	A B C k	R[A] := {}					*/

OP_SELF,/*	A B C	R[A+1] := R[B]; R[A] := R[B][RK(C):string]	*/

OP_ADDI,/*	A B sC	R[A] := R[B] + sC				*/

OP_ADDK,/*	A B C	R[A] := R[B] + K[C]:number			*/
OP_SUBK,/*	A B C	R[A] := R[B] - K[C]:number			*/
OP_MULK,/*	A B C	R[A] := R[B] * K[C]:number			*/
OP_MODK,/*	A B C	R[A] := R[B] % K[C]:number			*/
OP_POWK,/*	A B C	R[A] := R[B] ^ K[C]:number			*/
OP_DIVK,/*	A B C	R[A] := R[B] / K[C]:number			*/
OP_IDIVK,/*	A B C	R[A] := R[B] // K[C]:number			*/

OP_BANDK,/*	A B C	R[A] := R[B] & K[C]:integer			*/
OP_BORK,/*	A B C	R[A] := R[B] | K[C]:integer			*/
OP_BXORK,/*	A B C	R[A] := R[B] ~ K[C]:integer			*/

OP_SHRI,/*	A B sC	R[A] := R[B] >> sC				*/
OP_SHLI,/*	A B sC	R[A] := sC << R[B]				*/

OP_ADD,/*	A B C	R[A] := R[B] + R[C]				*/
OP_SUB,/*	A B C	R[A] := R[B] - R[C]				*/
OP_MUL,/*	A B C	R[A] := R[B] * R[C]				*/
OP_MOD,/*	A B C	R[A] := R[B] % R[C]				*/
OP_POW,/*	A B C	R[A] := R[B] ^ R[C]				*/
OP_DIV,/*	A B C	R[A] := R[B] / R[C]				*/
OP_IDIV,/*	A B C	R[A] := R[B] // R[C]				*/

OP_BAND,/*	A B C	R[A] := R[B] & R[C]				*/
OP_BOR,/*	A B C	R[A] := R[B] | R[C]				*/
OP_BXOR,/*	A B C	R[A] := R[B] ~ R[C]				*/
OP_SHL,/*	A B C	R[A] := R[B] << R[C]				*/
OP_SHR,/*	A B C	R[A] := R[B] >> R[C]				*/

OP_MMBIN,/*	A B C	call C metamethod over R[A] and R[B]	(*)	*/
OP_MMBINI,/*	A sB C k	call C metamethod over R[A] and sB	*/
OP_MMBINK,/*	A B C k		call C metamethod over R[A] and K[B]	*/

OP_UNM,/*	A B	R[A] := -R[B]					*/
OP_BNOT,/*	A B	R[A] := ~R[B]					*/
OP_NOT,/*	A B	R[A] := not R[B]				*/
OP_LEN,/*	A B	R[A] := #R[B] (length operator)			*/

OP_CONCAT,/*	A B	R[A] := R[A].. ... ..R[A + B - 1]		*/

OP_CLOSE,/*	A	close all upvalues >= R[A]			*/
OP_TBC,/*	A	mark variable A "to be closed"			*/
OP_JMP,/*	sJ	pc += sJ					*/
OP_EQ,/*	A B k	if ((R[A] == R[B]) ~= k) then pc++		*/
OP_LT,/*	A B k	if ((R[A] <  R[B]) ~= k) then pc++		*/
OP_LE,/*	A B k	if ((R[A] <= R[B]) ~= k) then pc++		*/

OP_EQK,/*	A B k	if ((R[A] == K[B]) ~= k) then pc++		*/
OP_EQI,/*	A sB k	if ((R[A] == sB) ~= k) then pc++		*/
OP_LTI,/*	A sB k	if ((R[A] < sB) ~= k) then pc++			*/
OP_LEI,/*	A sB k	if ((R[A] <= sB) ~= k) then pc++		*/
OP_GTI,/*	A sB k	if ((R[A] > sB) ~= k) then pc++			*/
OP_GEI,/*	A sB k	if ((R[A] >= sB) ~= k) then pc++		*/

OP_TEST,/*	A k	if (not R[A] == k) then pc++			*/
OP_TESTSET,/*	A B k	if (not R[B] == k) then pc++ else R[A] := R[B] (*) */

OP_CALL,/*	A B C	R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1]) */
OP_TAILCALL,/*	A B C k	return R[A](R[A+1], ... ,R[A+B-1])		*/

OP_RETURN,/*	A B C k	return R[A], ... ,R[A+B-2]	(see note)	*/
OP_RETURN0,/*		return						*/
OP_RETURN1,/*	A	return R[A]					*/

OP_FORLOOP,/*	A Bx	update counters; if loop continues then pc-=Bx; */
OP_FORPREP,/*	A Bx	<check values and prepare counters>;
                        if not to run then pc+=Bx+1;			*/

OP_TFORPREP,/*	A Bx	create upvalue for R[A + 3]; pc+=Bx		*/
OP_TFORCALL,/*	A C	R[A+4], ... ,R[A+3+C] := R[A](R[A+1], R[A+2]);	*/
OP_TFORLOOP,/*	A Bx	if R[A+2] ~= nil then { R[A]=R[A+2]; pc -= Bx }	*/

OP_SETLIST,/*	A B C k	R[A][C+i] := R[A+i], 1 <= i <= B		*/

OP_CLOSURE,/*	A Bx	R[A] := closure(KPROTO[Bx])			*/

OP_VARARG,/*	A C	R[A], R[A+1], ..., R[A+C-2] = vararg		*/

OP_VARARGPREP,/*A	(adjust vararg parameters)			*/

OP_EXTRAARG/*	Ax	extra (larger) argument for previous opcode	*/
} OpCode;
```

### LVM Interface (lvm.h)
```c
#include "ldo.h"
#include "lobject.h"
#include "ltm.h"

/* Rounding modes for float->integer coercion */
typedef enum {
  F2Ieq,     /* no rounding; accepts only integral values */
  F2Ifloor,  /* takes the floor of the number */
  F2Iceil    /* takes the ceil of the number */
} F2Imod;

int luaV_equalobj (lua_State *L, const TValue *t1, const TValue *t2);
int luaV_lessthan (lua_State *L, const TValue *l, const TValue *r);
int luaV_lessequal (lua_State *L, const TValue *l, const TValue *r);
int luaV_tonumber_ (const TValue *obj, lua_Number *n);
int luaV_tointeger (const TValue *obj, lua_Integer *p, F2Imod mode);
int luaV_tointegerns (const TValue *obj, lua_Integer *p, F2Imod mode);
int luaV_flttointeger (lua_Number n, lua_Integer *p, F2Imod mode);
void luaV_finishget (lua_State *L, const TValue *t, TValue *key, StkId val, const TValue *slot);
void luaV_finishset (lua_State *L, const TValue *t, TValue *key, TValue *val, const TValue *slot);
void luaV_finishOp (lua_State *L);
void luaV_execute (lua_State *L, CallInfo *ci);
void luaV_concat (lua_State *L, int total);
lua_Integer luaV_idiv (lua_State *L, lua_Integer x, lua_Integer y);
lua_Integer luaV_mod (lua_State *L, lua_Integer x, lua_Integer y);
lua_Number luaV_modf (lua_State *L, lua_Number x, lua_Number y);
lua_Integer luaV_shiftl (lua_Integer x, lua_Integer y);
void luaV_objlen (lua_State *L, StkId ra, const TValue *rb);
```

### Stacks! (ldo.h)
```c
#include "lobject.h"
#include "lstate.h"
#include "lzio.h"

/* type of protected functions, to be ran by 'runprotected' */
typedef void (*Pfunc) (lua_State *L, void *ud);

void luaD_seterrorobj (lua_State *L, int errcode, StkId oldtop);
int luaD_protectedparser (lua_State *L, ZIO *z, const char *name, const char *mode);
void luaD_hook (lua_State *L, int event, int line, int fTransfer, int nTransfer);
void luaD_hookcall (lua_State *L, CallInfo *ci);
int luaD_pretailcall (lua_State *L, CallInfo *ci, StkId func, int narg1, int delta);
CallInfo *luaD_precall (lua_State *L, StkId func, int nResults);
void luaD_call (lua_State *L, StkId func, int nResults);
void luaD_callnoyield (lua_State *L, StkId func, int nResults);
StkId luaD_tryfuncTM (lua_State *L, StkId func);
int luaD_closeprotected (lua_State *L, ptrdiff_t level, int status);
int luaD_pcall (lua_State *L, Pfunc func, void *u, ptrdiff_t oldtop, ptrdiff_t ef);
void luaD_poscall (lua_State *L, CallInfo *ci, int nres);
int luaD_reallocstack (lua_State *L, int newsize, int raiseerror);
int luaD_growstack (lua_State *L, int n, int raiseerror);
void luaD_shrinkstack (lua_State *L);
void luaD_inctop (lua_State *L);

l_noret luaD_throw (lua_State *L, int errcode);
int luaD_rawrunprotected (lua_State *L, Pfunc f, void *ud);
```

### Metamethod Access (ltm.h)
```c
#include "lobject.h"

typedef enum {
  TM_INDEX,
  TM_NEWINDEX,
  TM_GC,
  TM_MODE,
  TM_LEN,
  TM_EQ, /* last tag method with fast access */
  TM_ADD,
  TM_SUB,
  TM_MUL,
  TM_MOD,
  TM_POW,
  TM_DIV,
  TM_IDIV,
  TM_BAND,
  TM_BOR,
  TM_BXOR,
  TM_SHL,
  TM_SHR,
  TM_UNM,
  TM_BNOT,
  TM_LT,
  TM_LE,
  TM_CONCAT,
  TM_CALL,
  TM_CLOSE,
  TM_N /* number of elements in the enum */
} TMS;

LUAI_DDEC(const char *const luaT_typenames_[LUA_TOTALTYPES];)

const char *luaT_objtypename (lua_State *L, const TValue *o);
const TValue *luaT_gettm (Table *events, TMS event, TString *ename);
const TValue *luaT_gettmbyobj (lua_State *L, const TValue *o, TMS event);
void luaT_init (lua_State *L);
void luaT_callTM (lua_State *L, const TValue *f, const TValue *p1, const TValue *p2, const TValue *p3);
void luaT_callTMres (lua_State *L, const TValue *f, const TValue *p1, const TValue *p2, StkId p3);
void luaT_trybinTM (lua_State *L, const TValue *p1, const TValue *p2, StkId res, TMS event);
void luaT_tryconcatTM (lua_State *L);
void luaT_trybinassocTM (lua_State *L, const TValue *p1, const TValue *p2, int inv, StkId res, TMS event);
void luaT_trybiniTM (lua_State *L, const TValue *p1, lua_Integer i2, int inv, StkId res, TMS event);
int luaT_callorderTM (lua_State *L, const TValue *p1, const TValue *p2, TMS event);
int luaT_callorderiTM (lua_State *L, const TValue *p1, int v2, int inv, int isfloat, TMS event);
void luaT_adjustvarargs (lua_State *L, int nfixparams, struct CallInfo *ci, const Proto *p);
void luaT_getvarargs (lua_State *L, struct CallInfo *ci, StkId where, int wanted);
```
