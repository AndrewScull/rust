#![unstable(feature = "dios")]

use core::prelude::*;

use ffi::CString;
use libc;
use io::{self, ErrorKind};
use mem;
use ptr;
use sys_common;

/*
The type for the channel should not allow shared memory but I don't think there is a trait for that.
*/

pub fn spawn_task(f: fn(Channel)) -> io::Result<Channel> {
    unsafe {
        // Get a reference for the executable
        let exe_ref = libc::dios_pickref(libc::dios_self_exe());
    
        // Put the address in argv (bit dodgy, but this is a hack anyway)
        let entry_ptr: libc::intptr_t = mem::transmute(f);
        let offset_ptr: libc::intptr_t = mem::transmute(sys_common::thread::start_thread);
        let arg0: *mut i8 = mem::transmute(CString::new("thread").unwrap().as_ptr());
        let arg1: *mut i8 = mem::transmute(CString::new(format!("{}", entry_ptr - offset_ptr)).unwrap().as_ptr());


        // Create the channels
        let (chan, mut sname, mut rname) = channel().unwrap();
        let mut sname_str: [libc::c_char; 44] = mem::uninitialized();
        let mut rname_str: [libc::c_char; 44] = mem::uninitialized();
    
        // Launch the new task
        let mut argv = [arg0,
                        arg1,
                        libc::dios_nametostr(&mut sname, &mut sname_str[0]),
                        libc::dios_nametostr(&mut rname, &mut rname_str[0])];
        let mut task_spec = libc::dios_task_spec_t {
            input_names: ptr::null_mut(),
            input_count: 0,
            output_names: ptr::null_mut(),
            output_count: 0,
            argc: argv.len() as i32,
            argv: argv.as_mut_ptr()
        };
        let mut new_ref: *mut libc::dios_ref_t = mem::uninitialized();
        if libc::dios_run(0, exe_ref, &mut task_spec, &mut new_ref) != 0 {
            Err(io::Error::new(ErrorKind::Other,
                               "dios_run didn't work :(",
                               None))
        } else {
            Ok(chan)
        }
    }
}


////////////////////////////////
// Channel

pub struct Channel {
    send: *mut libc::dios_ref_t,
    recv: *mut libc::dios_ref_t,
}

impl Channel {
    pub fn new(send: *mut libc::dios_ref_t, recv: *mut libc::dios_ref_t) -> Channel {
        Channel {
            send: send,
            recv: recv
        }
    }

    pub fn send(&self, t: u32) -> io::Result<()> {
        unsafe {
            let size = mem::size_of::<u32>() as libc::uint64_t;
            let mut iov: *mut libc::dios_iovec_t = mem::uninitialized();
            if libc::dios_begin_write(0, self.send, size, &mut iov) != 0 {
                return Err(io::Error::new(ErrorKind::Other,
                               "sender dios_begin_write failed",
                               None))
            }

            ptr::copy_nonoverlapping((*iov).buf as *mut u32, &t, 1);

            if libc::dios_end_write(0, self.send, size, iov) != 0 {
                return Err(io::Error::new(ErrorKind::Other,
                               "sender dios_end_write failed",
                               None))
            }
        }

        Ok(())
    }

    pub fn recv(&self) -> io::Result<u32> {
        unsafe {
            let mut t: u32 = mem::uninitialized();
            let size = mem::size_of::<u32>() as libc::uint64_t;
            let mut iov: *mut libc::dios_iovec_t = mem::uninitialized();
            if libc::dios_begin_read(0, self.recv, size, &mut iov) != 0 {
                return Err(io::Error::new(ErrorKind::Other,
                               "sender dios_begin_read failed",
                               None))
            }

            ptr::copy_nonoverlapping(&mut t, (*iov).buf as *const u32, 1);

            if libc::dios_end_read(0, self.recv, iov) != 0 {
                return Err(io::Error::new(ErrorKind::Other,
                               "sender dios_end_read failed",
                               None))
            }
        
            Ok(t)
        }
    }
}

fn channel() -> io::Result<(Channel, libc::dios_name_t, libc::dios_name_t)> {
    match (make_shmem_fifo(), make_shmem_fifo()) {
        (Ok((send, sname)), Ok((recv, rname))) =>
            Ok((Channel::new(send, recv), sname, rname)),
        (Err(e),_) | (_,Err(e)) => Err(e)
    }
}

fn make_shmem_fifo() -> io::Result<(*mut libc::dios_ref_t, libc::dios_name_t)> {
    unsafe {
        let mut name: libc::dios_name_t = mem::uninitialized();
        let mut obj: *mut libc::dios_ref_t = mem::uninitialized();
        if libc::dios_create(0x100, libc::dios_object_type_t::D_OBJ_SHMEM, ptr::null_mut(),
                             &mut name, &mut obj) != 0 {
            Err(io::Error::new(ErrorKind::Other,
                               "Failed to make the shmem fifo",
                               None))
        } else {
            Ok((obj, name))
        }
    }
}
