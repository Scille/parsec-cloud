// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

//! Zero-copy borrowed view over a JS `Uint8Array`/`Buffer` backing store.

use neon::{
    context::Context,
    handle::{Handle, Root},
    prelude::*,
    types::{JsTypedArray, buffer::TypedArray},
};

/// A borrowed, zero-copy view of the bytes of a JS `Uint8Array`/`Buffer`.
///
/// Unlike `TypedArray::as_slice(...).to_vec()`, no data is copied: `as_slice()`
/// points directly into the JS `ArrayBuffer` backing store. The inner `Root`
/// keeps the JS object pinned against garbage collection for as long as the
/// view lives, so the view is `Send` and can be moved into an async task.
///
/// # Safety contract (by convention, not enforced by the type system)
///
/// While a `JsBytesView` is alive, the JS side must not **detach** the
/// underlying `ArrayBuffer` (e.g. `structuredClone(buf, {transfer: [buf]})`
/// or `buf.transfer()`). Detaching frees the backing store and would turn
/// the borrowed slice into a dangling pointer. In practice: don't touch (let
/// alone transfer) the `Uint8Array` you passed to a binding until the
/// returned promise settles.
pub struct JsBytesView {
    // Never read: the `Root` exists only to pin the JS object against GC
    // until this view (and hence the borrow over its backing store) is dropped.
    #[allow(dead_code)]
    root: Root<JsTypedArray<u8>>,
    data: &'static [u8],
}

impl JsBytesView {
    pub fn new<'a, C: Context<'a>>(
        cx: &mut C,
        js_val: Handle<'a, JsTypedArray<u8>>,
    ) -> NeonResult<Self> {
        let root = js_val.root(cx);
        let slice = js_val.as_slice(cx);
        // SAFETY:
        // - `root` pins the JS typed array (and its `ArrayBuffer`) against GC
        //   for the whole lifetime of the view, so the backing store cannot be
        //   freed by the collector.
        // - V8 allocates `ArrayBuffer` backing stores off-heap and never moves
        //   them, so the pointer stays valid as long as the buffer is alive.
        // - The caller (generated code) must uphold the struct-level contract:
        //   the JS side must not detach the buffer while the view is alive.
        // - `slice` is a fresh borrow of `js_val` whose data pointer we re-borrow
        //   with an unbounded lifetime; the original borrow is not used again.
        let data = unsafe { std::slice::from_raw_parts(slice.as_ptr(), slice.len()) };
        Ok(Self { root, data })
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
}
