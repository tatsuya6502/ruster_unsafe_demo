#[macro_use]
extern crate ruster_unsafe;
use ruster_unsafe::*;
use std::mem::uninitialized;

extern crate libc;
use libc::c_uchar;
use libc::size_t;

/// Create NIF module data and init function.
nif_init!(b"ruster_unsafe_demo\0", Some(load), Some(reload), Some(upgrade), Some(unload),
	nif!(b"static_atom\0", 0, static_atom),
	nif!(b"native_add\0" , 2, native_add, ERL_NIF_DIRTY_JOB_IO_BOUND),
	nif!(b"tuple_add\0"  , 1, tuple_add, ERL_NIF_DIRTY_JOB_CPU_BOUND),
	nif!(b"getenv\0"     , 1, getenv, ERL_NIF_DIRTY_JOB_IO_BOUND)
	);


static mut my_atom:ERL_NIF_TERM = 0 as ERL_NIF_TERM;


/// Initialize static atom.
extern "C" fn load(env: *mut ErlNifEnv,
                   _priv_data: *mut *mut c_void,
                   _load_info: ERL_NIF_TERM)-> c_int {
	unsafe {
		my_atom = enif_make_atom(env, b"static atom from Rust\0" as *const u8);
		0
	}
}

/// Does nothing, reports success
extern "C" fn reload(_env: *mut ErlNifEnv,
                     _priv_data: *mut *mut c_void,
                     _load_info: ERL_NIF_TERM) -> c_int { 0 }

/// Does nothing, reports success
extern "C" fn upgrade(_env: *mut ErlNifEnv,
                      _priv_data: *mut *mut c_void,
                      _old_priv_data: *mut *mut c_void,
                      _load_info: ERL_NIF_TERM) -> c_int { 0 }

/// Does nothing, reports success
extern "C" fn unload(_env: *mut ErlNifEnv,
                     _priv_data: *mut c_void) {}

/// Provide static atom that was initialized by `load()`
extern "C" fn static_atom(_env:*mut ErlNifEnv,
                          _argc: c_int,
                          _args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
	unsafe { my_atom }
}

/// Add two integers. `native_add(A,B) -> A+B.`
extern "C" fn native_add(env: *mut ErlNifEnv,
                         argc: c_int,
                         args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
	unsafe {
		let mut a:c_int = uninitialized();
		let mut b:c_int = uninitialized();
    	if argc == 2 &&
    	   0 != enif_get_int(env, *args, &mut a) && 
    	   0 != enif_get_int(env, *args.offset(1), &mut b) {
    	   	enif_make_int(env, a+b)
    	 }
    	 else {
    	  	enif_make_badarg(env)
    	 }
	}
}

/// Add integers provided in a 2-tuple. `tuple_add({A,B}) -> A+B.`
extern "C" fn tuple_add(env: *mut ErlNifEnv,
                        argc: c_int,
                        args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
	unsafe {
		let mut a:c_int = uninitialized();
		let mut b:c_int = uninitialized();
		let mut size:c_int = uninitialized();
		let mut tup:*const ERL_NIF_TERM = uninitialized();
    	if argc == 1 &&
    	   0 != enif_get_tuple(env, *args, &mut size, &mut tup) &&
    	   size == 2 &&
    	   0 != enif_get_int(env, *tup, &mut a) && 
    	   0 != enif_get_int(env, *tup.offset(1), &mut b) {
    	   	enif_make_int(env, a+b)
    	}
    	else {
    	   	enif_make_badarg(env)
    	}
	}
}

/// Return env variable for the given key.
extern "C" fn getenv(env: *mut ErlNifEnv,
                     argc: c_int,
                     args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    unsafe {
        let mut args_len: u32 = uninitialized();
        let mut key: c_uchar = uninitialized();
        let mut value: c_uchar = uninitialized();
        let mut val_size: size_t = uninitialized();

        if argc == 1 &&
           0 != enif_get_list_length(env, *args, &mut args_len) &&
           0 != enif_get_string(env, *args, &mut key, args_len + 1,
                                ErlNifCharEncoding::ERL_NIF_LATIN1) &&
           0 == enif_getenv(&key, &mut value, &mut val_size) {
            enif_make_string(env, &value, ErlNifCharEncoding::ERL_NIF_LATIN1)
        } else {
            enif_make_badarg(env)
        }
    }
}
