
use libc::{c_uint, size_t, uint8_t, uint32_t};

pub enum NFCContextStruct { }
pub type NFCContext = NFCContextStruct;

pub enum NFCDeviceStruct { }
pub type NFCDevice = NFCDeviceStruct;

pub enum NFCDriverStruct { }
pub type NFCDriver = NFCDriverStruct;

pub type NFCConnstring = [::libc::c_char; 1024usize];

#[repr(C)]
#[derive(PartialEq)]
pub enum NFCProperty {
    NP_TIMEOUT_COMMAND,
    NP_TIMEOUT_ATR,
    NP_TIMEOUT_COM,
    NP_HANDLE_CRC,
    NP_HANDLE_PARITY,
    NP_ACTIVATE_FIELD,
    NP_ACTIVATE_CRYPTO1,
    NP_INFINITE_SELECT,
    NP_ACCEPT_INVALID_FRAMES,
    NP_ACCEPT_MULTIPLE_FRAMES,
    NP_AUTO_ISO14443_4,
    NP_EASY_FRAMING,
    NP_FORCE_ISO14443_A,
    NP_FORCE_ISO14443_B,
    NP_FORCE_SPEED_106,
}

pub enum NFCDepMode {
    NDM_UNDEFINED,
    NDM_PASSIVE,
    NDM_ACTIVE,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub abtNFCID3: [uint8_t; 10usize],
    pub btDID: uint8_t,
    pub btBS: uint8_t,
    pub btBR: uint8_t,
    pub btTO: uint8_t,
    pub btPP: uint8_t,
    pub abtGB: [uint8_t; 48usize],
    pub szGB: size_t,
    pub ndm: NFCDepMode,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_dep_info = Struct_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub abtAtqa: [uint8_t; 2usize],
    pub btSak: uint8_t,
    pub szUidLen: size_t,
    pub abtUid: [uint8_t; 10usize],
    pub szAtsLen: size_t,
    pub abtAts: [uint8_t; 254usize],
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443a_info = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub szLen: size_t,
    pub btResCode: uint8_t,
    pub abtId: [uint8_t; 8usize],
    pub abtPad: [uint8_t; 8usize],
    pub abtSysCode: [uint8_t; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_felica_info = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub abtPupi: [uint8_t; 4usize],
    pub abtApplicationData: [uint8_t; 4usize],
    pub abtProtocolInfo: [uint8_t; 3usize],
    pub ui8CardIdentifier: uint8_t,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b_info = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub abtDIV: [uint8_t; 4usize],
    pub btVerLog: uint8_t,
    pub btConfig: uint8_t,
    pub szAtrLen: size_t,
    pub abtAtr: [uint8_t; 33usize],
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443bi_info = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub abtUID: [uint8_t; 8usize],
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b2sr_info = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub abtUID: [uint8_t; 4usize],
    pub btProdCode: uint8_t,
    pub btFabCode: uint8_t,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_iso14443b2ct_info = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub btSensRes: [uint8_t; 2usize],
    pub btId: [uint8_t; 4usize],
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_jewel_info = Struct_Unnamed10;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed11 {
    pub _bindgen_data_: [u8; 283usize],
}
impl Union_Unnamed11 {
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
impl ::std::clone::Clone for Union_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_target_info = Union_Unnamed11;
pub type Enum_Unnamed12 = c_uint;
pub const NBR_UNDEFINED: c_uint = 0;
pub const NBR_106: c_uint = 1;
pub const NBR_212: c_uint = 2;
pub const NBR_424: c_uint = 3;
pub const NBR_847: c_uint = 4;
pub type nfc_baud_rate = Enum_Unnamed12;
pub type Enum_Unnamed13 = c_uint;
pub const NMT_ISO14443A: c_uint = 1;
pub const NMT_JEWEL: c_uint = 2;
pub const NMT_ISO14443B: c_uint = 3;
pub const NMT_ISO14443BI: c_uint = 4;
pub const NMT_ISO14443B2SR: c_uint = 5;
pub const NMT_ISO14443B2CT: c_uint = 6;
pub const NMT_FELICA: c_uint = 7;
pub const NMT_DEP: c_uint = 8;
pub type nfc_modulation_type = Enum_Unnamed13;
pub type Enum_Unnamed14 = c_uint;
pub const N_TARGET: c_uint = 0;
pub const N_INITIATOR: c_uint = 1;
pub type nfc_mode = Enum_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub nmt: nfc_modulation_type,
    pub nbr: nfc_baud_rate,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type nfc_modulation = Struct_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub nti: nfc_target_info,
    pub nm: nfc_modulation,
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type NFCTarget = Struct_Unnamed16;
#[link(name = "/usr/include/nfc")]
extern "C" {
    pub fn nfc_init(context: *mut *mut NFCContext) -> ();
    pub fn nfc_exit(context: *mut NFCContext) -> ();
    pub fn nfc_register_driver(driver: *const NFCDriver) -> ::libc::c_int;
    pub fn nfc_open(context: *mut NFCContext, connstring: NFCConnstring)
     -> *mut NFCDevice;
    pub fn nfc_close(pnd: *mut NFCDevice) -> ();
    pub fn nfc_abort_command(pnd: *mut NFCDevice) -> ::libc::c_int;
    pub fn nfc_list_devices(context: *mut NFCContext,
                            connstrings: *mut NFCConnstring,
                            connstrings_len: size_t) -> size_t;
    pub fn nfc_idle(pnd: *mut NFCDevice) -> ::libc::c_int;
    pub fn nfc_initiator_init(pnd: *mut NFCDevice) -> ::libc::c_int;
    pub fn nfc_initiator_init_secure_element(pnd: *mut NFCDevice)
     -> ::libc::c_int;
    pub fn nfc_initiator_select_passive_target(pnd: *mut NFCDevice,
                                               nm: nfc_modulation,
                                               pbtInitData: *const uint8_t,
                                               szInitData: size_t,
                                               pnt: *mut NFCTarget)
     -> ::libc::c_int;
    pub fn nfc_initiator_list_passive_targets(pnd: *mut NFCDevice,
                                              nm: nfc_modulation,
                                              ant: *mut NFCTarget,
                                              szTargets: size_t)
     -> ::libc::c_int;
    pub fn nfc_initiator_poll_target(pnd: *mut NFCDevice,
                                     pnmTargetTypes: *const nfc_modulation,
                                     szTargetTypes: size_t, uiPollNr: uint8_t,
                                     uiPeriod: uint8_t, pnt: *mut NFCTarget)
     -> ::libc::c_int;
    pub fn nfc_initiator_select_dep_target(pnd: *mut NFCDevice,
                                           ndm: NFCDepMode,
                                           nbr: nfc_baud_rate,
                                           pndiInitiator: *const nfc_dep_info,
                                           pnt: *mut NFCTarget,
                                           timeout: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nfc_initiator_poll_dep_target(pnd: *mut NFCDevice,
                                         ndm: NFCDepMode,
                                         nbr: nfc_baud_rate,
                                         pndiInitiator: *const nfc_dep_info,
                                         pnt: *mut NFCTarget,
                                         timeout: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nfc_initiator_deselect_target(pnd: *mut NFCDevice)
     -> ::libc::c_int;
    pub fn nfc_initiator_transceive_bytes(pnd: *mut NFCDevice,
                                          pbtTx: *const uint8_t, szTx: size_t,
                                          pbtRx: *mut uint8_t, szRx: size_t,
                                          timeout: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nfc_initiator_transceive_bits(pnd: *mut NFCDevice,
                                         pbtTx: *const uint8_t,
                                         szTxBits: size_t,
                                         pbtTxPar: *const uint8_t,
                                         pbtRx: *mut uint8_t, szRx: size_t,
                                         pbtRxPar: *mut uint8_t)
     -> ::libc::c_int;
    pub fn nfc_initiator_transceive_bytes_timed(pnd: *mut NFCDevice,
                                                pbtTx: *const uint8_t,
                                                szTx: size_t,
                                                pbtRx: *mut uint8_t,
                                                szRx: size_t,
                                                cycles: *mut uint32_t)
     -> ::libc::c_int;
    pub fn nfc_initiator_transceive_bits_timed(pnd: *mut NFCDevice,
                                               pbtTx: *const uint8_t,
                                               szTxBits: size_t,
                                               pbtTxPar: *const uint8_t,
                                               pbtRx: *mut uint8_t,
                                               szRx: size_t,
                                               pbtRxPar: *mut uint8_t,
                                               cycles: *mut uint32_t)
     -> ::libc::c_int;
    pub fn nfc_initiator_target_is_present(pnd: *mut NFCDevice,
                                           pnt: *const NFCTarget)
     -> ::libc::c_int;
    pub fn nfc_target_init(pnd: *mut NFCDevice, pnt: *mut NFCTarget,
                           pbtRx: *mut uint8_t, szRx: size_t,
                           timeout: ::libc::c_int) -> ::libc::c_int;
    pub fn nfc_target_send_bytes(pnd: *mut NFCDevice, pbtTx: *const uint8_t,
                                 szTx: size_t, timeout: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nfc_target_receive_bytes(pnd: *mut NFCDevice, pbtRx: *mut uint8_t,
                                    szRx: size_t, timeout: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nfc_target_send_bits(pnd: *mut NFCDevice, pbtTx: *const uint8_t,
                                szTxBits: size_t, pbtTxPar: *const uint8_t)
     -> ::libc::c_int;
    pub fn nfc_target_receive_bits(pnd: *mut NFCDevice, pbtRx: *mut uint8_t,
                                   szRx: size_t, pbtRxPar: *mut uint8_t)
     -> ::libc::c_int;
    pub fn nfc_strerror(pnd: *const NFCDevice) -> *const ::libc::c_char;
    pub fn nfc_strerror_r(pnd: *const NFCDevice, buf: *mut ::libc::c_char,
                          buflen: size_t) -> ::libc::c_int;
    pub fn nfc_perror(pnd: *const NFCDevice, s: *const ::libc::c_char) -> ();
    pub fn nfc_device_get_last_error(pnd: *const NFCDevice) -> ::libc::c_int;
    pub fn nfc_device_get_name(pnd: *mut NFCDevice) -> *const ::libc::c_char;
    pub fn nfc_device_get_connstring(pnd: *mut NFCDevice)
     -> *const ::libc::c_char;
    pub fn nfc_device_get_supported_modulation(pnd: *mut NFCDevice,
                                               mode: nfc_mode,
                                               supported_mt:
                                                   *mut *const nfc_modulation_type)
     -> ::libc::c_int;
    pub fn nfc_device_get_supported_baud_rate(pnd: *mut NFCDevice,
                                              nmt: nfc_modulation_type,
                                              supported_br:
                                                  *mut *const nfc_baud_rate)
     -> ::libc::c_int;
    pub fn nfc_device_set_property_int(pnd: *mut NFCDevice,
                                       property: NFCProperty,
                                       value: ::libc::c_int) -> ::libc::c_int;
    pub fn nfc_device_set_property_bool(pnd: *mut NFCDevice,
                                        property: NFCProperty, bEnable: u8)
     -> ::libc::c_int;
    pub fn iso14443a_crc(pbtData: *mut uint8_t, szLen: size_t,
                         pbtCrc: *mut uint8_t) -> ();
    pub fn iso14443a_crc_append(pbtData: *mut uint8_t, szLen: size_t) -> ();
    pub fn iso14443b_crc(pbtData: *mut uint8_t, szLen: size_t,
                         pbtCrc: *mut uint8_t) -> ();
    pub fn iso14443b_crc_append(pbtData: *mut uint8_t, szLen: size_t) -> ();
    pub fn iso14443a_locate_historical_bytes(pbtAts: *mut uint8_t,
                                             szAts: size_t,
                                             pszTk: *mut size_t)
     -> *mut uint8_t;
    pub fn nfc_free(p: *mut ::libc::c_void) -> ();
    pub fn nfc_version() -> *const ::libc::c_char;
    pub fn nfc_device_get_information_about(pnd: *mut NFCDevice,
                                            buf: *mut *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn str_nfc_modulation_type(nmt: nfc_modulation_type)
     -> *const ::libc::c_char;
    pub fn str_nfc_baud_rate(nbr: nfc_baud_rate) -> *const ::libc::c_char;
    pub fn str_nfc_target(buf: *mut *mut ::libc::c_char,
                          pnt: *const NFCTarget, verbose: u8)
     -> ::libc::c_int;
}
