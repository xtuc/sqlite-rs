//! https://www.sqlite.org/c3ref/pcache_methods2.html

// This is a workaround for rusqlite/rusqlite missing support for
// `sqlite3_pcache_methods2` and/or not publishing on crates.io
mod ffi_bindgen;

use ffi_bindgen as ffi;
use std::collections::HashMap;
use std::os::raw::{c_int, c_uint, c_void};
use std::ptr;

pub enum InitState {
    Ok,
    UseDefaultCachePage,
}

type BoxError = Box<dyn std::error::Error>;

pub fn make_page<'a>(page_size: usize, extra_size: usize) -> Page {
    let page = vec![0u8; page_size].into_boxed_slice();
    let extra = vec![0u8; extra_size].into_boxed_slice();
    Page {
        pBuf: Box::into_raw(page) as *mut c_void,
        pExtra: Box::into_raw(extra) as *mut c_void,
    }
}

#[derive(PartialEq)]
pub enum CreateFlag {
    /// Do not allocate a new page. Return NULL.
    NoAllocation,

    /// Allocate a new page if it easy and convenient to do so. Otherwise return NULL.
    AllocateIfConvenient,

    /// Make every effort to allocate a new page. Only return NULL if allocating a new page is
    /// effectively impossible.
    Allocate,
}

#[derive(Debug)]
pub enum DiscardStrategy {
    /// The page must be evicted from the cache
    MustBeEvicted,

    /// Page may be discarded or retained at the discretion of page cache implementation
    CanDecide,
}

pub trait PageCacheBuiler<'a, T: PageCache<'a>> {
    /// SQLite invokes the `create` method to construct a new cache instance.
    /// SQLite will typically create one cache instance for each open database file, though this is
    /// not guaranteed.  The first parameter, `page_size`, is the size in bytes of the pages that
    /// must be allocated by the cache. `page_size` will always a power of two.  The second
    /// parameter `extra_size` is a number of bytes of extra storage associated with each page
    /// cache entry.  The `extra_size` parameter will a number less than 250. SQLite will use the
    /// extra extra bytes on each page to store metadata about the underlying database page on
    /// disk. The value passed depends on the SQLite version, the target platform, and how SQLite
    /// was compiled.
    /// The third argument to `create`, `bpurgeable`, is true if the cache being
    /// created will be used to cache database pages of a file stored on disk, or false if it is
    /// used for an in-memory database. The cache implementation does not have to do anything
    /// special based with the value of bPurgeable; it is purely advisory. On a cache where
    /// bPurgeable is false, SQLite will never invoke [unpin] except to deliberately delete a page.
    /// In other words, calls to [unpin] on a cache with bPurgeable set to false will always have
    /// the "discard" flag set to true. Hence, a cache created with bPurgeable false will never
    /// contain any unpinned pages.
    fn create(page_size: usize, extra_size: usize, bpurgeable: bool) -> T;
}

pub use ffi::sqlite3_pcache_page as Page;

pub trait PageCache<'a> {
    /// The `cache_size` method may be called at any time by SQLite to set the suggested maximum
    /// cache-size (number of pages stored by) the cache instance passed as the first argument.
    /// This is the value configured using the SQLite "PRAGMA cache_size" command. It
    /// is advisory only.
    fn cache_size(&mut self, cache_size: usize);

    /// The `page_count` method must return the number of pages currently stored in the cache, both
    /// pinned and unpinned.
    fn page_count(&mut self) -> usize;

    /// The `fetch` method locates a page in the cache or None (see [CreateFlag] for detail on cache
    /// miss).
    /// The page to be fetched is determined by the `key`. The minimum key value is 1. After it has
    /// been retrieved using `fetch`, the page is considered to be "pinned".
    ///
    /// SQLite will normally invoke `fetch` with a createFlag of NoAllocation or
    /// AllocateIfConvenient. SQLite will only use a createFlag of Allocate after a prior call with
    /// a createFlag of AllocateIfConvenient failed. In between the `fetch` calls, SQLite may
    /// attempt to unpin one or more cache pages by spilling the content of pinned pages to disk
    /// and synching the operating system disk cache.
    fn fetch(&'a mut self, key: usize, create_flag: CreateFlag) -> Option<&'a mut Page>;

    /// `unpin` is called by SQLite with a pointer to a currently pinned page.
    /// The page cache implementation may choose to evict unpinned pages at any time.
    fn unpin(&mut self, key: usize, discard: DiscardStrategy);

    /// The `rekey` method is used to change the key value associated with the page passed as the
    /// second argument. If the cache previously contains an entry associated with `new_key`, it must
    /// be discarded. Any prior cache entry associated with `new_key` is guaranteed not to be pinned.
    fn rekey(&mut self, old_key: usize, new_key: usize);

    /// When SQLite calls the `truncate` method, the cache must discard all existing cache entries
    /// with page numbers (keys) greater than or equal to the value of the `limit` parameter passed
    /// to `truncate`. If any of these pages are pinned, they are implicitly unpinned, meaning
    /// that they can be safely discarded.
    fn truncate(&mut self, limit: usize);

    /// The `destroy` method is used to delete a cache allocated by `create`. All resources
    /// associated with the specified cache should be freed.
    fn destroy(&mut self);

    /// SQLite invokes the `shrink` method when it wants the page cache to free up as much of heap
    /// memory as possible. The page cache implementation is not obligated to free any memory, but
    /// well-behaved implementations should do their best.
    fn shrink(&mut self);
}

pub fn build<'a, B: PageCacheBuiler<'a, T>, T: PageCache<'a> + 'a>(
) -> *mut ffi::sqlite3_pcache_methods2 {
    Box::into_raw(Box::new(ffi::sqlite3_pcache_methods2 {
        iVersion: 1,
        pArg: ptr::null_mut(),
        xInit: Some(pcache::init),
        xShutdown: Some(pcache::shutdown),
        xCreate: Some(pcache::create::<'a, B, T>),
        xCachesize: Some(pcache::cache_size::<'a, T>),
        xPagecount: Some(pcache::page_count::<'a, T>),
        xFetch: Some(pcache::fetch::<'a, T>),
        xUnpin: Some(pcache::unpin::<'a, T>),
        xRekey: Some(pcache::rekey::<'a, T>),
        xTruncate: Some(pcache::truncate::<'a, T>),
        xDestroy: Some(pcache::destroy::<'a, T>),
        xShrink: Some(pcache::shrink::<'a, T>),
    }))
}

pub fn register(pcache: *mut ffi::sqlite3_pcache_methods2) -> Result<(), BoxError> {
    log::trace!("register SQLITE_CONFIG_PCACHE2");
    let ret = unsafe { ffi::sqlite3_config(ffi::SQLITE_CONFIG_PCACHE2, pcache) };
    if ret != ffi::SQLITE_OK {
        Err(format!("sqlite3_config returned code: {}", ret).into())
    } else {
        Ok(())
    }
}

struct Context<T> {
    start_canary: u64,

    inner: T,
    page_ptr_to_keys: HashMap<*const c_void, usize>,

    end_canary: u64,
}

use std::fmt;

impl<T> fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("start_canary", &self.start_canary)
            .field("page_ptr_to_keys", &self.page_ptr_to_keys)
            .field("end_canary", &self.end_canary)
            .finish()
    }
}

impl<T> Drop for Context<T> {
    fn drop(&mut self) {
        panic!("nope")
    }
}

mod pcache {
    use super::*;

    fn get_ctx<'b, T>(ptr: *mut ffi::sqlite3_pcache) -> &'b mut Context<T> {
        let ctx = unsafe { &mut *(ptr as *mut Context<T>) };
        log::trace!("ctx {:?}", ctx);
        assert_eq!(ctx.start_canary, 0xaaaabbbbccccdddd);
        assert_eq!(ctx.end_canary, 0x1111222233334444);
        ctx
    }

    pub(super) extern "C" fn init(_arg1: *mut c_void) -> c_int {
        ffi::SQLITE_OK
    }
    pub(super) extern "C" fn shutdown(_arg1: *mut c_void) {}

    pub(super) extern "C" fn create<'a, Builder: PageCacheBuiler<'a, T>, T: PageCache<'a>>(
        page_size: c_int,
        extra_size: c_int,
        bpurgeable: c_int,
    ) -> *mut ffi::sqlite3_pcache {
        let bpurgeable = if bpurgeable == 1 { true } else { false };
        let pcache = Builder::create(page_size as usize, extra_size as usize, bpurgeable);
        let ctx = Box::new(Context {
            start_canary: 0xaaaabbbbccccdddd,
            inner: pcache,
            page_ptr_to_keys: HashMap::new(),
            end_canary: 0x1111222233334444,
        });

        let ctx = Box::into_raw(ctx) as *mut ffi::sqlite3_pcache;
        log::trace!("create ctx={:?}", ctx);
        ctx
    }

    pub(super) extern "C" fn cache_size<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
        n_cache_size: c_int,
    ) {
        log::trace!("cache_size arg1={:?} n_cache_size={}", arg1, n_cache_size);
        let ctx = get_ctx::<T>(arg1);
        ctx.inner.cache_size(n_cache_size as usize);
    }

    pub(super) extern "C" fn page_count<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
    ) -> c_int {
        log::trace!("page_count arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);
        ctx.inner.page_count() as c_int
    }

    pub(super) extern "C" fn fetch<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
        key: c_uint,
        create_flag: c_int,
    ) -> *mut Page {
        log::trace!("fetch arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);
        let create_flag = match create_flag {
            0 => CreateFlag::NoAllocation,
            1 => CreateFlag::AllocateIfConvenient,
            2 => CreateFlag::Allocate,
            v => panic!("unknown create_flag: {}", v),
        };
        match ctx.inner.fetch(key as usize, create_flag) {
            None => ptr::null_mut(),
            Some(page) => {
                let addr = page as *mut Page as *const c_void;
                log::trace!("[page_ptr_to_keys] fetch addr={:?} key={}", addr, key);
                ctx.page_ptr_to_keys.insert(addr, key as usize);

                log::trace!("page: {:?}", page);
                page as *mut Page
            }
        }
    }

    pub(super) extern "C" fn unpin<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
        arg2: *mut ffi::sqlite3_pcache_page,
        discard: c_int,
    ) {
        log::trace!("unpin arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);

        let addr = arg2 as *const c_void;
        log::trace!("[page_ptr_to_keys] unpin addr={:?}", addr);
        if let Some(key) = ctx.page_ptr_to_keys.get(&addr) {
            log::trace!("[page_ptr_to_keys] unpin key={}", key);

            let discard = match discard {
                0 => DiscardStrategy::CanDecide,
                _ => DiscardStrategy::MustBeEvicted,
            };

            ctx.inner.unpin(*key, discard);
        } else {
            log::trace!("[page_ptr_to_keys] unpin key not found");
        }
    }

    pub(super) extern "C" fn rekey<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
        _arg2: *mut ffi::sqlite3_pcache_page,
        old_key: c_uint,
        new_key: c_uint,
    ) {
        log::trace!("rekey arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);
        ctx.inner.rekey(old_key as usize, new_key as usize);
    }

    pub(super) extern "C" fn truncate<'a, T: PageCache<'a> + 'a>(
        arg1: *mut ffi::sqlite3_pcache,
        i_limit: c_uint,
    ) {
        log::trace!("truncate arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);
        ctx.inner.truncate(i_limit as usize);
    }

    pub(super) extern "C" fn destroy<'a, T: PageCache<'a> + 'a>(arg1: *mut ffi::sqlite3_pcache) {
        log::trace!("destroy arg1={:?}", arg1);

        let ctx = get_ctx::<T>(arg1);
        ctx.inner.destroy();

        unsafe {
            let _ = Box::from_raw(arg1);
        }
    }

    pub(super) extern "C" fn shrink<'a, T: PageCache<'a> + 'a>(arg1: *mut ffi::sqlite3_pcache) {
        log::trace!("shink arg1={:?}", arg1);
        let ctx = get_ctx::<T>(arg1);
        ctx.inner.shrink();
    }
}
