//! https://sqlite.org/opcode.html

use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Opcode {
    Savepoint,
    AutoCommit,
    Transaction,
    SorterNext, /* jump                                       */
    Prev,       /* jump                                       */
    Next,       /* jump                                       */
    Checkpoint,
    JournalMode,
    Vacuum,
    VFilter,       /* jump, synopsis: iplan=r[P3] zplan='P4'     */
    VUpdate,       /* synopsis: data=r[P3@P2]                    */
    Goto,          /* jump                                       */
    Gosub,         /* jump                                       */
    InitCoroutine, /* jump                                       */
    Yield,         /* jump                                       */
    MustBeInt,     /* jump                                       */
    Jump,          /* jump                                       */
    Once,          /* jump                                       */
    If,            /* jump                                       */
    Not,           /* same as TK_NOT, synopsis: r[P2]= !r[P1]    */
    IfNot,         /* jump                                       */
    IsNullOrType,  /* jump, synopsis: if typeof(r[P1]) IN (P3,5) goto P2 */
    IfNullRow,     /* jump, synopsis: if P1.nullRow then r[P3]=NULL, goto P2 */
    SeekLT,        /* jump, synopsis: key=r[P3@P4]               */
    SeekLE,        /* jump, synopsis: key=r[P3@P4]               */
    SeekGE,        /* jump, synopsis: key=r[P3@P4]               */
    SeekGT,        /* jump, synopsis: key=r[P3@P4]               */
    IfNotOpen,     /* jump, synopsis: if( !csr[P1] ) goto P2     */
    IfNoHope,      /* jump, synopsis: key=r[P3@P4]               */
    NoConflict,    /* jump, synopsis: key=r[P3@P4]               */
    NotFound,      /* jump, synopsis: key=r[P3@P4]               */
    Found,         /* jump, synopsis: key=r[P3@P4]               */
    SeekRowid,     /* jump, synopsis: intkey=r[P3]               */
    NotExists,     /* jump, synopsis: intkey=r[P3]               */
    Last,          /* jump                                       */
    IfSmaller,     /* jump                                       */
    SorterSort,    /* jump                                       */
    Sort,          /* jump                                       */
    Rewind,        /* jump                                       */
    IdxLE,         /* jump, synopsis: key=r[P3@P4]               */
    IdxGT,         /* jump, synopsis: key=r[P3@P4]               */
    IdxLT,         /* jump, synopsis: key=r[P3@P4]               */
    IdxGE,         /* jump, synopsis: key=r[P3@P4]               */
    Or,            /* same as TK_OR, synopsis: r[P3]=(r[P1] || r[P2]) */
    And,           /* same as TK_AND, synopsis: r[P3]=(r[P1] && r[P2]) */
    RowSetRead,    /* jump, synopsis: r[P3]=rowset(P1)           */
    RowSetTest,    /* jump, synopsis: if r[P3] in rowset(P1) goto P2 */
    Program,       /* jump                                       */
    FkIfZero,      /* jump, synopsis: if fkctr[P1]==0 goto P2    */
    IfPos,         /* jump, synopsis: if r[P1]>0 then r[P1]-=P3, goto P2 */
    IsNull,        /* jump, same as TK_ISNULL, synopsis: if r[P1]==NULL goto P2 */
    NotNull,       /* jump, same as TK_NOTNULL, synopsis: if r[P1]!=NULL goto P2 */
    Ne,            /* jump, same as TK_NE, synopsis: IF r[P3]!=r[P1] */
    Eq,            /* jump, same as TK_EQ, synopsis: IF r[P3]==r[P1] */
    Gt,            /* jump, same as TK_GT, synopsis: IF r[P3]>r[P1] */
    Le,            /* jump, same as TK_LE, synopsis: IF r[P3]<=r[P1] */
    Lt,            /* jump, same as TK_LT, synopsis: IF r[P3]<r[P1] */
    Ge,            /* jump, same as TK_GE, synopsis: IF r[P3]>=r[P1] */
    ElseEq,        /* jump, same as TK_ESCAPE                    */
    IfNotZero,     /* jump, synopsis: if r[P1]!=0 then r[P1]--, goto P2 */
    DecrJumpZero,  /* jump, synopsis: if (--r[P1])==0 goto P2    */
    IncrVacuum,    /* jump                                       */
    VNext,         /* jump                                       */
    Filter,        /* jump, synopsis: if key(P3@P4) not in filter(P1) goto P2 */
    Init,          /* jump, synopsis: Start at P2                */
    PureFunc,      /* synopsis: r[P3]=func(r[P2@NP])             */
    Function,      /* synopsis: r[P3]=func(r[P2@NP])             */
    Return,
    EndCoroutine,
    HaltIfNull, /* synopsis: if r[P3]=null halt               */
    Halt,
    Integer,  /* synopsis: r[P2]=P1                         */
    Int64,    /* synopsis: r[P2]=P4                         */
    String,   /* synopsis: r[P2]='P4' (len=P1)              */
    Null,     /* synopsis: r[P2..P3]=NULL                   */
    SoftNull, /* synopsis: r[P1]=NULL                       */
    Blob,     /* synopsis: r[P2]=P4 (len=P1)                */
    Variable, /* synopsis: r[P2]=parameter(P1,P4)           */
    Move,     /* synopsis: r[P2@P3]=r[P1@P3]                */
    Copy,     /* synopsis: r[P2@P3+1]=r[P1@P3+1]            */
    SCopy,    /* synopsis: r[P2]=r[P1]                      */
    IntCopy,  /* synopsis: r[P2]=r[P1]                      */
    FkCheck,
    ResultRow, /* synopsis: output=r[P1@P2]                  */
    CollSeq,
    AddImm, /* synopsis: r[P1]=r[P1]+P2                   */
    RealAffinity,
    Cast, /* synopsis: affinity(r[P1])                  */
    Permutation,
    Compare,    /* synopsis: r[P1@P3] <-> r[P2@P3]            */
    IsTrue,     /* synopsis: r[P2] = coalesce(r[P1]==TRUE,P3) ^ P4 */
    ZeroOrNull, /* synopsis: r[P2] = 0 OR NULL                */
    Offset,     /* synopsis: r[P3] = sqlite_offset(P1)        */
    Column,     /* synopsis: r[P3]=PX                         */
    TypeCheck,  /* synopsis: typecheck(r[P1@P2])              */
    Affinity,   /* synopsis: affinity(r[P1@P2])               */
    MakeRecord, /* synopsis: r[P3]=mkrec(r[P1@P2])            */
    Count,      /* synopsis: r[P2]=count()                    */
    ReadCookie,
    SetCookie,
    ReopenIdx,  /* synopsis: root=P2 iDb=P3                   */
    OpenRead,   /* synopsis: root=P2 iDb=P3                   */
    BitAnd,     /* same as TK_BITAND, synopsis: r[P3]=r[P1]&r[P2] */
    BitOr,      /* same as TK_BITOR, synopsis: r[P3]=r[P1]|r[P2] */
    ShiftLeft,  /* same as TK_LSHIFT, synopsis: r[P3]=r[P2]<<r[P1] */
    ShiftRight, /* same as TK_RSHIFT, synopsis: r[P3]=r[P2]>>r[P1] */
    Add,        /* same as TK_PLUS, synopsis: r[P3]=r[P1]+r[P2] */
    Subtract,   /* same as TK_MINUS, synopsis: r[P3]=r[P2]-r[P1] */
    Multiply,   /* same as TK_STAR, synopsis: r[P3]=r[P1]*r[P2] */
    Divide,     /* same as TK_SLASH, synopsis: r[P3]=r[P2]/r[P1] */
    Remainder,  /* same as TK_REM, synopsis: r[P3]=r[P2]%r[P1] */
    Concat,     /* same as TK_CONCAT, synopsis: r[P3]=r[P2]+r[P1] */
    OpenWrite,  /* synopsis: root=P2 iDb=P3                   */
    OpenDup,
    BitNot,        /* same as TK_BITNOT, synopsis: r[P2]= ~r[P1] */
    OpenAutoindex, /* synopsis: nColumn=P2                       */
    OpenEphemeral, /* synopsis: nColumn=P2                       */
    String8,       /* same as TK_STRING, synopsis: r[P2]='P4'    */
    SorterOpen,
    SequenceTest, /* synopsis: if( cursor[P1].ctr++ ) pc = P2   */
    OpenPseudo,   /* synopsis: P3 columns in r[P2]              */
    Close,
    ColumnsUsed,
    SeekScan, /* synopsis: Scan-ahead up to P1 rows         */
    SeekHit,  /* synopsis: set P2<=seekHit<=P3              */
    Sequence, /* synopsis: r[P2]=cursor[P1].ctr++           */
    NewRowid, /* synopsis: r[P2]=rowid                      */
    Insert,   /* synopsis: intkey=r[P3] data=r[P2]          */
    RowCell,
    Delete,
    ResetCount,
    SorterCompare, /* synopsis: if key(P1)!=trim(r[P3],P4) goto P2 */
    SorterData,    /* synopsis: r[P2]=data                       */
    RowData,       /* synopsis: r[P2]=data                       */
    Rowid,         /* synopsis: r[P2]=rowid                      */
    NullRow,
    SeekEnd,
    IdxInsert,    /* synopsis: key=r[P2]                        */
    SorterInsert, /* synopsis: key=r[P2]                        */
    IdxDelete,    /* synopsis: key=r[P2@P3]                     */
    DeferredSeek, /* synopsis: Move P3 to P1.rowid if needed    */
    IdxRowid,     /* synopsis: r[P2]=rowid                      */
    FinishSeek,
    Destroy,
    Clear,
    ResetSorter,
    CreateBtree, /* synopsis: r[P2]=root iDb=P1 flags=P3       */
    SqlExec,
    ParseSchema,
    LoadAnalysis,
    DropTable,
    DropIndex,
    DropTrigger,
    Real, /* same as TK_FLOAT, synopsis: r[P2]=P4       */
    IntegrityCk,
    RowSetAdd, /* synopsis: rowset(P1)=r[P2]                 */
    Param,
    FkCounter,   /* synopsis: fkctr[P1]+=P2                    */
    MemMax,      /* synopsis: r[P1]=max(r[P1],r[P2])           */
    OffsetLimit, /* synopsis: if r[P1]>0 then r[P2]=r[P1]+max(0,r[P3]) else r[P2]=(-1) */
    AggInverse,  /* synopsis: accum=r[P3] inverse(r[P2@P5])    */
    AggStep,     /* synopsis: accum=r[P3] step(r[P2@P5])       */
    AggStep1,    /* synopsis: accum=r[P3] step(r[P2@P5])       */
    AggValue,    /* synopsis: r[P3]=value N=P2                 */
    AggFinal,    /* synopsis: accum=r[P1] N=P2                 */
    Expire,
    CursorLock,
    CursorUnlock,
    TableLock, /* synopsis: iDb=P1 root=P2 write=P3          */
    VBegin,
    VCreate,
    VDestroy,
    VOpen,
    VInitIn, /* synopsis: r[P2]=ValueList(P1,P3)           */
    VColumn, /* synopsis: r[P3]=vcolumn(P2)                */
    VRename,
    Pagecount,
    MaxPgcnt,
    FilterAdd, /* synopsis: filter(P1) += key(P3@P4)         */
    Trace,
    CursorHint,
    ReleaseReg, /* synopsis: release r[P1@P2] mask P3         */
    Noop,
    Explain,
    Abortable,
}
