use std::os::raw::{c_void};
use std::mem::MaybeUninit;
use std::ptr;
use std::alloc::{alloc, Layout};

#[repr(C)]
pub struct queue_entry {
    next: Option<Box<queue_entry>>,
    data: *mut c_void,
}

#[repr(C)]
pub struct queue_head {
    head: Option<Box<queue_entry>>,
    tail: Option<Box<queue_entry>>,
    num: u32,
}

#[no_mangle]
pub extern "C" fn queue_init (queue: &mut queue_head) {
    queue.head = None;
    queue.tail = None;
    queue.num = 0;
}

#[no_mangle]
pub extern "C" fn create_queue_head() -> *mut queue_head {
    let head = Box::new(MaybeUninit::<queue_head>::uninit());
    Box::into_raw(head) as *mut queue_head
}

#[no_mangle]
pub extern "C" fn queue_push(queue: *mut queue_head, data: *mut c_void) -> *mut c_void {
    
    unsafe {
        
        if queue.is_null() {
            return ptr::null_mut();
        }
        
        let entry_layout = Layout::new::<queue_entry>();
        let entry = alloc(entry_layout) as *mut queue_entry;
        if entry.is_null() {
            return ptr::null_mut();
        }
        
        // (*entry).next = None;
        (*entry).data = data;
        
        if (*queue).tail.is_some() {
            (*queue).tail = Some(Box::from_raw(entry));
        } else {
            (*queue).head = Some(Box::from_raw(entry));
            (*queue).tail = Some(Box::from_raw(entry));
        }

        (*queue).num += 1;
        data
    }
}

#[no_mangle]
pub extern "C" fn queue_pop(queue: *mut queue_head) -> *mut c_void {
    if queue.is_null() || unsafe { (*queue).head.is_none() } {
        return ptr::null_mut();
    }

    let entry = unsafe { (*queue).head.as_ref().unwrap().as_ref() };
    let data = entry.data;

    if entry.next.is_none() {
        unsafe {
            (*queue).tail = None;
        }
    }

    unsafe {
        (*queue).num -= 1;
        // dealloc(entry as *mut u8, Layout::new::<queue_entry>());
    }

    data
}

#[no_mangle]
pub extern "C" fn queue_peek(queue: *const queue_head) -> *mut std::ffi::c_void {
    if queue.is_null() || unsafe { (*queue).head.is_none() } {
        return ptr::null_mut();
    }

    unsafe {
        (*(*queue).head.as_ref().unwrap()).data
    }
}