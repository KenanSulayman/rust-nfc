/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct __mbstate_t {
    pub _bindgen_data_: [u64; 16usize],
}
impl __mbstate_t {
    pub unsafe fn __mbstate8(&mut self)
     -> *mut [::std::os::raw::c_char; 128usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _mbstateL(&mut self) -> *mut ::std::os::raw::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for __mbstate_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for __mbstate_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                  *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
impl ::std::default::Default for __darwin_pthread_handler_rec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_attr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
impl ::std::clone::Clone for _opaque_pthread_cond_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_cond_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_condattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_mutex_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_mutex_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_mutexattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_once_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
impl ::std::clone::Clone for _opaque_pthread_rwlock_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_rwlock_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
impl ::std::default::Default for _opaque_pthread_rwlockattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
impl ::std::clone::Clone for _opaque_pthread_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type rsize_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for __sbuf {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum __sFILEX { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                               *mut ::std::os::raw::c_void)
                                          -> ::std::os::raw::c_int>,
    pub _read: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::std::os::raw::c_void,
                                                          arg2:
                                                              *mut ::std::os::raw::c_char,
                                                          arg3:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>,
    pub _seek: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::std::os::raw::c_void,
                                                          arg2: fpos_t,
                                                          arg3:
                                                              ::std::os::raw::c_int)
                                         -> fpos_t>,
    pub _write: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                               *mut ::std::os::raw::c_void,
                                                           arg2:
                                                               *const ::std::os::raw::c_char,
                                                           arg3:
                                                               ::std::os::raw::c_int)
                                          -> ::std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl ::std::default::Default for __sFILE {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FILE = __sFILE;
pub type off_t = __darwin_off_t;
pub type ssize_t = isize;
pub enum nfc_context { }
pub enum nfc_device { }
pub enum nfc_driver { }
pub type nfc_connstring = [::std::os::raw::c_char; 1024usize];
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nfc_property {
    NP_TIMEOUT_COMMAND = 0,
    NP_TIMEOUT_ATR = 1,
    NP_TIMEOUT_COM = 2,
    NP_HANDLE_CRC = 3,
    NP_HANDLE_PARITY = 4,
    NP_ACTIVATE_FIELD = 5,
    NP_ACTIVATE_CRYPTO1 = 6,
    NP_INFINITE_SELECT = 7,
    NP_ACCEPT_INVALID_FRAMES = 8,
    NP_ACCEPT_MULTIPLE_FRAMES = 9,
    NP_AUTO_ISO14443_4 = 10,
    NP_EASY_FRAMING = 11,
    NP_FORCE_ISO14443_A = 12,
    NP_FORCE_ISO14443_B = 13,
    NP_FORCE_SPEED_106 = 14,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nfc_dep_mode { NDM_UNDEFINED = 0, NDM_PASSIVE = 1, NDM_ACTIVE = 2, }
#[repr(C)]
#[derive(Copy)]
pub struct nfc_dep_info {
    pub abtNFCID3: [uint8_t; 10usize],
    pub btDID: uint8_t,
    pub btBS: uint8_t,
    pub btBR: uint8_t,
    pub btTO: uint8_t,
    pub btPP: uint8_t,
    pub abtGB: [uint8_t; 48usize],
    pub szGB: size_t,
    pub ndm: nfc_dep_mode,
}
impl ::std::clone::Clone for nfc_dep_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for nfc_dep_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct nfc_iso14443a_info {
    pub abtAtqa: [uint8_t; 2usize],
    pub btSak: uint8_t,
    pub szUidLen: size_t,
    pub abtUid: [uint8_t; 10usize],
    pub szAtsLen: size_t,
    pub abtAts: [uint8_t; 254usize],
}
impl ::std::clone::Clone for nfc_iso14443a_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for nfc_iso14443a_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_felica_info {
    pub szLen: size_t,
    pub btResCode: uint8_t,
    pub abtId: [uint8_t; 8usize],
    pub abtPad: [uint8_t; 8usize],
    pub abtSysCode: [uint8_t; 2usize],
}
impl ::std::default::Default for nfc_felica_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_iso14443b_info {
    pub abtPupi: [uint8_t; 4usize],
    pub abtApplicationData: [uint8_t; 4usize],
    pub abtProtocolInfo: [uint8_t; 3usize],
    pub ui8CardIdentifier: uint8_t,
}
impl ::std::default::Default for nfc_iso14443b_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct nfc_iso14443bi_info {
    pub abtDIV: [uint8_t; 4usize],
    pub btVerLog: uint8_t,
    pub btConfig: uint8_t,
    pub szAtrLen: size_t,
    pub abtAtr: [uint8_t; 33usize],
}
impl ::std::clone::Clone for nfc_iso14443bi_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for nfc_iso14443bi_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_iso14443b2sr_info {
    pub abtUID: [uint8_t; 8usize],
}
impl ::std::default::Default for nfc_iso14443b2sr_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_iso14443b2ct_info {
    pub abtUID: [uint8_t; 4usize],
    pub btProdCode: uint8_t,
    pub btFabCode: uint8_t,
}
impl ::std::default::Default for nfc_iso14443b2ct_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_jewel_info {
    pub btSensRes: [uint8_t; 2usize],
    pub btId: [uint8_t; 4usize],
}
impl ::std::default::Default for nfc_jewel_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_target_info {
    pub _bindgen_data_: [u8; 283usize],
}
impl nfc_target_info {
    pub unsafe fn nai(&mut self) -> *mut nfc_iso14443a_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nfi(&mut self) -> *mut nfc_felica_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nbi(&mut self) -> *mut nfc_iso14443b_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nii(&mut self) -> *mut nfc_iso14443bi_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nsi(&mut self) -> *mut nfc_iso14443b2sr_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nci(&mut self) -> *mut nfc_iso14443b2ct_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nji(&mut self) -> *mut nfc_jewel_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ndi(&mut self) -> *mut nfc_dep_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for nfc_target_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nfc_baud_rate {
    NBR_UNDEFINED = 0,
    NBR_106 = 1,
    NBR_212 = 2,
    NBR_424 = 3,
    NBR_847 = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nfc_modulation_type {
    NMT_ISO14443A = 1,
    NMT_JEWEL = 2,
    NMT_ISO14443B = 3,
    NMT_ISO14443BI = 4,
    NMT_ISO14443B2SR = 5,
    NMT_ISO14443B2CT = 6,
    NMT_FELICA = 7,
    NMT_DEP = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nfc_mode { N_TARGET = 0, N_INITIATOR = 1, }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_modulation {
    pub nmt: nfc_modulation_type,
    pub nbr: nfc_baud_rate,
}
impl ::std::default::Default for nfc_modulation {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nfc_target {
    pub nti: nfc_target_info,
    pub nm: nfc_modulation,
}
impl ::std::default::Default for nfc_target {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for __va_list_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "nfc", kind = "dylib")]
extern "C" {
    pub static mut __stdinp: *mut FILE;
    pub static mut __stdoutp: *mut FILE;
    pub static mut __stderrp: *mut FILE;
    pub static sys_nerr: ::std::os::raw::c_int;
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
#[link(name = "nfc", kind = "dylib")]
extern "C" {
    pub fn renameat(arg1: ::std::os::raw::c_int,
                    arg2: *const ::std::os::raw::c_char,
                    arg3: ::std::os::raw::c_int,
                    arg4: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn renamex_np(arg1: *const ::std::os::raw::c_char,
                      arg2: *const ::std::os::raw::c_char,
                      arg3: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn renameatx_np(arg1: ::std::os::raw::c_int,
                        arg2: *const ::std::os::raw::c_char,
                        arg3: ::std::os::raw::c_int,
                        arg4: *const ::std::os::raw::c_char,
                        arg5: ::std::os::raw::c_uint)
     -> ::std::os::raw::c_int;
    pub fn clearerr(arg1: *mut FILE);
    pub fn fclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn feof(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fflush(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fgetc(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t)
     -> ::std::os::raw::c_int;
    pub fn fgets(arg1: *mut ::std::os::raw::c_char,
                 arg2: ::std::os::raw::c_int, arg3: *mut FILE)
     -> *mut ::std::os::raw::c_char;
    pub fn fopen(__filename: *const ::std::os::raw::c_char,
                 __mode: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn fprintf(arg1: *mut FILE, arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn fputc(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn fputs(arg1: *const ::std::os::raw::c_char, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn fread(__ptr: *mut ::std::os::raw::c_void, __size: size_t,
                 __nitems: size_t, __stream: *mut FILE) -> size_t;
    pub fn freopen(arg1: *const ::std::os::raw::c_char,
                   arg2: *const ::std::os::raw::c_char, arg3: *mut FILE)
     -> *mut FILE;
    pub fn fscanf(arg1: *mut FILE, arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn fseek(arg1: *mut FILE, arg2: ::std::os::raw::c_long,
                 arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t)
     -> ::std::os::raw::c_int;
    pub fn ftell(arg1: *mut FILE) -> ::std::os::raw::c_long;
    pub fn fwrite(__ptr: *const ::std::os::raw::c_void, __size: size_t,
                  __nitems: size_t, __stream: *mut FILE) -> size_t;
    pub fn getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar() -> ::std::os::raw::c_int;
    pub fn gets(arg1: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn perror(arg1: *const ::std::os::raw::c_char);
    pub fn printf(arg1: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn putc(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putchar(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn puts(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn remove(arg1: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rename(__old: *const ::std::os::raw::c_char,
                  __new: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rewind(arg1: *mut FILE);
    pub fn scanf(arg1: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn setbuf(arg1: *mut FILE, arg2: *mut ::std::os::raw::c_char);
    pub fn setvbuf(arg1: *mut FILE, arg2: *mut ::std::os::raw::c_char,
                   arg3: ::std::os::raw::c_int, arg4: size_t)
     -> ::std::os::raw::c_int;
    pub fn sprintf(arg1: *mut ::std::os::raw::c_char,
                   arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn sscanf(arg1: *const ::std::os::raw::c_char,
                  arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn ungetc(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn vfprintf(arg1: *mut FILE, arg2: *const ::std::os::raw::c_char,
                    arg3: va_list) -> ::std::os::raw::c_int;
    pub fn vprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list)
     -> ::std::os::raw::c_int;
    pub fn vsprintf(arg1: *mut ::std::os::raw::c_char,
                    arg2: *const ::std::os::raw::c_char, arg3: va_list)
     -> ::std::os::raw::c_int;
    pub fn ctermid(arg1: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn fdopen(arg1: ::std::os::raw::c_int,
                  arg2: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn fileno(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn pclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn popen(arg1: *const ::std::os::raw::c_char,
                 arg2: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn __srget(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn __svfscanf(arg1: *mut FILE, arg2: *const ::std::os::raw::c_char,
                      arg3: va_list) -> ::std::os::raw::c_int;
    pub fn __swbuf(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn __sputc(_c: ::std::os::raw::c_int, _p: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn flockfile(arg1: *mut FILE);
    pub fn ftrylockfile(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn funlockfile(arg1: *mut FILE);
    pub fn getc_unlocked(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
    pub fn putc_unlocked(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putchar_unlocked(arg1: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn getw(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn putw(arg1: ::std::os::raw::c_int, arg2: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn tempnam(__dir: *const ::std::os::raw::c_char,
                   __prefix: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn fseeko(__stream: *mut FILE, __offset: off_t,
                  __whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ftello(__stream: *mut FILE) -> off_t;
    pub fn snprintf(__str: *mut ::std::os::raw::c_char, __size: size_t,
                    __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn vfscanf(__stream: *mut FILE,
                   __format: *const ::std::os::raw::c_char, arg1: va_list)
     -> ::std::os::raw::c_int;
    pub fn vscanf(__format: *const ::std::os::raw::c_char, arg1: va_list)
     -> ::std::os::raw::c_int;
    pub fn vsnprintf(__str: *mut ::std::os::raw::c_char, __size: size_t,
                     __format: *const ::std::os::raw::c_char, arg1: va_list)
     -> ::std::os::raw::c_int;
    pub fn vsscanf(__str: *const ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char, arg1: va_list)
     -> ::std::os::raw::c_int;
    pub fn dprintf(arg1: ::std::os::raw::c_int,
                   arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn vdprintf(arg1: ::std::os::raw::c_int,
                    arg2: *const ::std::os::raw::c_char, arg3: va_list)
     -> ::std::os::raw::c_int;
    pub fn getdelim(__linep: *mut *mut ::std::os::raw::c_char,
                    __linecapp: *mut size_t,
                    __delimiter: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ssize_t;
    pub fn getline(__linep: *mut *mut ::std::os::raw::c_char,
                   __linecapp: *mut size_t, __stream: *mut FILE) -> ssize_t;
    pub fn asprintf(arg1: *mut *mut ::std::os::raw::c_char,
                    arg2: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn ctermid_r(arg1: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn fgetln(arg1: *mut FILE, arg2: *mut size_t)
     -> *mut ::std::os::raw::c_char;
    pub fn fmtcheck(arg1: *const ::std::os::raw::c_char,
                    arg2: *const ::std::os::raw::c_char)
     -> *const ::std::os::raw::c_char;
    pub fn fpurge(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn setbuffer(arg1: *mut FILE, arg2: *mut ::std::os::raw::c_char,
                     arg3: ::std::os::raw::c_int);
    pub fn setlinebuf(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn vasprintf(arg1: *mut *mut ::std::os::raw::c_char,
                     arg2: *const ::std::os::raw::c_char, arg3: va_list)
     -> ::std::os::raw::c_int;
    pub fn zopen(arg1: *const ::std::os::raw::c_char,
                 arg2: *const ::std::os::raw::c_char,
                 arg3: ::std::os::raw::c_int) -> *mut FILE;
    pub fn funopen(arg1: *const ::std::os::raw::c_void,
                   arg2:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut ::std::os::raw::c_void,
                                                                  arg2:
                                                                      *mut ::std::os::raw::c_char,
                                                                  arg3:
                                                                      ::std::os::raw::c_int)
                                                 -> ::std::os::raw::c_int>,
                   arg3:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut ::std::os::raw::c_void,
                                                                  arg2:
                                                                      *const ::std::os::raw::c_char,
                                                                  arg3:
                                                                      ::std::os::raw::c_int)
                                                 -> ::std::os::raw::c_int>,
                   arg4:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut ::std::os::raw::c_void,
                                                                  arg2:
                                                                      fpos_t,
                                                                  arg3:
                                                                      ::std::os::raw::c_int)
                                                 -> fpos_t>,
                   arg5:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut ::std::os::raw::c_void)
                                                 -> ::std::os::raw::c_int>)
     -> *mut FILE;
    pub fn __sprintf_chk(arg1: *mut ::std::os::raw::c_char,
                         arg2: ::std::os::raw::c_int, arg3: size_t,
                         arg4: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn __snprintf_chk(arg1: *mut ::std::os::raw::c_char, arg2: size_t,
                          arg3: ::std::os::raw::c_int, arg4: size_t,
                          arg5: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn __vsprintf_chk(arg1: *mut ::std::os::raw::c_char,
                          arg2: ::std::os::raw::c_int, arg3: size_t,
                          arg4: *const ::std::os::raw::c_char, arg5: va_list)
     -> ::std::os::raw::c_int;
    pub fn __vsnprintf_chk(arg1: *mut ::std::os::raw::c_char, arg2: size_t,
                           arg3: ::std::os::raw::c_int, arg4: size_t,
                           arg5: *const ::std::os::raw::c_char, arg6: va_list)
     -> ::std::os::raw::c_int;
    pub fn nfc_init(context: *mut *mut nfc_context);
    pub fn nfc_exit(context: *mut nfc_context);
    pub fn nfc_register_driver(driver: *const nfc_driver)
     -> ::std::os::raw::c_int;
    pub fn nfc_open(context: *mut nfc_context, connstring: nfc_connstring)
     -> *mut nfc_device;
    pub fn nfc_close(pnd: *mut nfc_device);
    pub fn nfc_abort_command(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_list_devices(context: *mut nfc_context,
                            connstrings: *mut nfc_connstring,
                            connstrings_len: size_t) -> size_t;
    pub fn nfc_idle(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_initiator_init(pnd: *mut nfc_device) -> ::std::os::raw::c_int;
    pub fn nfc_initiator_init_secure_element(pnd: *mut nfc_device)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_select_passive_target(pnd: *mut nfc_device,
                                               nm: nfc_modulation,
                                               pbtInitData: *const uint8_t,
                                               szInitData: size_t,
                                               pnt: *mut nfc_target)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_list_passive_targets(pnd: *mut nfc_device,
                                              nm: nfc_modulation,
                                              ant: *mut nfc_target,
                                              szTargets: size_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_poll_target(pnd: *mut nfc_device,
                                     pnmTargetTypes: *const nfc_modulation,
                                     szTargetTypes: size_t, uiPollNr: uint8_t,
                                     uiPeriod: uint8_t, pnt: *mut nfc_target)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_select_dep_target(pnd: *mut nfc_device,
                                           ndm: nfc_dep_mode,
                                           nbr: nfc_baud_rate,
                                           pndiInitiator: *const nfc_dep_info,
                                           pnt: *mut nfc_target,
                                           timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_poll_dep_target(pnd: *mut nfc_device,
                                         ndm: nfc_dep_mode,
                                         nbr: nfc_baud_rate,
                                         pndiInitiator: *const nfc_dep_info,
                                         pnt: *mut nfc_target,
                                         timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_deselect_target(pnd: *mut nfc_device)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bytes(pnd: *mut nfc_device,
                                          pbtTx: *const uint8_t, szTx: size_t,
                                          pbtRx: *mut uint8_t, szRx: size_t,
                                          timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bits(pnd: *mut nfc_device,
                                         pbtTx: *const uint8_t,
                                         szTxBits: size_t,
                                         pbtTxPar: *const uint8_t,
                                         pbtRx: *mut uint8_t, szRx: size_t,
                                         pbtRxPar: *mut uint8_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bytes_timed(pnd: *mut nfc_device,
                                                pbtTx: *const uint8_t,
                                                szTx: size_t,
                                                pbtRx: *mut uint8_t,
                                                szRx: size_t,
                                                cycles: *mut uint32_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_transceive_bits_timed(pnd: *mut nfc_device,
                                               pbtTx: *const uint8_t,
                                               szTxBits: size_t,
                                               pbtTxPar: *const uint8_t,
                                               pbtRx: *mut uint8_t,
                                               szRx: size_t,
                                               pbtRxPar: *mut uint8_t,
                                               cycles: *mut uint32_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_initiator_target_is_present(pnd: *mut nfc_device,
                                           pnt: *const nfc_target)
     -> ::std::os::raw::c_int;
    pub fn nfc_target_init(pnd: *mut nfc_device, pnt: *mut nfc_target,
                           pbtRx: *mut uint8_t, szRx: size_t,
                           timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_target_send_bytes(pnd: *mut nfc_device, pbtTx: *const uint8_t,
                                 szTx: size_t, timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_target_receive_bytes(pnd: *mut nfc_device, pbtRx: *mut uint8_t,
                                    szRx: size_t,
                                    timeout: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_target_send_bits(pnd: *mut nfc_device, pbtTx: *const uint8_t,
                                szTxBits: size_t, pbtTxPar: *const uint8_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_target_receive_bits(pnd: *mut nfc_device, pbtRx: *mut uint8_t,
                                   szRx: size_t, pbtRxPar: *mut uint8_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_strerror(pnd: *const nfc_device)
     -> *const ::std::os::raw::c_char;
    pub fn nfc_strerror_r(pnd: *const nfc_device,
                          buf: *mut ::std::os::raw::c_char, buflen: size_t)
     -> ::std::os::raw::c_int;
    pub fn nfc_perror(pnd: *const nfc_device,
                      s: *const ::std::os::raw::c_char);
    pub fn nfc_device_get_last_error(pnd: *const nfc_device)
     -> ::std::os::raw::c_int;
    pub fn nfc_device_get_name(pnd: *mut nfc_device)
     -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_connstring(pnd: *mut nfc_device)
     -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_supported_modulation(pnd: *mut nfc_device,
                                               mode: nfc_mode,
                                               supported_mt:
                                                   *mut *const nfc_modulation_type)
     -> ::std::os::raw::c_int;
    pub fn nfc_device_get_supported_baud_rate(pnd: *mut nfc_device,
                                              nmt: nfc_modulation_type,
                                              supported_br:
                                                  *mut *const nfc_baud_rate)
     -> ::std::os::raw::c_int;
    pub fn nfc_device_get_supported_baud_rate_target_mode(pnd:
                                                              *mut nfc_device,
                                                          nmt:
                                                              nfc_modulation_type,
                                                          supported_br:
                                                              *mut *const nfc_baud_rate)
     -> ::std::os::raw::c_int;
    pub fn nfc_device_set_property_int(pnd: *mut nfc_device,
                                       property: nfc_property,
                                       value: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nfc_device_set_property_bool(pnd: *mut nfc_device,
                                        property: nfc_property, bEnable: u8)
     -> ::std::os::raw::c_int;
    pub fn iso14443a_crc(pbtData: *mut uint8_t, szLen: size_t,
                         pbtCrc: *mut uint8_t);
    pub fn iso14443a_crc_append(pbtData: *mut uint8_t, szLen: size_t);
    pub fn iso14443b_crc(pbtData: *mut uint8_t, szLen: size_t,
                         pbtCrc: *mut uint8_t);
    pub fn iso14443b_crc_append(pbtData: *mut uint8_t, szLen: size_t);
    pub fn iso14443a_locate_historical_bytes(pbtAts: *mut uint8_t,
                                             szAts: size_t,
                                             pszTk: *mut size_t)
     -> *mut uint8_t;
    pub fn nfc_free(p: *mut ::std::os::raw::c_void);
    pub fn nfc_version() -> *const ::std::os::raw::c_char;
    pub fn nfc_device_get_information_about(pnd: *mut nfc_device,
                                            buf:
                                                *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn str_nfc_modulation_type(nmt: nfc_modulation_type)
     -> *const ::std::os::raw::c_char;
    pub fn str_nfc_baud_rate(nbr: nfc_baud_rate)
     -> *const ::std::os::raw::c_char;
    pub fn str_nfc_target(buf: *mut *mut ::std::os::raw::c_char,
                          pnt: *const nfc_target, verbose: u8)
     -> ::std::os::raw::c_int;
}


#[test]
fn it_works() {
}
