/* automatically generated by rust-bindgen 0.64.0 */

#![allow(dead_code, 
                    non_camel_case_types, 
                    non_snake_case, 
                    non_upper_case_globals,
                    trivial_numeric_casts)]
#![allow(clippy::cast_lossless,
                clippy::approx_constant,
                clippy::cast_possible_truncation,
                clippy::default_trait_access,
                clippy::missing_const_for_fn,
                clippy::missing_safety_doc,
                clippy::must_use_candidate,
                clippy::octal_escapes,        
                clippy::ptr_as_ptr,
                clippy::semicolon_if_nothing_returned,
                clippy::too_many_arguments,
                clippy::too_many_lines,
                clippy::transmute_ptr_to_ptr,
                clippy::type_complexity,
                clippy::unnecessary_cast,
                clippy::unreadable_literal, 
                clippy::upper_case_acronyms,
                clippy::use_self,
                clippy::used_underscore_binding,
                clippy::useless_transmute)]

pub type SRes = ::std::os::raw::c_int;
pub type Byte = ::std::os::raw::c_uchar;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type UInt32 = ::std::os::raw::c_uint;
pub type UInt64 = ::std::os::raw::c_ulonglong;
pub type SizeT = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISeqInStream {
    pub Read: ::std::option::Option<
        unsafe extern "C" fn(
            p: *const ISeqInStream,
            buf: *mut ::std::os::raw::c_void,
            size: *mut usize,
        ) -> SRes,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISeqOutStream {
    pub Write: ::std::option::Option<
        unsafe extern "C" fn(
            p: *const ISeqOutStream,
            buf: *const ::std::os::raw::c_void,
            size: usize,
        ) -> usize,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICompressProgress {
    pub Progress: ::std::option::Option<
        unsafe extern "C" fn(p: *const ICompressProgress, inSize: UInt64, outSize: UInt64) -> SRes,
    >,
}
pub type ISzAllocPtr = *const ISzAlloc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISzAlloc {
    pub Alloc: ::std::option::Option<
        unsafe extern "C" fn(p: ISzAllocPtr, size: usize) -> *mut ::std::os::raw::c_void,
    >,
    pub Free: ::std::option::Option<
        unsafe extern "C" fn(p: ISzAllocPtr, address: *mut ::std::os::raw::c_void),
    >,
}
pub type CLzmaProb = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CLzmaProps {
    pub lc: Byte,
    pub lp: Byte,
    pub pb: Byte,
    pub _pad_: Byte,
    pub dicSize: UInt32,
}
pub type CLzmaProps = _CLzmaProps;
extern "C" {
    pub fn LzmaProps_Decode(
        p: *mut CLzmaProps,
        data: *const Byte,
        size: ::std::os::raw::c_uint,
    ) -> SRes;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CLzmaDec {
    pub prop: CLzmaProps,
    pub probs: *mut CLzmaProb,
    pub probs_1664: *mut CLzmaProb,
    pub dic: *mut Byte,
    pub dicBufSize: SizeT,
    pub dicPos: SizeT,
    pub buf: *const Byte,
    pub range: UInt32,
    pub code: UInt32,
    pub processedPos: UInt32,
    pub checkDicSize: UInt32,
    pub reps: [UInt32; 4usize],
    pub state: UInt32,
    pub remainLen: UInt32,
    pub numProbs: UInt32,
    pub tempBufSize: ::std::os::raw::c_uint,
    pub tempBuf: [Byte; 20usize],
}
extern "C" {
    pub fn LzmaDec_Init(p: *mut CLzmaDec);
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELzmaFinishMode {
    LZMA_FINISH_ANY = 0,
    LZMA_FINISH_END = 1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELzmaStatus {
    LZMA_STATUS_NOT_SPECIFIED = 0,
    LZMA_STATUS_FINISHED_WITH_MARK = 1,
    LZMA_STATUS_NOT_FINISHED = 2,
    LZMA_STATUS_NEEDS_MORE_INPUT = 3,
    LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK = 4,
}
extern "C" {
    pub fn LzmaDec_AllocateProbs(
        p: *mut CLzmaDec,
        props: *const Byte,
        propsSize: ::std::os::raw::c_uint,
        alloc: ISzAllocPtr,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaDec_FreeProbs(p: *mut CLzmaDec, alloc: ISzAllocPtr);
}
extern "C" {
    pub fn LzmaDec_Allocate(
        p: *mut CLzmaDec,
        props: *const Byte,
        propsSize: ::std::os::raw::c_uint,
        alloc: ISzAllocPtr,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaDec_Free(p: *mut CLzmaDec, alloc: ISzAllocPtr);
}
extern "C" {
    pub fn LzmaDec_DecodeToDic(
        p: *mut CLzmaDec,
        dicLimit: SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaDec_DecodeToBuf(
        p: *mut CLzmaDec,
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaDecode(
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        propData: *const Byte,
        propSize: ::std::os::raw::c_uint,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
        alloc: ISzAllocPtr,
    ) -> SRes;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CLzma2Dec {
    pub state: ::std::os::raw::c_uint,
    pub control: Byte,
    pub needInitLevel: Byte,
    pub isExtraMode: Byte,
    pub _pad_: Byte,
    pub packSize: UInt32,
    pub unpackSize: UInt32,
    pub decoder: CLzmaDec,
}
extern "C" {
    pub fn Lzma2Dec_AllocateProbs(p: *mut CLzma2Dec, prop: Byte, alloc: ISzAllocPtr) -> SRes;
}
extern "C" {
    pub fn Lzma2Dec_Allocate(p: *mut CLzma2Dec, prop: Byte, alloc: ISzAllocPtr) -> SRes;
}
extern "C" {
    pub fn Lzma2Dec_Init(p: *mut CLzma2Dec);
}
extern "C" {
    pub fn Lzma2Dec_DecodeToDic(
        p: *mut CLzma2Dec,
        dicLimit: SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
    ) -> SRes;
}
extern "C" {
    pub fn Lzma2Dec_DecodeToBuf(
        p: *mut CLzma2Dec,
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
    ) -> SRes;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELzma2ParseStatus {
    LZMA2_PARSE_STATUS_NEW_BLOCK = 5,
    LZMA2_PARSE_STATUS_NEW_CHUNK = 6,
}
extern "C" {
    pub fn Lzma2Dec_Parse(
        p: *mut CLzma2Dec,
        outSize: SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        checkFinishBlock: ::std::os::raw::c_int,
    ) -> ELzma2ParseStatus;
}
extern "C" {
    pub fn Lzma2Decode(
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: *mut SizeT,
        prop: Byte,
        finishMode: ELzmaFinishMode,
        status: *mut ELzmaStatus,
        alloc: ISzAllocPtr,
    ) -> SRes;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CLzmaEncProps {
    pub level: ::std::os::raw::c_int,
    pub dictSize: UInt32,
    pub lc: ::std::os::raw::c_int,
    pub lp: ::std::os::raw::c_int,
    pub pb: ::std::os::raw::c_int,
    pub algo: ::std::os::raw::c_int,
    pub fb: ::std::os::raw::c_int,
    pub btMode: ::std::os::raw::c_int,
    pub numHashBytes: ::std::os::raw::c_int,
    pub mc: UInt32,
    pub writeEndMark: ::std::os::raw::c_uint,
    pub numThreads: ::std::os::raw::c_int,
    pub reduceSize: UInt64,
    pub affinity: UInt64,
}
pub type CLzmaEncProps = _CLzmaEncProps;
extern "C" {
    pub fn LzmaEncProps_Init(p: *mut CLzmaEncProps);
}
extern "C" {
    pub fn LzmaEncProps_Normalize(p: *mut CLzmaEncProps);
}
extern "C" {
    pub fn LzmaEncProps_GetDictSize(props2: *const CLzmaEncProps) -> UInt32;
}
pub type CLzmaEncHandle = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn LzmaEnc_Create(alloc: ISzAllocPtr) -> CLzmaEncHandle;
}
extern "C" {
    pub fn LzmaEnc_Destroy(p: CLzmaEncHandle, alloc: ISzAllocPtr, allocBig: ISzAllocPtr);
}
extern "C" {
    pub fn LzmaEnc_SetProps(p: CLzmaEncHandle, props: *const CLzmaEncProps) -> SRes;
}
extern "C" {
    pub fn LzmaEnc_SetDataSize(p: CLzmaEncHandle, expectedDataSiize: UInt64);
}
extern "C" {
    pub fn LzmaEnc_WriteProperties(
        p: CLzmaEncHandle,
        properties: *mut Byte,
        size: *mut SizeT,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaEnc_IsWriteEndMark(p: CLzmaEncHandle) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn LzmaEnc_Encode(
        p: CLzmaEncHandle,
        outStream: *mut ISeqOutStream,
        inStream: *mut ISeqInStream,
        progress: *mut ICompressProgress,
        alloc: ISzAllocPtr,
        allocBig: ISzAllocPtr,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaEnc_MemEncode(
        p: CLzmaEncHandle,
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: SizeT,
        writeEndMark: ::std::os::raw::c_int,
        progress: *mut ICompressProgress,
        alloc: ISzAllocPtr,
        allocBig: ISzAllocPtr,
    ) -> SRes;
}
extern "C" {
    pub fn LzmaEncode(
        dest: *mut Byte,
        destLen: *mut SizeT,
        src: *const Byte,
        srcLen: SizeT,
        props: *const CLzmaEncProps,
        propsEncoded: *mut Byte,
        propsSize: *mut SizeT,
        writeEndMark: ::std::os::raw::c_int,
        progress: *mut ICompressProgress,
        alloc: ISzAllocPtr,
        allocBig: ISzAllocPtr,
    ) -> SRes;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CLzma2EncProps {
    pub lzmaProps: CLzmaEncProps,
    pub blockSize: UInt64,
    pub numBlockThreads_Reduced: ::std::os::raw::c_int,
    pub numBlockThreads_Max: ::std::os::raw::c_int,
    pub numTotalThreads: ::std::os::raw::c_int,
}
extern "C" {
    pub fn Lzma2EncProps_Init(p: *mut CLzma2EncProps);
}
extern "C" {
    pub fn Lzma2EncProps_Normalize(p: *mut CLzma2EncProps);
}
pub type CLzma2EncHandle = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn Lzma2Enc_Create(alloc: ISzAllocPtr, allocBig: ISzAllocPtr) -> CLzma2EncHandle;
}
extern "C" {
    pub fn Lzma2Enc_Destroy(p: CLzma2EncHandle);
}
extern "C" {
    pub fn Lzma2Enc_SetProps(p: CLzma2EncHandle, props: *const CLzma2EncProps) -> SRes;
}
extern "C" {
    pub fn Lzma2Enc_SetDataSize(p: CLzma2EncHandle, expectedDataSiize: UInt64);
}
extern "C" {
    pub fn Lzma2Enc_WriteProperties(p: CLzma2EncHandle) -> Byte;
}
extern "C" {
    pub fn Lzma2Enc_Encode2(
        p: CLzma2EncHandle,
        outStream: *mut ISeqOutStream,
        outBuf: *mut Byte,
        outBufSize: *mut usize,
        inStream: *mut ISeqInStream,
        inData: *const Byte,
        inDataSize: usize,
        progress: *mut ICompressProgress,
    ) -> SRes;
}
