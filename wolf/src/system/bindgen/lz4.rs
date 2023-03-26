/* automatically generated by rust-bindgen 0.64.0 */
#![allow(dead_code, non_camel_case_types, non_snake_case, trivial_numeric_casts)]
#![allow(clippy::unreadable_literal, clippy::upper_case_acronyms)]

extern "C" {
    pub fn LZ4_versionNumber() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_versionString() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " LZ4_compress_default() :\n  Compresses 'srcSize' bytes from buffer 'src'\n  into already allocated 'dst' buffer of size 'dstCapacity'.\n  Compression is guaranteed to succeed if 'dstCapacity' >= LZ4_compressBound(srcSize).\n  It also runs faster, so it's a recommended setting.\n  If the function cannot compress 'src' into a more limited 'dst' budget,\n  compression stops *immediately*, and the function result is zero.\n  In which case, 'dst' content is undefined (invalid).\n      srcSize : max supported value is LZ4_MAX_INPUT_SIZE.\n      dstCapacity : size of buffer 'dst' (which must be already allocated)\n     @return  : the number of bytes written into buffer 'dst' (necessarily <= dstCapacity)\n                or 0 if compression fails\n Note : This function is protected against buffer overflow scenarios (never writes outside 'dst' buffer, nor read outside 'source' buffer)."]
    pub fn LZ4_compress_default(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decompress_safe() :\n @compressedSize : is the exact complete size of the compressed block.\n @dstCapacity : is the size of destination buffer (which must be already allocated),\n                is an upper bound of decompressed size.\n @return : the number of bytes decompressed into destination buffer (necessarily <= dstCapacity)\n           If destination buffer is not large enough, decoding will stop and output an error code (negative value).\n           If the source stream is detected malformed, the function will stop decoding and return a negative result.\n Note 1 : This function is protected against malicious data packets :\n          it will never writes outside 'dst' buffer, nor read outside 'source' buffer,\n          even if the compressed block is maliciously modified to order the decoder to do these actions.\n          In such case, the decoder stops immediately, and considers the compressed block malformed.\n Note 2 : compressedSize and dstCapacity must be provided to the function, the compressed block does not contain them.\n          The implementation is free to send / store / derive this information in whichever way is most beneficial.\n          If there is a need for a different format which bundles together both compressed data and its metadata, consider looking at lz4frame.h instead."]
    pub fn LZ4_decompress_safe(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        compressedSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_compressBound() :\nProvides the maximum size that LZ4 compression may output in a \"worst case\" scenario (input data not compressible)\nThis function is primarily useful for memory allocation purposes (destination buffer size).\nMacro LZ4_COMPRESSBOUND() is also provided for compilation-time evaluation (stack memory allocation for example).\nNote that LZ4_compress_default() compresses faster when dstCapacity is >= LZ4_compressBound(srcSize)\ninputSize  : max supported value is LZ4_MAX_INPUT_SIZE\nreturn : maximum output size in a \"worst case\" scenario\nor 0, if input size is incorrect (too large or negative)"]
    pub fn LZ4_compressBound(inputSize: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_compress_fast() :\nSame as LZ4_compress_default(), but allows selection of \"acceleration\" factor.\nThe larger the acceleration value, the faster the algorithm, but also the lesser the compression.\nIt's a trade-off. It can be fine tuned, with each successive value providing roughly +~3% to speed.\nAn acceleration value of \"1\" is the same as regular LZ4_compress_default()\nValues <= 0 will be replaced by LZ4_ACCELERATION_DEFAULT (currently == 1, see lz4.c).\nValues > LZ4_ACCELERATION_MAX will be replaced by LZ4_ACCELERATION_MAX (currently == 65537, see lz4.c)."]
    pub fn LZ4_compress_fast(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
        acceleration: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_compress_fast_extState() :\n  Same as LZ4_compress_fast(), using an externally allocated memory space for its state.\n  Use LZ4_sizeofState() to know how much memory must be allocated,\n  and allocate it on 8-bytes boundaries (using `malloc()` typically).\n  Then, provide this buffer as `void* state` to compression function."]
    pub fn LZ4_sizeofState() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_fast_extState(
        state: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
        acceleration: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_compress_destSize() :\n  Reverse the logic : compresses as much data as possible from 'src' buffer\n  into already allocated buffer 'dst', of size >= 'targetDestSize'.\n  This function either compresses the entire 'src' content into 'dst' if it's large enough,\n  or fill 'dst' buffer completely with as much data as possible from 'src'.\n  note: acceleration parameter is fixed to \"default\".\n\n *srcSizePtr : will be modified to indicate how many bytes where read from 'src' to fill 'dst'.\n               New value is necessarily <= input value.\n @return : Nb bytes written into 'dst' (necessarily <= targetDestSize)\n           or 0 if compression fails.\n\n Note : from v1.8.2 to v1.9.1, this function had a bug (fixed un v1.9.2+):\n        the produced compressed content could, in specific circumstances,\n        require to be decompressed into a destination buffer larger\n        by at least 1 byte than the content to decompress.\n        If an application uses `LZ4_compress_destSize()`,\n        it's highly recommended to update liblz4 to v1.9.2 or better.\n        If this can't be done or ensured,\n        the receiving decompression function should provide\n        a dstCapacity which is > decompressedSize, by at least 1 byte.\n        See https://github.com/lz4/lz4/issues/859 for details"]
    pub fn LZ4_compress_destSize(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSizePtr: *mut ::std::os::raw::c_int,
        targetDstSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decompress_safe_partial() :\n  Decompress an LZ4 compressed block, of size 'srcSize' at position 'src',\n  into destination buffer 'dst' of size 'dstCapacity'.\n  Up to 'targetOutputSize' bytes will be decoded.\n  The function stops decoding on reaching this objective.\n  This can be useful to boost performance\n  whenever only the beginning of a block is required.\n\n @return : the number of bytes decoded in `dst` (necessarily <= targetOutputSize)\n           If source stream is detected malformed, function returns a negative result.\n\n  Note 1 : @return can be < targetOutputSize, if compressed block contains less data.\n\n  Note 2 : targetOutputSize must be <= dstCapacity\n\n  Note 3 : this function effectively stops decoding on reaching targetOutputSize,\n           so dstCapacity is kind of redundant.\n           This is because in older versions of this function,\n           decoding operation would still write complete sequences.\n           Therefore, there was no guarantee that it would stop writing at exactly targetOutputSize,\n           it could write more bytes, though only up to dstCapacity.\n           Some \"margin\" used to be required for this operation to work properly.\n           Thankfully, this is no longer necessary.\n           The function nonetheless keeps the same signature, in an effort to preserve API compatibility.\n\n  Note 4 : If srcSize is the exact size of the block,\n           then targetOutputSize can be any value,\n           including larger than the block's decompressed size.\n           The function will, at most, generate block's decompressed size.\n\n  Note 5 : If srcSize is _larger_ than block's compressed size,\n           then targetOutputSize **MUST** be <= block's decompressed size.\n           Otherwise, *silent corruption will occur*."]
    pub fn LZ4_decompress_safe_partial(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        targetOutputSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type LZ4_stream_t = LZ4_stream_u;
extern "C" {
    pub fn LZ4_createStream() -> *mut LZ4_stream_t;
}
extern "C" {
    pub fn LZ4_freeStream(streamPtr: *mut LZ4_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_resetStream_fast() : v1.9.0+\n  Use this to prepare an LZ4_stream_t for a new chain of dependent blocks\n  (e.g., LZ4_compress_fast_continue()).\n\n  An LZ4_stream_t must be initialized once before usage.\n  This is automatically done when created by LZ4_createStream().\n  However, should the LZ4_stream_t be simply declared on stack (for example),\n  it's necessary to initialize it first, using LZ4_initStream().\n\n  After init, start any new stream with LZ4_resetStream_fast().\n  A same LZ4_stream_t can be re-used multiple times consecutively\n  and compress multiple streams,\n  provided that it starts each new stream with LZ4_resetStream_fast().\n\n  LZ4_resetStream_fast() is much faster than LZ4_initStream(),\n  but is not compatible with memory regions containing garbage data.\n\n  Note: it's only useful to call LZ4_resetStream_fast()\n        in the context of streaming compression.\n        The *extState* functions perform their own resets.\n        Invoking LZ4_resetStream_fast() before is redundant, and even counterproductive."]
    pub fn LZ4_resetStream_fast(streamPtr: *mut LZ4_stream_t);
}
extern "C" {
    #[doc = " LZ4_loadDict() :\n  Use this function to reference a static dictionary into LZ4_stream_t.\n  The dictionary must remain available during compression.\n  LZ4_loadDict() triggers a reset, so any previous data will be forgotten.\n  The same dictionary will have to be loaded on decompression side for successful decoding.\n  Dictionary are useful for better compression of small data (KB range).\n  While LZ4 accept any input as dictionary,\n  results are generally better when using Zstandard's Dictionary Builder.\n  Loading a size of 0 is allowed, and is the same as reset.\n @return : loaded dictionary size, in bytes (necessarily <= 64 KB)"]
    pub fn LZ4_loadDict(
        streamPtr: *mut LZ4_stream_t,
        dictionary: *const ::std::os::raw::c_char,
        dictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_compress_fast_continue() :\n  Compress 'src' content using data from previously compressed blocks, for better compression ratio.\n 'dst' buffer must be already allocated.\n  If dstCapacity >= LZ4_compressBound(srcSize), compression is guaranteed to succeed, and runs faster.\n\n @return : size of compressed block\n           or 0 if there is an error (typically, cannot fit into 'dst').\n\n  Note 1 : Each invocation to LZ4_compress_fast_continue() generates a new block.\n           Each block has precise boundaries.\n           Each block must be decompressed separately, calling LZ4_decompress_*() with relevant metadata.\n           It's not possible to append blocks together and expect a single invocation of LZ4_decompress_*() to decompress them together.\n\n  Note 2 : The previous 64KB of source data is __assumed__ to remain present, unmodified, at same address in memory !\n\n  Note 3 : When input is structured as a double-buffer, each buffer can have any size, including < 64 KB.\n           Make sure that buffers are separated, by at least one byte.\n           This construction ensures that each block only depends on previous block.\n\n  Note 4 : If input buffer is a ring-buffer, it can have any size, including < 64 KB.\n\n  Note 5 : After an error, the stream status is undefined (invalid), it can only be reset or freed."]
    pub fn LZ4_compress_fast_continue(
        streamPtr: *mut LZ4_stream_t,
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
        acceleration: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_saveDict() :\n  If last 64KB data cannot be guaranteed to remain available at its current memory location,\n  save it into a safer place (char* safeBuffer).\n  This is schematically equivalent to a memcpy() followed by LZ4_loadDict(),\n  but is much faster, because LZ4_saveDict() doesn't need to rebuild tables.\n @return : saved dictionary size in bytes (necessarily <= maxDictSize), or 0 if error."]
    pub fn LZ4_saveDict(
        streamPtr: *mut LZ4_stream_t,
        safeBuffer: *mut ::std::os::raw::c_char,
        maxDictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type LZ4_streamDecode_t = LZ4_streamDecode_u;
extern "C" {
    pub fn LZ4_createStreamDecode() -> *mut LZ4_streamDecode_t;
}
extern "C" {
    pub fn LZ4_freeStreamDecode(LZ4_stream: *mut LZ4_streamDecode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_setStreamDecode() :\n  An LZ4_streamDecode_t context can be allocated once and re-used multiple times.\n  Use this function to start decompression of a new stream of blocks.\n  A dictionary can optionally be set. Use NULL or size 0 for a reset order.\n  Dictionary is presumed stable : it must remain accessible and unmodified during next decompression.\n @return : 1 if OK, 0 if error"]
    pub fn LZ4_setStreamDecode(
        LZ4_streamDecode: *mut LZ4_streamDecode_t,
        dictionary: *const ::std::os::raw::c_char,
        dictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decoderRingBufferSize() : v1.8.2+\n  Note : in a ring buffer scenario (optional),\n  blocks are presumed decompressed next to each other\n  up to the moment there is not enough remaining space for next block (remainingSize < maxBlockSize),\n  at which stage it resumes from beginning of ring buffer.\n  When setting such a ring buffer for streaming decompression,\n  provides the minimum size of this ring buffer\n  to be compatible with any source respecting maxBlockSize condition.\n @return : minimum ring buffer size,\n           or 0 if there is an error (invalid maxBlockSize)."]
    pub fn LZ4_decoderRingBufferSize(maxBlockSize: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decompress_safe_continue() :\n  This decoding function allows decompression of consecutive blocks in \"streaming\" mode.\n  The difference with the usual independent blocks is that\n  new blocks are allowed to find references into former blocks.\n  A block is an unsplittable entity, and must be presented entirely to the decompression function.\n  LZ4_decompress_safe_continue() only accepts one block at a time.\n  It's modeled after `LZ4_decompress_safe()` and behaves similarly.\n\n @LZ4_streamDecode : decompression state, tracking the position in memory of past data\n @compressedSize : exact complete size of one compressed block.\n @dstCapacity : size of destination buffer (which must be already allocated),\n                must be an upper bound of decompressed size.\n @return : number of bytes decompressed into destination buffer (necessarily <= dstCapacity)\n           If destination buffer is not large enough, decoding will stop and output an error code (negative value).\n           If the source stream is detected malformed, the function will stop decoding and return a negative result.\n\n  The last 64KB of previously decoded data *must* remain available and unmodified\n  at the memory position where they were previously decoded.\n  If less than 64KB of data has been decoded, all the data must be present.\n\n  Special : if decompression side sets a ring buffer, it must respect one of the following conditions :\n  - Decompression buffer size is _at least_ LZ4_decoderRingBufferSize(maxBlockSize).\n    maxBlockSize is the maximum size of any single block. It can have any value > 16 bytes.\n    In which case, encoding and decoding buffers do not need to be synchronized.\n    Actually, data can be produced by any source compliant with LZ4 format specification, and respecting maxBlockSize.\n  - Synchronized mode :\n    Decompression buffer size is _exactly_ the same as compression buffer size,\n    and follows exactly same update rule (block boundaries at same positions),\n    and decoding function is provided with exact decompressed size of each block (exception for last block of the stream),\n    _then_ decoding & encoding ring buffer can have any size, including small ones ( < 64 KB).\n  - Decompression buffer is larger than encoding buffer, by a minimum of maxBlockSize more bytes.\n    In which case, encoding and decoding buffers do not need to be synchronized,\n    and encoding ring buffer can have any size, including small ones ( < 64 KB).\n\n  Whenever these conditions are not possible,\n  save the last 64KB of decoded data into a safe buffer where it can't be modified during decompression,\n  then indicate where this data is saved using LZ4_setStreamDecode(), before decompressing next block."]
    pub fn LZ4_decompress_safe_continue(
        LZ4_streamDecode: *mut LZ4_streamDecode_t,
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decompress_safe_usingDict() :\n  Works the same as\n  a combination of LZ4_setStreamDecode() followed by LZ4_decompress_safe_continue()\n  However, it's stateless: it doesn't need any LZ4_streamDecode_t state.\n  Dictionary is presumed stable : it must remain accessible and unmodified during decompression.\n  Performance tip : Decompression speed can be substantially increased\n                    when dst == dictStart + dictSize."]
    pub fn LZ4_decompress_safe_usingDict(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        dstCapacity: ::std::os::raw::c_int,
        dictStart: *const ::std::os::raw::c_char,
        dictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_decompress_safe_partial_usingDict() :\n  Behaves the same as LZ4_decompress_safe_partial()\n  with the added ability to specify a memory segment for past data.\n  Performance tip : Decompression speed can be substantially increased\n                    when dst == dictStart + dictSize."]
    pub fn LZ4_decompress_safe_partial_usingDict(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        compressedSize: ::std::os::raw::c_int,
        targetOutputSize: ::std::os::raw::c_int,
        maxOutputSize: ::std::os::raw::c_int,
        dictStart: *const ::std::os::raw::c_char,
        dictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type LZ4_byte = u8;
pub type LZ4_u32 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ4_stream_t_internal {
    pub hashTable: [LZ4_u32; 4096usize],
    pub dictionary: *const LZ4_byte,
    pub dictCtx: *const LZ4_stream_t_internal,
    pub currentOffset: LZ4_u32,
    pub tableType: LZ4_u32,
    pub dictSize: LZ4_u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union LZ4_stream_u {
    pub minStateSize: [::std::os::raw::c_char; 16416usize],
    pub internal_donotuse: LZ4_stream_t_internal,
}
extern "C" {
    #[doc = " LZ4_initStream() : v1.9.0+\n  An LZ4_stream_t structure must be initialized at least once.\n  This is automatically done when invoking LZ4_createStream(),\n  but it's not when the structure is simply declared on stack (for example).\n\n  Use LZ4_initStream() to properly initialize a newly declared LZ4_stream_t.\n  It can also initialize any arbitrary buffer of sufficient size,\n  and will @return a pointer of proper type upon initialization.\n\n  Note : initialization fails if size and alignment conditions are not respected.\n         In which case, the function will @return NULL.\n  Note2: An LZ4_stream_t structure guarantees correct alignment and size.\n  Note3: Before v1.9.0, use LZ4_resetStream() instead"]
    pub fn LZ4_initStream(buffer: *mut ::std::os::raw::c_void, size: usize) -> *mut LZ4_stream_t;
}
#[doc = " LZ4_streamDecode_t :\n  Never ever use below internal definitions directly !\n  These definitions are not API/ABI safe, and may change in future versions.\n  If you need static allocation, declare or allocate an LZ4_streamDecode_t object."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LZ4_streamDecode_t_internal {
    pub externalDict: *const LZ4_byte,
    pub prefixEnd: *const LZ4_byte,
    pub extDictSize: usize,
    pub prefixSize: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union LZ4_streamDecode_u {
    pub minStateSize: [::std::os::raw::c_char; 32usize],
    pub internal_donotuse: LZ4_streamDecode_t_internal,
}
extern "C" {
    #[doc = " Obsolete compression functions (since v1.7.3)"]
    pub fn LZ4_compress(
        src: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_limitedOutput(
        src: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        srcSize: ::std::os::raw::c_int,
        maxOutputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_withState(
        state: *mut ::std::os::raw::c_void,
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        inputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_limitedOutput_withState(
        state: *mut ::std::os::raw::c_void,
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        inputSize: ::std::os::raw::c_int,
        maxOutputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_continue(
        LZ4_streamPtr: *mut LZ4_stream_t,
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        inputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_compress_limitedOutput_continue(
        LZ4_streamPtr: *mut LZ4_stream_t,
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        inputSize: ::std::os::raw::c_int,
        maxOutputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Obsolete decompression functions (since v1.8.0)"]
    pub fn LZ4_uncompress(
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        outputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_uncompress_unknownOutputSize(
        source: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
        isize_: ::std::os::raw::c_int,
        maxOutputSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_create(inputBuffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn LZ4_sizeofStreamState() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_resetStreamState(
        state: *mut ::std::os::raw::c_void,
        inputBuffer: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_slideInputBuffer(state: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Obsolete streaming decoding functions (since v1.7.0)"]
    pub fn LZ4_decompress_safe_withPrefix64k(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        compressedSize: ::std::os::raw::c_int,
        maxDstSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_decompress_fast_withPrefix64k(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        originalSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Obsolete LZ4_decompress_fast variants (since v1.9.0) :\n  These functions used to be faster than LZ4_decompress_safe(),\n  but this is no longer the case. They are now slower.\n  This is because LZ4_decompress_fast() doesn't know the input size,\n  and therefore must progress more cautiously into the input buffer to not read beyond the end of block.\n  On top of that `LZ4_decompress_fast()` is not protected vs malformed or malicious inputs, making it a security liability.\n  As a consequence, LZ4_decompress_fast() is strongly discouraged, and deprecated.\n\n  The last remaining LZ4_decompress_fast() specificity is that\n  it can decompress a block without knowing its compressed size.\n  Such functionality can be achieved in a more secure manner\n  by employing LZ4_decompress_safe_partial().\n\n  Parameters:\n  originalSize : is the uncompressed size to regenerate.\n                 `dst` must be already allocated, its size must be >= 'originalSize' bytes.\n @return : number of bytes read from source buffer (== compressed size).\n           The function expects to finish at block's end exactly.\n           If the source stream is detected malformed, the function stops decoding and returns a negative result.\n  note : LZ4_decompress_fast*() requires originalSize. Thanks to this information, it never writes past the output buffer.\n         However, since it doesn't know its 'src' size, it may read an unknown amount of input, past input buffer bounds.\n         Also, since match offsets are not validated, match reads from 'src' may underflow too.\n         These issues never happen if input (compressed) data is correct.\n         But they may happen if input data is invalid (error or intentional tampering).\n         As a consequence, use these functions in trusted environments with trusted data **only**."]
    pub fn LZ4_decompress_fast(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        originalSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_decompress_fast_continue(
        LZ4_streamDecode: *mut LZ4_streamDecode_t,
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        originalSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LZ4_decompress_fast_usingDict(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        originalSize: ::std::os::raw::c_int,
        dictStart: *const ::std::os::raw::c_char,
        dictSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " LZ4_resetStream() :\n  An LZ4_stream_t structure must be initialized at least once.\n  This is done with LZ4_initStream(), or LZ4_resetStream().\n  Consider switching to LZ4_initStream(),\n  invoking LZ4_resetStream() will trigger deprecation warnings in the future."]
    pub fn LZ4_resetStream(streamPtr: *mut LZ4_stream_t);
}
