#[allow(non_camel_case_types)]

pub const SQLITE_VERSION: &[u8; 7usize] = b"3.14.0\0";
pub const SQLITE_VERSION_NUMBER: i32 = 3014000;
pub const SQLITE_SOURCE_ID: &[u8; 61usize] =
    b"2016-08-08 13:40:27 d5e98057028abcf7217d0d2b2e29bbbcdf09d6de\0";
pub const SQLITE_OK: i32 = 0;
pub const SQLITE_ERROR: i32 = 1;
pub const SQLITE_INTERNAL: i32 = 2;
pub const SQLITE_PERM: i32 = 3;
pub const SQLITE_ABORT: i32 = 4;
pub const SQLITE_BUSY: i32 = 5;
pub const SQLITE_LOCKED: i32 = 6;
pub const SQLITE_NOMEM: i32 = 7;
pub const SQLITE_READONLY: i32 = 8;
pub const SQLITE_INTERRUPT: i32 = 9;
pub const SQLITE_IOERR: i32 = 10;
pub const SQLITE_CORRUPT: i32 = 11;
pub const SQLITE_NOTFOUND: i32 = 12;
pub const SQLITE_FULL: i32 = 13;
pub const SQLITE_CANTOPEN: i32 = 14;
pub const SQLITE_PROTOCOL: i32 = 15;
pub const SQLITE_EMPTY: i32 = 16;
pub const SQLITE_SCHEMA: i32 = 17;
pub const SQLITE_TOOBIG: i32 = 18;
pub const SQLITE_CONSTRAINT: i32 = 19;
pub const SQLITE_MISMATCH: i32 = 20;
pub const SQLITE_MISUSE: i32 = 21;
pub const SQLITE_NOLFS: i32 = 22;
pub const SQLITE_AUTH: i32 = 23;
pub const SQLITE_FORMAT: i32 = 24;
pub const SQLITE_RANGE: i32 = 25;
pub const SQLITE_NOTADB: i32 = 26;
pub const SQLITE_NOTICE: i32 = 27;
pub const SQLITE_WARNING: i32 = 28;
pub const SQLITE_ROW: i32 = 100;
pub const SQLITE_DONE: i32 = 101;
pub const SQLITE_IOERR_READ: i32 = 266;
pub const SQLITE_IOERR_SHORT_READ: i32 = 522;
pub const SQLITE_IOERR_WRITE: i32 = 778;
pub const SQLITE_IOERR_FSYNC: i32 = 1034;
pub const SQLITE_IOERR_DIR_FSYNC: i32 = 1290;
pub const SQLITE_IOERR_TRUNCATE: i32 = 1546;
pub const SQLITE_IOERR_FSTAT: i32 = 1802;
pub const SQLITE_IOERR_UNLOCK: i32 = 2058;
pub const SQLITE_IOERR_RDLOCK: i32 = 2314;
pub const SQLITE_IOERR_DELETE: i32 = 2570;
pub const SQLITE_IOERR_BLOCKED: i32 = 2826;
pub const SQLITE_IOERR_NOMEM: i32 = 3082;
pub const SQLITE_IOERR_ACCESS: i32 = 3338;
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: i32 = 3594;
pub const SQLITE_IOERR_LOCK: i32 = 3850;
pub const SQLITE_IOERR_CLOSE: i32 = 4106;
pub const SQLITE_IOERR_DIR_CLOSE: i32 = 4362;
pub const SQLITE_IOERR_SHMOPEN: i32 = 4618;
pub const SQLITE_IOERR_SHMSIZE: i32 = 4874;
pub const SQLITE_IOERR_SHMLOCK: i32 = 5130;
pub const SQLITE_IOERR_SHMMAP: i32 = 5386;
pub const SQLITE_IOERR_SEEK: i32 = 5642;
pub const SQLITE_IOERR_DELETE_NOENT: i32 = 5898;
pub const SQLITE_IOERR_MMAP: i32 = 6154;
pub const SQLITE_IOERR_GETTEMPPATH: i32 = 6410;
pub const SQLITE_IOERR_CONVPATH: i32 = 6666;
pub const SQLITE_IOERR_VNODE: i32 = 6922;
pub const SQLITE_IOERR_AUTH: i32 = 7178;
pub const SQLITE_LOCKED_SHAREDCACHE: i32 = 262;
pub const SQLITE_BUSY_RECOVERY: i32 = 261;
pub const SQLITE_BUSY_SNAPSHOT: i32 = 517;
pub const SQLITE_CANTOPEN_NOTEMPDIR: i32 = 270;
pub const SQLITE_CANTOPEN_ISDIR: i32 = 526;
pub const SQLITE_CANTOPEN_FULLPATH: i32 = 782;
pub const SQLITE_CANTOPEN_CONVPATH: i32 = 1038;
pub const SQLITE_CORRUPT_VTAB: i32 = 267;
pub const SQLITE_READONLY_RECOVERY: i32 = 264;
pub const SQLITE_READONLY_CANTLOCK: i32 = 520;
pub const SQLITE_READONLY_ROLLBACK: i32 = 776;
pub const SQLITE_READONLY_DBMOVED: i32 = 1032;
pub const SQLITE_ABORT_ROLLBACK: i32 = 516;
pub const SQLITE_CONSTRAINT_CHECK: i32 = 275;
pub const SQLITE_CONSTRAINT_COMMITHOOK: i32 = 531;
pub const SQLITE_CONSTRAINT_FOREIGNKEY: i32 = 787;
pub const SQLITE_CONSTRAINT_FUNCTION: i32 = 1043;
pub const SQLITE_CONSTRAINT_NOTNULL: i32 = 1299;
pub const SQLITE_CONSTRAINT_PRIMARYKEY: i32 = 1555;
pub const SQLITE_CONSTRAINT_TRIGGER: i32 = 1811;
pub const SQLITE_CONSTRAINT_UNIQUE: i32 = 2067;
pub const SQLITE_CONSTRAINT_VTAB: i32 = 2323;
pub const SQLITE_CONSTRAINT_ROWID: i32 = 2579;
pub const SQLITE_NOTICE_RECOVER_WAL: i32 = 283;
pub const SQLITE_NOTICE_RECOVER_ROLLBACK: i32 = 539;
pub const SQLITE_WARNING_AUTOINDEX: i32 = 284;
pub const SQLITE_AUTH_USER: i32 = 279;
pub const SQLITE_OK_LOAD_PERMANENTLY: i32 = 256;
pub const SQLITE_OPEN_READONLY: i32 = 1;
pub const SQLITE_OPEN_READWRITE: i32 = 2;
pub const SQLITE_OPEN_CREATE: i32 = 4;
pub const SQLITE_OPEN_DELETEONCLOSE: i32 = 8;
pub const SQLITE_OPEN_EXCLUSIVE: i32 = 16;
pub const SQLITE_OPEN_AUTOPROXY: i32 = 32;
pub const SQLITE_OPEN_URI: i32 = 64;
pub const SQLITE_OPEN_MEMORY: i32 = 128;
pub const SQLITE_OPEN_MAIN_DB: i32 = 256;
pub const SQLITE_OPEN_TEMP_DB: i32 = 512;
pub const SQLITE_OPEN_TRANSIENT_DB: i32 = 1024;
pub const SQLITE_OPEN_MAIN_JOURNAL: i32 = 2048;
pub const SQLITE_OPEN_TEMP_JOURNAL: i32 = 4096;
pub const SQLITE_OPEN_SUBJOURNAL: i32 = 8192;
pub const SQLITE_OPEN_MASTER_JOURNAL: i32 = 16384;
pub const SQLITE_OPEN_NOMUTEX: i32 = 32768;
pub const SQLITE_OPEN_FULLMUTEX: i32 = 65536;
pub const SQLITE_OPEN_SHAREDCACHE: i32 = 131072;
pub const SQLITE_OPEN_PRIVATECACHE: i32 = 262144;
pub const SQLITE_OPEN_WAL: i32 = 524288;
pub const SQLITE_IOCAP_ATOMIC: i32 = 1;
pub const SQLITE_IOCAP_ATOMIC512: i32 = 2;
pub const SQLITE_IOCAP_ATOMIC1K: i32 = 4;
pub const SQLITE_IOCAP_ATOMIC2K: i32 = 8;
pub const SQLITE_IOCAP_ATOMIC4K: i32 = 16;
pub const SQLITE_IOCAP_ATOMIC8K: i32 = 32;
pub const SQLITE_IOCAP_ATOMIC16K: i32 = 64;
pub const SQLITE_IOCAP_ATOMIC32K: i32 = 128;
pub const SQLITE_IOCAP_ATOMIC64K: i32 = 256;
pub const SQLITE_IOCAP_SAFE_APPEND: i32 = 512;
pub const SQLITE_IOCAP_SEQUENTIAL: i32 = 1024;
pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: i32 = 2048;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: i32 = 4096;
pub const SQLITE_IOCAP_IMMUTABLE: i32 = 8192;
pub const SQLITE_LOCK_NONE: i32 = 0;
pub const SQLITE_LOCK_SHARED: i32 = 1;
pub const SQLITE_LOCK_RESERVED: i32 = 2;
pub const SQLITE_LOCK_PENDING: i32 = 3;
pub const SQLITE_LOCK_EXCLUSIVE: i32 = 4;
pub const SQLITE_SYNC_NORMAL: i32 = 2;
pub const SQLITE_SYNC_FULL: i32 = 3;
pub const SQLITE_SYNC_DATAONLY: i32 = 16;
pub const SQLITE_FCNTL_LOCKSTATE: i32 = 1;
pub const SQLITE_FCNTL_GET_LOCKPROXYFILE: i32 = 2;
pub const SQLITE_FCNTL_SET_LOCKPROXYFILE: i32 = 3;
pub const SQLITE_FCNTL_LAST_ERRNO: i32 = 4;
pub const SQLITE_FCNTL_SIZE_HINT: i32 = 5;
pub const SQLITE_FCNTL_CHUNK_SIZE: i32 = 6;
pub const SQLITE_FCNTL_FILE_POINTER: i32 = 7;
pub const SQLITE_FCNTL_SYNC_OMITTED: i32 = 8;
pub const SQLITE_FCNTL_WIN32_AV_RETRY: i32 = 9;
pub const SQLITE_FCNTL_PERSIST_WAL: i32 = 10;
pub const SQLITE_FCNTL_OVERWRITE: i32 = 11;
pub const SQLITE_FCNTL_VFSNAME: i32 = 12;
pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: i32 = 13;
pub const SQLITE_FCNTL_PRAGMA: i32 = 14;
pub const SQLITE_FCNTL_BUSYHANDLER: i32 = 15;
pub const SQLITE_FCNTL_TEMPFILENAME: i32 = 16;
pub const SQLITE_FCNTL_MMAP_SIZE: i32 = 18;
pub const SQLITE_FCNTL_TRACE: i32 = 19;
pub const SQLITE_FCNTL_HAS_MOVED: i32 = 20;
pub const SQLITE_FCNTL_SYNC: i32 = 21;
pub const SQLITE_FCNTL_COMMIT_PHASETWO: i32 = 22;
pub const SQLITE_FCNTL_WIN32_SET_HANDLE: i32 = 23;
pub const SQLITE_FCNTL_WAL_BLOCK: i32 = 24;
pub const SQLITE_FCNTL_ZIPVFS: i32 = 25;
pub const SQLITE_FCNTL_RBU: i32 = 26;
pub const SQLITE_FCNTL_VFS_POINTER: i32 = 27;
pub const SQLITE_FCNTL_JOURNAL_POINTER: i32 = 28;
pub const SQLITE_GET_LOCKPROXYFILE: i32 = 2;
pub const SQLITE_SET_LOCKPROXYFILE: i32 = 3;
pub const SQLITE_LAST_ERRNO: i32 = 4;
pub const SQLITE_ACCESS_EXISTS: i32 = 0;
pub const SQLITE_ACCESS_READWRITE: i32 = 1;
pub const SQLITE_ACCESS_READ: i32 = 2;
pub const SQLITE_SHM_UNLOCK: i32 = 1;
pub const SQLITE_SHM_LOCK: i32 = 2;
pub const SQLITE_SHM_SHARED: i32 = 4;
pub const SQLITE_SHM_EXCLUSIVE: i32 = 8;
pub const SQLITE_SHM_NLOCK: i32 = 8;
pub const SQLITE_CONFIG_SINGLETHREAD: i32 = 1;
pub const SQLITE_CONFIG_MULTITHREAD: i32 = 2;
pub const SQLITE_CONFIG_SERIALIZED: i32 = 3;
pub const SQLITE_CONFIG_MALLOC: i32 = 4;
pub const SQLITE_CONFIG_GETMALLOC: i32 = 5;
pub const SQLITE_CONFIG_SCRATCH: i32 = 6;
pub const SQLITE_CONFIG_PAGECACHE: i32 = 7;
pub const SQLITE_CONFIG_HEAP: i32 = 8;
pub const SQLITE_CONFIG_MEMSTATUS: i32 = 9;
pub const SQLITE_CONFIG_MUTEX: i32 = 10;
pub const SQLITE_CONFIG_GETMUTEX: i32 = 11;
pub const SQLITE_CONFIG_LOOKASIDE: i32 = 13;
pub const SQLITE_CONFIG_PCACHE: i32 = 14;
pub const SQLITE_CONFIG_GETPCACHE: i32 = 15;
pub const SQLITE_CONFIG_LOG: i32 = 16;
pub const SQLITE_CONFIG_URI: i32 = 17;
pub const SQLITE_CONFIG_PCACHE2: i32 = 18;
pub const SQLITE_CONFIG_GETPCACHE2: i32 = 19;
pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: i32 = 20;
pub const SQLITE_CONFIG_SQLLOG: i32 = 21;
pub const SQLITE_CONFIG_MMAP_SIZE: i32 = 22;
pub const SQLITE_CONFIG_WIN32_HEAPSIZE: i32 = 23;
pub const SQLITE_CONFIG_PCACHE_HDRSZ: i32 = 24;
pub const SQLITE_CONFIG_PMASZ: i32 = 25;
pub const SQLITE_CONFIG_STMTJRNL_SPILL: i32 = 26;
pub const SQLITE_DBCONFIG_LOOKASIDE: i32 = 1001;
pub const SQLITE_DBCONFIG_ENABLE_FKEY: i32 = 1002;
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: i32 = 1003;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: i32 = 1004;
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: i32 = 1005;
pub const SQLITE_DENY: i32 = 1;
pub const SQLITE_IGNORE: i32 = 2;
pub const SQLITE_CREATE_INDEX: i32 = 1;
pub const SQLITE_CREATE_TABLE: i32 = 2;
pub const SQLITE_CREATE_TEMP_INDEX: i32 = 3;
pub const SQLITE_CREATE_TEMP_TABLE: i32 = 4;
pub const SQLITE_CREATE_TEMP_TRIGGER: i32 = 5;
pub const SQLITE_CREATE_TEMP_VIEW: i32 = 6;
pub const SQLITE_CREATE_TRIGGER: i32 = 7;
pub const SQLITE_CREATE_VIEW: i32 = 8;
pub const SQLITE_DELETE: i32 = 9;
pub const SQLITE_DROP_INDEX: i32 = 10;
pub const SQLITE_DROP_TABLE: i32 = 11;
pub const SQLITE_DROP_TEMP_INDEX: i32 = 12;
pub const SQLITE_DROP_TEMP_TABLE: i32 = 13;
pub const SQLITE_DROP_TEMP_TRIGGER: i32 = 14;
pub const SQLITE_DROP_TEMP_VIEW: i32 = 15;
pub const SQLITE_DROP_TRIGGER: i32 = 16;
pub const SQLITE_DROP_VIEW: i32 = 17;
pub const SQLITE_INSERT: i32 = 18;
pub const SQLITE_PRAGMA: i32 = 19;
pub const SQLITE_READ: i32 = 20;
pub const SQLITE_SELECT: i32 = 21;
pub const SQLITE_TRANSACTION: i32 = 22;
pub const SQLITE_UPDATE: i32 = 23;
pub const SQLITE_ATTACH: i32 = 24;
pub const SQLITE_DETACH: i32 = 25;
pub const SQLITE_ALTER_TABLE: i32 = 26;
pub const SQLITE_REINDEX: i32 = 27;
pub const SQLITE_ANALYZE: i32 = 28;
pub const SQLITE_CREATE_VTABLE: i32 = 29;
pub const SQLITE_DROP_VTABLE: i32 = 30;
pub const SQLITE_FUNCTION: i32 = 31;
pub const SQLITE_SAVEPOINT: i32 = 32;
pub const SQLITE_COPY: i32 = 0;
pub const SQLITE_RECURSIVE: i32 = 33;
pub const SQLITE_TRACE_STMT: i32 = 1;
pub const SQLITE_TRACE_PROFILE: i32 = 2;
pub const SQLITE_TRACE_ROW: i32 = 4;
pub const SQLITE_TRACE_CLOSE: i32 = 8;
pub const SQLITE_LIMIT_LENGTH: i32 = 0;
pub const SQLITE_LIMIT_SQL_LENGTH: i32 = 1;
pub const SQLITE_LIMIT_COLUMN: i32 = 2;
pub const SQLITE_LIMIT_EXPR_DEPTH: i32 = 3;
pub const SQLITE_LIMIT_COMPOUND_SELECT: i32 = 4;
pub const SQLITE_LIMIT_VDBE_OP: i32 = 5;
pub const SQLITE_LIMIT_FUNCTION_ARG: i32 = 6;
pub const SQLITE_LIMIT_ATTACHED: i32 = 7;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: i32 = 8;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: i32 = 9;
pub const SQLITE_LIMIT_TRIGGER_DEPTH: i32 = 10;
pub const SQLITE_LIMIT_WORKER_THREADS: i32 = 11;
pub const SQLITE_INTEGER: i32 = 1;
pub const SQLITE_FLOAT: i32 = 2;
pub const SQLITE_BLOB: i32 = 4;
pub const SQLITE_NULL: i32 = 5;
pub const SQLITE_TEXT: i32 = 3;
pub const SQLITE3_TEXT: i32 = 3;
pub const SQLITE_UTF8: i32 = 1;
pub const SQLITE_UTF16LE: i32 = 2;
pub const SQLITE_UTF16BE: i32 = 3;
pub const SQLITE_UTF16: i32 = 4;
pub const SQLITE_ANY: i32 = 5;
pub const SQLITE_UTF16_ALIGNED: i32 = 8;
pub const SQLITE_DETERMINISTIC: i32 = 2048;
pub const SQLITE_INDEX_SCAN_UNIQUE: i32 = 1;
pub const SQLITE_INDEX_CONSTRAINT_EQ: i32 = 2;
pub const SQLITE_INDEX_CONSTRAINT_GT: i32 = 4;
pub const SQLITE_INDEX_CONSTRAINT_LE: i32 = 8;
pub const SQLITE_INDEX_CONSTRAINT_LT: i32 = 16;
pub const SQLITE_INDEX_CONSTRAINT_GE: i32 = 32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: i32 = 64;
pub const SQLITE_INDEX_CONSTRAINT_LIKE: i32 = 65;
pub const SQLITE_INDEX_CONSTRAINT_GLOB: i32 = 66;
pub const SQLITE_INDEX_CONSTRAINT_REGEXP: i32 = 67;
pub const SQLITE_MUTEX_FAST: i32 = 0;
pub const SQLITE_MUTEX_RECURSIVE: i32 = 1;
pub const SQLITE_MUTEX_STATIC_MASTER: i32 = 2;
pub const SQLITE_MUTEX_STATIC_MEM: i32 = 3;
pub const SQLITE_MUTEX_STATIC_MEM2: i32 = 4;
pub const SQLITE_MUTEX_STATIC_OPEN: i32 = 4;
pub const SQLITE_MUTEX_STATIC_PRNG: i32 = 5;
pub const SQLITE_MUTEX_STATIC_LRU: i32 = 6;
pub const SQLITE_MUTEX_STATIC_LRU2: i32 = 7;
pub const SQLITE_MUTEX_STATIC_PMEM: i32 = 7;
pub const SQLITE_MUTEX_STATIC_APP1: i32 = 8;
pub const SQLITE_MUTEX_STATIC_APP2: i32 = 9;
pub const SQLITE_MUTEX_STATIC_APP3: i32 = 10;
pub const SQLITE_MUTEX_STATIC_VFS1: i32 = 11;
pub const SQLITE_MUTEX_STATIC_VFS2: i32 = 12;
pub const SQLITE_MUTEX_STATIC_VFS3: i32 = 13;
pub const SQLITE_TESTCTRL_FIRST: i32 = 5;
pub const SQLITE_TESTCTRL_PRNG_SAVE: i32 = 5;
pub const SQLITE_TESTCTRL_PRNG_RESTORE: i32 = 6;
pub const SQLITE_TESTCTRL_PRNG_RESET: i32 = 7;
pub const SQLITE_TESTCTRL_BITVEC_TEST: i32 = 8;
pub const SQLITE_TESTCTRL_FAULT_INSTALL: i32 = 9;
pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: i32 = 10;
pub const SQLITE_TESTCTRL_PENDING_BYTE: i32 = 11;
pub const SQLITE_TESTCTRL_ASSERT: i32 = 12;
pub const SQLITE_TESTCTRL_ALWAYS: i32 = 13;
pub const SQLITE_TESTCTRL_RESERVE: i32 = 14;
pub const SQLITE_TESTCTRL_OPTIMIZATIONS: i32 = 15;
pub const SQLITE_TESTCTRL_ISKEYWORD: i32 = 16;
pub const SQLITE_TESTCTRL_SCRATCHMALLOC: i32 = 17;
pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: i32 = 18;
pub const SQLITE_TESTCTRL_EXPLAIN_STMT: i32 = 19;
pub const SQLITE_TESTCTRL_NEVER_CORRUPT: i32 = 20;
pub const SQLITE_TESTCTRL_VDBE_COVERAGE: i32 = 21;
pub const SQLITE_TESTCTRL_BYTEORDER: i32 = 22;
pub const SQLITE_TESTCTRL_ISINIT: i32 = 23;
pub const SQLITE_TESTCTRL_SORTER_MMAP: i32 = 24;
pub const SQLITE_TESTCTRL_IMPOSTER: i32 = 25;
pub const SQLITE_TESTCTRL_LAST: i32 = 25;
pub const SQLITE_STATUS_MEMORY_USED: i32 = 0;
pub const SQLITE_STATUS_PAGECACHE_USED: i32 = 1;
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: i32 = 2;
pub const SQLITE_STATUS_SCRATCH_USED: i32 = 3;
pub const SQLITE_STATUS_SCRATCH_OVERFLOW: i32 = 4;
pub const SQLITE_STATUS_MALLOC_SIZE: i32 = 5;
pub const SQLITE_STATUS_PARSER_STACK: i32 = 6;
pub const SQLITE_STATUS_PAGECACHE_SIZE: i32 = 7;
pub const SQLITE_STATUS_SCRATCH_SIZE: i32 = 8;
pub const SQLITE_STATUS_MALLOC_COUNT: i32 = 9;
pub const SQLITE_DBSTATUS_LOOKASIDE_USED: i32 = 0;
pub const SQLITE_DBSTATUS_CACHE_USED: i32 = 1;
pub const SQLITE_DBSTATUS_SCHEMA_USED: i32 = 2;
pub const SQLITE_DBSTATUS_STMT_USED: i32 = 3;
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: i32 = 4;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: i32 = 5;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: i32 = 6;
pub const SQLITE_DBSTATUS_CACHE_HIT: i32 = 7;
pub const SQLITE_DBSTATUS_CACHE_MISS: i32 = 8;
pub const SQLITE_DBSTATUS_CACHE_WRITE: i32 = 9;
pub const SQLITE_DBSTATUS_DEFERRED_FKS: i32 = 10;
pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: i32 = 11;
pub const SQLITE_DBSTATUS_MAX: i32 = 11;
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: i32 = 1;
pub const SQLITE_STMTSTATUS_SORT: i32 = 2;
pub const SQLITE_STMTSTATUS_AUTOINDEX: i32 = 3;
pub const SQLITE_STMTSTATUS_VM_STEP: i32 = 4;
pub const SQLITE_CHECKPOINT_PASSIVE: i32 = 0;
pub const SQLITE_CHECKPOINT_FULL: i32 = 1;
pub const SQLITE_CHECKPOINT_RESTART: i32 = 2;
pub const SQLITE_CHECKPOINT_TRUNCATE: i32 = 3;
pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: i32 = 1;
pub const SQLITE_ROLLBACK: i32 = 1;
pub const SQLITE_FAIL: i32 = 3;
pub const SQLITE_REPLACE: i32 = 5;
pub const SQLITE_SCANSTAT_NLOOP: i32 = 0;
pub const SQLITE_SCANSTAT_NVISIT: i32 = 1;
pub const SQLITE_SCANSTAT_EST: i32 = 2;
pub const SQLITE_SCANSTAT_NAME: i32 = 3;
pub const SQLITE_SCANSTAT_EXPLAIN: i32 = 4;
pub const SQLITE_SCANSTAT_SELECTID: i32 = 5;
pub const NOT_WITHIN: i32 = 0;
pub const PARTLY_WITHIN: i32 = 1;
pub const FULLY_WITHIN: i32 = 2;
pub const __SQLITESESSION_H_: i32 = 1;
pub const SQLITE_CHANGESET_DATA: i32 = 1;
pub const SQLITE_CHANGESET_NOTFOUND: i32 = 2;
pub const SQLITE_CHANGESET_CONFLICT: i32 = 3;
pub const SQLITE_CHANGESET_CONSTRAINT: i32 = 4;
pub const SQLITE_CHANGESET_FOREIGN_KEY: i32 = 5;
pub const SQLITE_CHANGESET_OMIT: i32 = 0;
pub const SQLITE_CHANGESET_REPLACE: i32 = 1;
pub const SQLITE_CHANGESET_ABORT: i32 = 2;
pub const FTS5_TOKENIZE_QUERY: i32 = 1;
pub const FTS5_TOKENIZE_PREFIX: i32 = 2;
pub const FTS5_TOKENIZE_DOCUMENT: i32 = 4;
pub const FTS5_TOKENIZE_AUX: i32 = 8;
pub const FTS5_TOKEN_COLOCATED: i32 = 1;
extern "C" {
    pub static mut sqlite3_version: [::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn sqlite3_libversion() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_sourceid() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_libversion_number() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_compileoption_used(
        zOptName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_compileoption_get(N: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_threadsafe() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3 {
    _unused: [u8; 0],
}
pub type sqlite_int64 = ::std::os::raw::c_longlong;
pub type sqlite_uint64 = ::std::os::raw::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
extern "C" {
    pub fn sqlite3_close(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_close_v2(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
pub type sqlite3_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::os::raw::c_char,
        arg4: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn sqlite3_exec(
        arg1: *mut sqlite3,
        sql: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut ::std::os::raw::c_char,
                arg4: *mut *mut ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        arg2: *mut ::std::os::raw::c_void,
        errmsg: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_io_methods {
    pub iVersion: ::std::os::raw::c_int,
    pub xClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xRead: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: *mut ::std::os::raw::c_void,
            iAmt: ::std::os::raw::c_int,
            iOfst: sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xWrite: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: *const ::std::os::raw::c_void,
            iAmt: ::std::os::raw::c_int,
            iOfst: sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xTruncate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file, size: sqlite3_int64) -> ::std::os::raw::c_int,
    >,
    pub xSync: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFileSize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            pSize: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUnlock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCheckReservedLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            pResOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFileControl: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            op: ::std::os::raw::c_int,
            pArg: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSectorSize: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xDeviceCharacteristics: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_file) -> ::std::os::raw::c_int,
    >,
    pub xShmMap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iPg: ::std::os::raw::c_int,
            pgsz: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xShmLock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            offset: ::std::os::raw::c_int,
            n: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xShmBarrier: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_file)>,
    pub xShmUnmap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            deleteFlag: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iOfst: sqlite3_int64,
            iAmt: ::std::os::raw::c_int,
            pp: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUnfetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_file,
            iOfst: sqlite3_int64,
            p: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_mutex {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_api_routines {
    _unused: [u8; 0],
}
pub type sqlite3_syscall_ptr = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vfs {
    pub iVersion: ::std::os::raw::c_int,
    pub szOsFile: ::std::os::raw::c_int,
    pub mxPathname: ::std::os::raw::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const ::std::os::raw::c_char,
    pub pAppData: *mut ::std::os::raw::c_void,
    pub xOpen: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            arg2: *mut sqlite3_file,
            flags: ::std::os::raw::c_int,
            pOutFlags: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDelete: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            syncDir: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xAccess: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            pResOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFullPathname: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            nOut: ::std::os::raw::c_int,
            zOut: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDlOpen: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zFilename: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub xDlError: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            nByte: ::std::os::raw::c_int,
            zErrMsg: *mut ::std::os::raw::c_char,
        ),
    >,
    pub xDlSym: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: *mut ::std::os::raw::c_void,
            zSymbol: *const ::std::os::raw::c_char,
        ) -> ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: *mut ::std::os::raw::c_void,
                zSymbol: *const ::std::os::raw::c_char,
            ),
        >,
    >,
    pub xDlClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut ::std::os::raw::c_void),
    >,
    pub xRandomness: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            nByte: ::std::os::raw::c_int,
            zOut: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSleep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            microseconds: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCurrentTime: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut f64) -> ::std::os::raw::c_int,
    >,
    pub xGetLastError: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCurrentTimeInt64: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSetSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
            arg2: sqlite3_syscall_ptr,
        ) -> ::std::os::raw::c_int,
    >,
    pub xGetSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
        ) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            zName: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
}
extern "C" {
    pub fn sqlite3_initialize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_shutdown() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_os_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_os_end() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_config(arg1: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_db_config(
        arg1: *mut sqlite3,
        op: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_mem_methods {
    pub xMalloc: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub xFree: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub xRealloc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub xSize: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub xRoundup: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub xInit: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub xShutdown: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub pAppData: *mut ::std::os::raw::c_void,
}
extern "C" {
    pub fn sqlite3_extended_result_codes(
        arg1: *mut sqlite3,
        onoff: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_last_insert_rowid(arg1: *mut sqlite3) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_changes(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_total_changes(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_interrupt(arg1: *mut sqlite3);
}
extern "C" {
    pub fn sqlite3_complete(sql: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_complete16(sql: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_busy_handler(
        arg1: *mut sqlite3,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_busy_timeout(
        arg1: *mut sqlite3,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_get_table(
        db: *mut sqlite3,
        zSql: *const ::std::os::raw::c_char,
        pazResult: *mut *mut *mut ::std::os::raw::c_char,
        pnRow: *mut ::std::os::raw::c_int,
        pnColumn: *mut ::std::os::raw::c_int,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_free_table(result: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn sqlite3_mprintf(arg1: *const ::std::os::raw::c_char, ...)
        -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_snprintf(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
        ...
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_malloc(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_malloc64(arg1: sqlite3_uint64) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_realloc64(
        arg1: *mut ::std::os::raw::c_void,
        arg2: sqlite3_uint64,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn sqlite3_msize(arg1: *mut ::std::os::raw::c_void) -> sqlite3_uint64;
}
extern "C" {
    pub fn sqlite3_memory_used() -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_memory_highwater(resetFlag: ::std::os::raw::c_int) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_randomness(N: ::std::os::raw::c_int, P: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn sqlite3_set_authorizer(
        arg1: *mut sqlite3,
        xAuth: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_char,
                arg4: *const ::std::os::raw::c_char,
                arg5: *const ::std::os::raw::c_char,
                arg6: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        pUserData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_trace(
        arg1: *mut sqlite3,
        xTrace: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
            ),
        >,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_profile(
        arg1: *mut sqlite3,
        xProfile: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
                arg3: sqlite3_uint64,
            ),
        >,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_trace_v2(
        arg1: *mut sqlite3,
        uMask: ::std::os::raw::c_uint,
        xCallback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_uint,
                arg2: *mut ::std::os::raw::c_void,
                arg3: *mut ::std::os::raw::c_void,
                arg4: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        pCtx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_progress_handler(
        arg1: *mut sqlite3,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        arg4: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn sqlite3_open(
        filename: *const ::std::os::raw::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_open16(
        filename: *const ::std::os::raw::c_void,
        ppDb: *mut *mut sqlite3,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_open_v2(
        filename: *const ::std::os::raw::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::std::os::raw::c_int,
        zVfs: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_uri_parameter(
        zFilename: *const ::std::os::raw::c_char,
        zParam: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_uri_boolean(
        zFile: *const ::std::os::raw::c_char,
        zParam: *const ::std::os::raw::c_char,
        bDefault: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_uri_int64(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: sqlite3_int64,
    ) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_errcode(db: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_extended_errcode(db: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_errmsg(arg1: *mut sqlite3) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_errmsg16(arg1: *mut sqlite3) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_errstr(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_stmt {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3_limit(
        arg1: *mut sqlite3,
        id: ::std::os::raw::c_int,
        newVal: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::std::os::raw::c_char,
        nByte: ::std::os::raw::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::std::os::raw::c_char,
        nByte: ::std::os::raw::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_prepare16(
        db: *mut sqlite3,
        zSql: *const ::std::os::raw::c_void,
        nByte: ::std::os::raw::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_prepare16_v2(
        db: *mut sqlite3,
        zSql: *const ::std::os::raw::c_void,
        nByte: ::std::os::raw::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_expanded_sql(pStmt: *mut sqlite3_stmt) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_stmt_busy(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mem {
    _unused: [u8; 0],
}
pub type sqlite3_value = Mem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_context {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3_bind_blob(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_void,
        n: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_blob64(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_void,
        arg4: sqlite3_uint64,
        arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_double(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_int(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_int64(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: sqlite3_int64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_null(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_text(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_text16(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_text64(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
        arg4: sqlite3_uint64,
        arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        encoding: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_value(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: *const sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_zeroblob(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_zeroblob64(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
        arg3: sqlite3_uint64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_parameter_count(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_bind_parameter_name(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_bind_parameter_index(
        arg1: *mut sqlite3_stmt,
        zName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_clear_bindings(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_name(
        arg1: *mut sqlite3_stmt,
        N: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_column_name16(
        arg1: *mut sqlite3_stmt,
        N: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_database_name(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_column_database_name16(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_table_name(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_column_table_name16(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_origin_name(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_column_origin_name16(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_decltype(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_column_decltype16(
        arg1: *mut sqlite3_stmt,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_step(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_blob(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_bytes(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_bytes16(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_double(arg1: *mut sqlite3_stmt, iCol: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn sqlite3_column_int(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_int64(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_column_text(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn sqlite3_column_text16(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_column_type(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_column_value(
        arg1: *mut sqlite3_stmt,
        iCol: ::std::os::raw::c_int,
    ) -> *mut sqlite3_value;
}
extern "C" {
    pub fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::std::os::raw::c_char,
        nArg: ::std::os::raw::c_int,
        eTextRep: ::std::os::raw::c_int,
        pApp: *mut ::std::os::raw::c_void,
        xFunc: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xStep: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_function16(
        db: *mut sqlite3,
        zFunctionName: *const ::std::os::raw::c_void,
        nArg: ::std::os::raw::c_int,
        eTextRep: ::std::os::raw::c_int,
        pApp: *mut ::std::os::raw::c_void,
        xFunc: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xStep: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_function_v2(
        db: *mut sqlite3,
        zFunctionName: *const ::std::os::raw::c_char,
        nArg: ::std::os::raw::c_int,
        eTextRep: ::std::os::raw::c_int,
        pApp: *mut ::std::os::raw::c_void,
        xFunc: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xStep: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_aggregate_count(arg1: *mut sqlite3_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_expired(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_transfer_bindings(
        arg1: *mut sqlite3_stmt,
        arg2: *mut sqlite3_stmt,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_global_recover() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_thread_cleanup();
}
extern "C" {
    pub fn sqlite3_memory_alarm(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: sqlite3_int64,
                arg3: ::std::os::raw::c_int,
            ),
        >,
        arg2: *mut ::std::os::raw::c_void,
        arg3: sqlite3_int64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_blob(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_value_bytes(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_bytes16(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_double(arg1: *mut sqlite3_value) -> f64;
}
extern "C" {
    pub fn sqlite3_value_int(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_int64(arg1: *mut sqlite3_value) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_value_text(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn sqlite3_value_text16(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_value_text16le(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_value_text16be(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_value_type(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_numeric_type(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_value_subtype(arg1: *mut sqlite3_value) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sqlite3_value_dup(arg1: *const sqlite3_value) -> *mut sqlite3_value;
}
extern "C" {
    pub fn sqlite3_value_free(arg1: *mut sqlite3_value);
}
extern "C" {
    pub fn sqlite3_aggregate_context(
        arg1: *mut sqlite3_context,
        nBytes: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_user_data(arg1: *mut sqlite3_context) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_context_db_handle(arg1: *mut sqlite3_context) -> *mut sqlite3;
}
extern "C" {
    pub fn sqlite3_get_auxdata(
        arg1: *mut sqlite3_context,
        N: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_set_auxdata(
        arg1: *mut sqlite3_context,
        N: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
pub type sqlite3_destructor_type =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn sqlite3_result_blob(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_blob64(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: sqlite3_uint64,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_double(arg1: *mut sqlite3_context, arg2: f64);
}
extern "C" {
    pub fn sqlite3_result_error(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn sqlite3_result_error16(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn sqlite3_result_error_toobig(arg1: *mut sqlite3_context);
}
extern "C" {
    pub fn sqlite3_result_error_nomem(arg1: *mut sqlite3_context);
}
extern "C" {
    pub fn sqlite3_result_error_code(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sqlite3_result_int(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sqlite3_result_int64(arg1: *mut sqlite3_context, arg2: sqlite3_int64);
}
extern "C" {
    pub fn sqlite3_result_null(arg1: *mut sqlite3_context);
}
extern "C" {
    pub fn sqlite3_result_text(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_text64(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: sqlite3_uint64,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        encoding: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn sqlite3_result_text16(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_text16le(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_text16be(
        arg1: *mut sqlite3_context,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn sqlite3_result_value(arg1: *mut sqlite3_context, arg2: *mut sqlite3_value);
}
extern "C" {
    pub fn sqlite3_result_zeroblob(arg1: *mut sqlite3_context, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sqlite3_result_zeroblob64(
        arg1: *mut sqlite3_context,
        n: sqlite3_uint64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_result_subtype(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sqlite3_create_collation(
        arg1: *mut sqlite3,
        zName: *const ::std::os::raw::c_char,
        eTextRep: ::std::os::raw::c_int,
        pArg: *mut ::std::os::raw::c_void,
        xCompare: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_void,
                arg4: ::std::os::raw::c_int,
                arg5: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_collation_v2(
        arg1: *mut sqlite3,
        zName: *const ::std::os::raw::c_char,
        eTextRep: ::std::os::raw::c_int,
        pArg: *mut ::std::os::raw::c_void,
        xCompare: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_void,
                arg4: ::std::os::raw::c_int,
                arg5: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_collation16(
        arg1: *mut sqlite3,
        zName: *const ::std::os::raw::c_void,
        eTextRep: ::std::os::raw::c_int,
        pArg: *mut ::std::os::raw::c_void,
        xCompare: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_void,
                arg4: ::std::os::raw::c_int,
                arg5: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_collation_needed(
        arg1: *mut sqlite3,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut sqlite3,
                eTextRep: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_char,
            ),
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_collation_needed16(
        arg1: *mut sqlite3,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut sqlite3,
                eTextRep: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_void,
            ),
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_sleep(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut sqlite3_temp_directory: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut sqlite3_data_directory: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_get_autocommit(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_db_handle(arg1: *mut sqlite3_stmt) -> *mut sqlite3;
}
extern "C" {
    pub fn sqlite3_db_filename(
        db: *mut sqlite3,
        zDbName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sqlite3_db_readonly(
        db: *mut sqlite3,
        zDbName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
}
extern "C" {
    pub fn sqlite3_commit_hook(
        arg1: *mut sqlite3,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_rollback_hook(
        arg1: *mut sqlite3,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg3: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_update_hook(
        arg1: *mut sqlite3,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_char,
                arg4: *const ::std::os::raw::c_char,
                arg5: sqlite3_int64,
            ),
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_enable_shared_cache(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_release_memory(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_db_release_memory(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_soft_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
}
extern "C" {
    pub fn sqlite3_soft_heap_limit(N: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sqlite3_table_column_metadata(
        db: *mut sqlite3,
        zDbName: *const ::std::os::raw::c_char,
        zTableName: *const ::std::os::raw::c_char,
        zColumnName: *const ::std::os::raw::c_char,
        pzDataType: *mut *const ::std::os::raw::c_char,
        pzCollSeq: *mut *const ::std::os::raw::c_char,
        pNotNull: *mut ::std::os::raw::c_int,
        pPrimaryKey: *mut ::std::os::raw::c_int,
        pAutoinc: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_load_extension(
        db: *mut sqlite3,
        zFile: *const ::std::os::raw::c_char,
        zProc: *const ::std::os::raw::c_char,
        pzErrMsg: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_enable_load_extension(
        db: *mut sqlite3,
        onoff: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_auto_extension(
        xEntryPoint: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_cancel_auto_extension(
        xEntryPoint: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_reset_auto_extension();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_module {
    pub iVersion: ::std::os::raw::c_int,
    pub xCreate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3,
            pAux: *mut ::std::os::raw::c_void,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
            ppVTab: *mut *mut sqlite3_vtab,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xConnect: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3,
            pAux: *mut ::std::os::raw::c_void,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
            ppVTab: *mut *mut sqlite3_vtab,
            arg2: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xBestIndex: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: *mut sqlite3_index_info,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDisconnect: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xDestroy: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xOpen: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            ppCursor: *mut *mut sqlite3_vtab_cursor,
        ) -> ::std::os::raw::c_int,
    >,
    pub xClose: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xFilter: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            idxNum: ::std::os::raw::c_int,
            idxStr: *const ::std::os::raw::c_char,
            argc: ::std::os::raw::c_int,
            argv: *mut *mut sqlite3_value,
        ) -> ::std::os::raw::c_int,
    >,
    pub xNext: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xEof: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> ::std::os::raw::c_int,
    >,
    pub xColumn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            arg2: *mut sqlite3_context,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRowid: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab_cursor,
            pRowid: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xUpdate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_vtab,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
            arg4: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xBegin: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xSync: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xCommit: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xRollback: ::std::option::Option<
        unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> ::std::os::raw::c_int,
    >,
    pub xFindFunction: ::std::option::Option<
        unsafe extern "C" fn(
            pVtab: *mut sqlite3_vtab,
            nArg: ::std::os::raw::c_int,
            zName: *const ::std::os::raw::c_char,
            pxFunc: *mut ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: ::std::os::raw::c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            ppArg: *mut *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRename: ::std::option::Option<
        unsafe extern "C" fn(
            pVtab: *mut sqlite3_vtab,
            zNew: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSavepoint: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRelease: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRollbackTo: ::std::option::Option<
        unsafe extern "C" fn(
            pVTab: *mut sqlite3_vtab,
            arg1: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_index_info {
    pub nConstraint: ::std::os::raw::c_int,
    pub aConstraint: *mut sqlite3_index_info_sqlite3_index_constraint,
    pub nOrderBy: ::std::os::raw::c_int,
    pub aOrderBy: *mut sqlite3_index_info_sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_info_sqlite3_index_constraint_usage,
    pub idxNum: ::std::os::raw::c_int,
    pub idxStr: *mut ::std::os::raw::c_char,
    pub needToFreeIdxStr: ::std::os::raw::c_int,
    pub orderByConsumed: ::std::os::raw::c_int,
    pub estimatedCost: f64,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::std::os::raw::c_int,
    pub colUsed: sqlite3_uint64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_constraint {
    pub iColumn: ::std::os::raw::c_int,
    pub op: ::std::os::raw::c_uchar,
    pub usable: ::std::os::raw::c_uchar,
    pub iTermOffset: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_orderby {
    pub iColumn: ::std::os::raw::c_int,
    pub desc: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_index_info_sqlite3_index_constraint_usage {
    pub argvIndex: ::std::os::raw::c_int,
    pub omit: ::std::os::raw::c_uchar,
}
extern "C" {
    pub fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::std::os::raw::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const ::std::os::raw::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::std::os::raw::c_void,
        xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::std::os::raw::c_int,
    pub zErrMsg: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
extern "C" {
    pub fn sqlite3_declare_vtab(
        arg1: *mut sqlite3,
        zSQL: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_overload_function(
        arg1: *mut sqlite3,
        zFuncName: *const ::std::os::raw::c_char,
        nArg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_blob {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3_blob_open(
        arg1: *mut sqlite3,
        zDb: *const ::std::os::raw::c_char,
        zTable: *const ::std::os::raw::c_char,
        zColumn: *const ::std::os::raw::c_char,
        iRow: sqlite3_int64,
        flags: ::std::os::raw::c_int,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_blob_reopen(
        arg1: *mut sqlite3_blob,
        arg2: sqlite3_int64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_blob_close(arg1: *mut sqlite3_blob) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_blob_bytes(arg1: *mut sqlite3_blob) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_blob_read(
        arg1: *mut sqlite3_blob,
        Z: *mut ::std::os::raw::c_void,
        N: ::std::os::raw::c_int,
        iOffset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_blob_write(
        arg1: *mut sqlite3_blob,
        z: *const ::std::os::raw::c_void,
        n: ::std::os::raw::c_int,
        iOffset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_vfs_find(zVfsName: *const ::std::os::raw::c_char) -> *mut sqlite3_vfs;
}
extern "C" {
    pub fn sqlite3_vfs_register(
        arg1: *mut sqlite3_vfs,
        makeDflt: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_vfs_unregister(arg1: *mut sqlite3_vfs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_mutex_alloc(arg1: ::std::os::raw::c_int) -> *mut sqlite3_mutex;
}
extern "C" {
    pub fn sqlite3_mutex_free(arg1: *mut sqlite3_mutex);
}
extern "C" {
    pub fn sqlite3_mutex_enter(arg1: *mut sqlite3_mutex);
}
extern "C" {
    pub fn sqlite3_mutex_try(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_mutex_leave(arg1: *mut sqlite3_mutex);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_mutex_methods {
    pub xMutexInit: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub xMutexEnd: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub xMutexAlloc: ::std::option::Option<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> *mut sqlite3_mutex,
    >,
    pub xMutexFree: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
    pub xMutexEnter: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
    pub xMutexTry: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int,
    >,
    pub xMutexLeave: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
    pub xMutexHeld: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int,
    >,
    pub xMutexNotheld: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int,
    >,
}
extern "C" {
    pub fn sqlite3_mutex_held(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_mutex_notheld(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_db_mutex(arg1: *mut sqlite3) -> *mut sqlite3_mutex;
}
extern "C" {
    pub fn sqlite3_file_control(
        arg1: *mut sqlite3,
        zDbName: *const ::std::os::raw::c_char,
        op: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_test_control(op: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_status(
        op: ::std::os::raw::c_int,
        pCurrent: *mut ::std::os::raw::c_int,
        pHighwater: *mut ::std::os::raw::c_int,
        resetFlag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_status64(
        op: ::std::os::raw::c_int,
        pCurrent: *mut sqlite3_int64,
        pHighwater: *mut sqlite3_int64,
        resetFlag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_db_status(
        arg1: *mut sqlite3,
        op: ::std::os::raw::c_int,
        pCur: *mut ::std::os::raw::c_int,
        pHiwtr: *mut ::std::os::raw::c_int,
        resetFlg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_stmt_status(
        arg1: *mut sqlite3_stmt,
        op: ::std::os::raw::c_int,
        resetFlg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type sqlite3_pcache = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::std::os::raw::c_void,
    pub pExtra: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_pcache_methods2 {
    pub iVersion: ::std::os::raw::c_int,
    pub pArg: *mut ::std::os::raw::c_void,
    pub xInit: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub xShutdown: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub xCreate: ::std::option::Option<
        unsafe extern "C" fn(
            szPage: ::std::os::raw::c_int,
            szExtra: ::std::os::raw::c_int,
            bPurgeable: ::std::os::raw::c_int,
        ) -> *mut sqlite3_pcache,
    >,
    pub xCachesize: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache, nCachesize: ::std::os::raw::c_int),
    >,
    pub xPagecount: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache) -> ::std::os::raw::c_int,
    >,
    pub xFetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            key: ::std::os::raw::c_uint,
            createFlag: ::std::os::raw::c_int,
        ) -> *mut sqlite3_pcache_page,
    >,
    pub xUnpin: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            arg2: *mut sqlite3_pcache_page,
            discard: ::std::os::raw::c_int,
        ),
    >,
    pub xRekey: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            arg2: *mut sqlite3_pcache_page,
            oldKey: ::std::os::raw::c_uint,
            newKey: ::std::os::raw::c_uint,
        ),
    >,
    pub xTruncate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache, iLimit: ::std::os::raw::c_uint),
    >,
    pub xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
    pub xShrink: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_pcache_methods {
    pub pArg: *mut ::std::os::raw::c_void,
    pub xInit: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub xShutdown: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub xCreate: ::std::option::Option<
        unsafe extern "C" fn(
            szPage: ::std::os::raw::c_int,
            bPurgeable: ::std::os::raw::c_int,
        ) -> *mut sqlite3_pcache,
    >,
    pub xCachesize: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache, nCachesize: ::std::os::raw::c_int),
    >,
    pub xPagecount: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache) -> ::std::os::raw::c_int,
    >,
    pub xFetch: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            key: ::std::os::raw::c_uint,
            createFlag: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub xUnpin: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            arg2: *mut ::std::os::raw::c_void,
            discard: ::std::os::raw::c_int,
        ),
    >,
    pub xRekey: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_pcache,
            arg2: *mut ::std::os::raw::c_void,
            oldKey: ::std::os::raw::c_uint,
            newKey: ::std::os::raw::c_uint,
        ),
    >,
    pub xTruncate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut sqlite3_pcache, iLimit: ::std::os::raw::c_uint),
    >,
    pub xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_backup {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3_backup_init(
        pDest: *mut sqlite3,
        zDestName: *const ::std::os::raw::c_char,
        pSource: *mut sqlite3,
        zSourceName: *const ::std::os::raw::c_char,
    ) -> *mut sqlite3_backup;
}
extern "C" {
    pub fn sqlite3_backup_step(
        p: *mut sqlite3_backup,
        nPage: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_unlock_notify(
        pBlocked: *mut sqlite3,
        xNotify: ::std::option::Option<
            unsafe extern "C" fn(
                apArg: *mut *mut ::std::os::raw::c_void,
                nArg: ::std::os::raw::c_int,
            ),
        >,
        pNotifyArg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_stricmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_strnicmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_strglob(
        zGlob: *const ::std::os::raw::c_char,
        zStr: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_strlike(
        zGlob: *const ::std::os::raw::c_char,
        zStr: *const ::std::os::raw::c_char,
        cEsc: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_log(
        iErrCode: ::std::os::raw::c_int,
        zFormat: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn sqlite3_wal_hook(
        arg1: *mut sqlite3,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut sqlite3,
                arg3: *const ::std::os::raw::c_char,
                arg4: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_wal_autocheckpoint(
        db: *mut sqlite3,
        N: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_wal_checkpoint(
        db: *mut sqlite3,
        zDb: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_wal_checkpoint_v2(
        db: *mut sqlite3,
        zDb: *const ::std::os::raw::c_char,
        eMode: ::std::os::raw::c_int,
        pnLog: *mut ::std::os::raw::c_int,
        pnCkpt: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_vtab_config(
        arg1: *mut sqlite3,
        op: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_vtab_on_conflict(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_stmt_scanstatus(
        pStmt: *mut sqlite3_stmt,
        idx: ::std::os::raw::c_int,
        iScanStatusOp: ::std::os::raw::c_int,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_stmt_scanstatus_reset(arg1: *mut sqlite3_stmt);
}
extern "C" {
    pub fn sqlite3_db_cacheflush(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_preupdate_hook(
        db: *mut sqlite3,
        xPreUpdate: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                db: *mut sqlite3,
                op: ::std::os::raw::c_int,
                zDb: *const ::std::os::raw::c_char,
                zName: *const ::std::os::raw::c_char,
                iKey1: sqlite3_int64,
                iKey2: sqlite3_int64,
            ),
        >,
        arg1: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sqlite3_preupdate_old(
        arg1: *mut sqlite3,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_preupdate_count(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_preupdate_depth(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_preupdate_new(
        arg1: *mut sqlite3,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_system_errno(arg1: *mut sqlite3) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_snapshot {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3_snapshot_get(
        db: *mut sqlite3,
        zSchema: *const ::std::os::raw::c_char,
        ppSnapshot: *mut *mut sqlite3_snapshot,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_snapshot_open(
        db: *mut sqlite3,
        zSchema: *const ::std::os::raw::c_char,
        pSnapshot: *mut sqlite3_snapshot,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3_snapshot_free(arg1: *mut sqlite3_snapshot);
}
extern "C" {
    pub fn sqlite3_snapshot_cmp(
        p1: *mut sqlite3_snapshot,
        p2: *mut sqlite3_snapshot,
    ) -> ::std::os::raw::c_int;
}
pub type sqlite3_rtree_dbl = f64;
extern "C" {
    pub fn sqlite3_rtree_geometry_callback(
        db: *mut sqlite3,
        zGeom: *const ::std::os::raw::c_char,
        xGeom: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_rtree_geometry,
                arg2: ::std::os::raw::c_int,
                arg3: *mut sqlite3_rtree_dbl,
                arg4: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pContext: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_rtree_geometry {
    pub pContext: *mut ::std::os::raw::c_void,
    pub nParam: ::std::os::raw::c_int,
    pub aParam: *mut sqlite3_rtree_dbl,
    pub pUser: *mut ::std::os::raw::c_void,
    pub xDelUser: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
extern "C" {
    pub fn sqlite3_rtree_query_callback(
        db: *mut sqlite3,
        zQueryFunc: *const ::std::os::raw::c_char,
        xQueryFunc: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_rtree_query_info) -> ::std::os::raw::c_int,
        >,
        pContext: *mut ::std::os::raw::c_void,
        xDestructor: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_rtree_query_info {
    pub pContext: *mut ::std::os::raw::c_void,
    pub nParam: ::std::os::raw::c_int,
    pub aParam: *mut sqlite3_rtree_dbl,
    pub pUser: *mut ::std::os::raw::c_void,
    pub xDelUser: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub aCoord: *mut sqlite3_rtree_dbl,
    pub anQueue: *mut ::std::os::raw::c_uint,
    pub nCoord: ::std::os::raw::c_int,
    pub iLevel: ::std::os::raw::c_int,
    pub mxLevel: ::std::os::raw::c_int,
    pub iRowid: sqlite3_int64,
    pub rParentScore: sqlite3_rtree_dbl,
    pub eParentWithin: ::std::os::raw::c_int,
    pub eWithin: ::std::os::raw::c_int,
    pub rScore: sqlite3_rtree_dbl,
    pub apSqlParam: *mut *mut sqlite3_value,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_session {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_changeset_iter {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3session_create(
        db: *mut sqlite3,
        zDb: *const ::std::os::raw::c_char,
        ppSession: *mut *mut sqlite3_session,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_delete(pSession: *mut sqlite3_session);
}
extern "C" {
    pub fn sqlite3session_enable(
        pSession: *mut sqlite3_session,
        bEnable: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_indirect(
        pSession: *mut sqlite3_session,
        bIndirect: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_attach(
        pSession: *mut sqlite3_session,
        zTab: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_table_filter(
        pSession: *mut sqlite3_session,
        xFilter: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                zTab: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        pCtx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn sqlite3session_changeset(
        pSession: *mut sqlite3_session,
        pnChangeset: *mut ::std::os::raw::c_int,
        ppChangeset: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_diff(
        pSession: *mut sqlite3_session,
        zFromDb: *const ::std::os::raw::c_char,
        zTbl: *const ::std::os::raw::c_char,
        pzErrMsg: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_patchset(
        pSession: *mut sqlite3_session,
        pnPatchset: *mut ::std::os::raw::c_int,
        ppPatchset: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_isempty(pSession: *mut sqlite3_session) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_start(
        pp: *mut *mut sqlite3_changeset_iter,
        nChangeset: ::std::os::raw::c_int,
        pChangeset: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_next(pIter: *mut sqlite3_changeset_iter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_op(
        pIter: *mut sqlite3_changeset_iter,
        pzTab: *mut *const ::std::os::raw::c_char,
        pnCol: *mut ::std::os::raw::c_int,
        pOp: *mut ::std::os::raw::c_int,
        pbIndirect: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_pk(
        pIter: *mut sqlite3_changeset_iter,
        pabPK: *mut *mut ::std::os::raw::c_uchar,
        pnCol: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_old(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::std::os::raw::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_new(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::std::os::raw::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_conflict(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::std::os::raw::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_fk_conflicts(
        pIter: *mut sqlite3_changeset_iter,
        pnOut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_finalize(pIter: *mut sqlite3_changeset_iter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_invert(
        nIn: ::std::os::raw::c_int,
        pIn: *const ::std::os::raw::c_void,
        pnOut: *mut ::std::os::raw::c_int,
        ppOut: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_concat(
        nA: ::std::os::raw::c_int,
        pA: *mut ::std::os::raw::c_void,
        nB: ::std::os::raw::c_int,
        pB: *mut ::std::os::raw::c_void,
        pnOut: *mut ::std::os::raw::c_int,
        ppOut: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_changegroup {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sqlite3changegroup_new(pp: *mut *mut sqlite3_changegroup) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changegroup_add(
        arg1: *mut sqlite3_changegroup,
        nData: ::std::os::raw::c_int,
        pData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changegroup_output(
        arg1: *mut sqlite3_changegroup,
        pnData: *mut ::std::os::raw::c_int,
        ppData: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changegroup_delete(arg1: *mut sqlite3_changegroup);
}
extern "C" {
    pub fn sqlite3changeset_apply(
        db: *mut sqlite3,
        nChangeset: ::std::os::raw::c_int,
        pChangeset: *mut ::std::os::raw::c_void,
        xFilter: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                zTab: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        xConflict: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                eConflict: ::std::os::raw::c_int,
                p: *mut sqlite3_changeset_iter,
            ) -> ::std::os::raw::c_int,
        >,
        pCtx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_apply_strm(
        db: *mut sqlite3,
        xInput: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pIn: *mut ::std::os::raw::c_void,
        xFilter: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                zTab: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        xConflict: ::std::option::Option<
            unsafe extern "C" fn(
                pCtx: *mut ::std::os::raw::c_void,
                eConflict: ::std::os::raw::c_int,
                p: *mut sqlite3_changeset_iter,
            ) -> ::std::os::raw::c_int,
        >,
        pCtx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_concat_strm(
        xInputA: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pInA: *mut ::std::os::raw::c_void,
        xInputB: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pInB: *mut ::std::os::raw::c_void,
        xOutput: ::std::option::Option<
            unsafe extern "C" fn(
                pOut: *mut ::std::os::raw::c_void,
                pData: *const ::std::os::raw::c_void,
                nData: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_invert_strm(
        xInput: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pIn: *mut ::std::os::raw::c_void,
        xOutput: ::std::option::Option<
            unsafe extern "C" fn(
                pOut: *mut ::std::os::raw::c_void,
                pData: *const ::std::os::raw::c_void,
                nData: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changeset_start_strm(
        pp: *mut *mut sqlite3_changeset_iter,
        xInput: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pIn: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_changeset_strm(
        pSession: *mut sqlite3_session,
        xOutput: ::std::option::Option<
            unsafe extern "C" fn(
                pOut: *mut ::std::os::raw::c_void,
                pData: *const ::std::os::raw::c_void,
                nData: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3session_patchset_strm(
        pSession: *mut sqlite3_session,
        xOutput: ::std::option::Option<
            unsafe extern "C" fn(
                pOut: *mut ::std::os::raw::c_void,
                pData: *const ::std::os::raw::c_void,
                nData: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changegroup_add_strm(
        arg1: *mut sqlite3_changegroup,
        xInput: ::std::option::Option<
            unsafe extern "C" fn(
                pIn: *mut ::std::os::raw::c_void,
                pData: *mut ::std::os::raw::c_void,
                pnData: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pIn: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sqlite3changegroup_output_strm(
        arg1: *mut sqlite3_changegroup,
        xOutput: ::std::option::Option<
            unsafe extern "C" fn(
                pOut: *mut ::std::os::raw::c_void,
                pData: *const ::std::os::raw::c_void,
                nData: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pOut: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fts5Context {
    _unused: [u8; 0],
}
pub type fts5_extension_function = ::std::option::Option<
    unsafe extern "C" fn(
        pApi: *const Fts5ExtensionApi,
        pFts: *mut Fts5Context,
        pCtx: *mut sqlite3_context,
        nVal: ::std::os::raw::c_int,
        apVal: *mut *mut sqlite3_value,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fts5PhraseIter {
    pub a: *const ::std::os::raw::c_uchar,
    pub b: *const ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fts5ExtensionApi {
    pub iVersion: ::std::os::raw::c_int,
    pub xUserData: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut Fts5Context) -> *mut ::std::os::raw::c_void,
    >,
    pub xColumnCount: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut Fts5Context) -> ::std::os::raw::c_int,
    >,
    pub xRowCount: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            pnRow: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xColumnTotalSize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iCol: ::std::os::raw::c_int,
            pnToken: *mut sqlite3_int64,
        ) -> ::std::os::raw::c_int,
    >,
    pub xTokenize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            pText: *const ::std::os::raw::c_char,
            nText: ::std::os::raw::c_int,
            pCtx: *mut ::std::os::raw::c_void,
            xToken: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: ::std::os::raw::c_int,
                    arg3: *const ::std::os::raw::c_char,
                    arg4: ::std::os::raw::c_int,
                    arg5: ::std::os::raw::c_int,
                    arg6: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> ::std::os::raw::c_int,
    >,
    pub xPhraseCount: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut Fts5Context) -> ::std::os::raw::c_int,
    >,
    pub xPhraseSize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iPhrase: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xInstCount: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            pnInst: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xInst: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iIdx: ::std::os::raw::c_int,
            piPhrase: *mut ::std::os::raw::c_int,
            piCol: *mut ::std::os::raw::c_int,
            piOff: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xRowid:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Fts5Context) -> sqlite3_int64>,
    pub xColumnText: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iCol: ::std::os::raw::c_int,
            pz: *mut *const ::std::os::raw::c_char,
            pn: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xColumnSize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iCol: ::std::os::raw::c_int,
            pnToken: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xQueryPhrase: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iPhrase: ::std::os::raw::c_int,
            pUserData: *mut ::std::os::raw::c_void,
            arg2: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *const Fts5ExtensionApi,
                    arg2: *mut Fts5Context,
                    arg3: *mut ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> ::std::os::raw::c_int,
    >,
    pub xSetAuxdata: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            pAux: *mut ::std::os::raw::c_void,
            xDelete: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        ) -> ::std::os::raw::c_int,
    >,
    pub xGetAuxdata: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            bClear: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub xPhraseFirst: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iPhrase: ::std::os::raw::c_int,
            arg2: *mut Fts5PhraseIter,
            arg3: *mut ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xPhraseNext: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            arg2: *mut Fts5PhraseIter,
            piCol: *mut ::std::os::raw::c_int,
            piOff: *mut ::std::os::raw::c_int,
        ),
    >,
    pub xPhraseFirstColumn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            iPhrase: ::std::os::raw::c_int,
            arg2: *mut Fts5PhraseIter,
            arg3: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub xPhraseNextColumn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Context,
            arg2: *mut Fts5PhraseIter,
            piCol: *mut ::std::os::raw::c_int,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fts5Tokenizer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fts5_tokenizer {
    pub xCreate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            azArg: *mut *const ::std::os::raw::c_char,
            nArg: ::std::os::raw::c_int,
            ppOut: *mut *mut Fts5Tokenizer,
        ) -> ::std::os::raw::c_int,
    >,
    pub xDelete: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Fts5Tokenizer)>,
    pub xTokenize: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Fts5Tokenizer,
            pCtx: *mut ::std::os::raw::c_void,
            flags: ::std::os::raw::c_int,
            pText: *const ::std::os::raw::c_char,
            nText: ::std::os::raw::c_int,
            xToken: ::std::option::Option<
                unsafe extern "C" fn(
                    pCtx: *mut ::std::os::raw::c_void,
                    tflags: ::std::os::raw::c_int,
                    pToken: *const ::std::os::raw::c_char,
                    nToken: ::std::os::raw::c_int,
                    iStart: ::std::os::raw::c_int,
                    iEnd: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fts5_api {
    pub iVersion: ::std::os::raw::c_int,
    pub xCreateTokenizer: ::std::option::Option<
        unsafe extern "C" fn(
            pApi: *mut fts5_api,
            zName: *const ::std::os::raw::c_char,
            pContext: *mut ::std::os::raw::c_void,
            pTokenizer: *mut fts5_tokenizer,
            xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        ) -> ::std::os::raw::c_int,
    >,
    pub xFindTokenizer: ::std::option::Option<
        unsafe extern "C" fn(
            pApi: *mut fts5_api,
            zName: *const ::std::os::raw::c_char,
            ppContext: *mut *mut ::std::os::raw::c_void,
            pTokenizer: *mut fts5_tokenizer,
        ) -> ::std::os::raw::c_int,
    >,
    pub xCreateFunction: ::std::option::Option<
        unsafe extern "C" fn(
            pApi: *mut fts5_api,
            zName: *const ::std::os::raw::c_char,
            pContext: *mut ::std::os::raw::c_void,
            xFunction: fts5_extension_function,
            xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        ) -> ::std::os::raw::c_int,
    >,
}
