extern crate tempdir;
extern crate notmuch_sys;

use std::fs::{self, File};
use std::io::Write;
use std::ptr;
use std::ffi::{CStr, CString};
use notmuch_sys::*;
use std::os::unix::ffi::OsStrExt;

static MESSAGE: &'static [u8] = b"\
To: bob@example.com
From: alice@example.com
Subject: My Message

Cool body!";

#[test]
fn it_works() {
    let dir = tempdir::TempDir::new("notmuch")
        .expect("failed to create temporary directory");

    let msg_path = dir.path().join("cur/message");

    fs::create_dir(msg_path.parent().unwrap())
        .expect("failed to maildir directory");

    let mut msg_file = File::create(&msg_path)
        .expect("failed to create message file");

    msg_file.write_all(MESSAGE)
        .expect("failed to write message");

    unsafe {
        let mut db = ptr::null_mut();
        {
            let dir_c_str = CString::new(dir.path()
                                         .as_os_str()
                                         .as_bytes()).unwrap();
            assert_eq!(
                notmuch_database_create(dir_c_str.as_ptr(), &mut db),
                notmuch_status_t::SUCCESS
            );
        }
        let mut msg = ptr::null_mut();
        {
            let msg_c_str = CString::new(msg_path.as_os_str()
                                         .as_bytes()).unwrap();
            assert_eq!(
                notmuch_database_add_message(db, msg_c_str.as_ptr(), &mut msg),
                notmuch_status_t::SUCCESS
            );
        }
        let header = notmuch_message_get_header(msg, b"From\0" as *const u8 as *const i8);
        assert!(!header.is_null());
        assert_eq!(
            CStr::from_ptr(header),
            CStr::from_ptr(b"alice@example.com\0" as *const u8 as *const i8)
        );
    }
}
