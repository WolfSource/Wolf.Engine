#[repr(C)]
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum WError {
    #[error("no error")]
    None,
    #[error("the memory was not initialized")]
    MemoryWasNotInitialized,
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("could not allocate memory")]
    OutOfMemory,
    #[error("could not cast to int")]
    IntCastError,
    #[error("could not cast to usize or isize")]
    SizeCastError,
    #[cfg(feature = "media_ffmpeg")]
    #[error("invalid AVCodec")]
    MediaCodecNotFound,
    #[cfg(feature = "media_ffmpeg")]
    #[error("invalid media URL")]
    MediaUrlInvalid,
    #[cfg(feature = "media_ffmpeg")]
    #[error("media does not have stream info")]
    MediaStreamInfoNotFound,
    #[cfg(feature = "media_ffmpeg")]
    #[error("invalid video frame size")]
    MediaInvalidVideoFrameSize,
    #[cfg(feature = "media_ffmpeg")]
    #[error("invalid av config")]
    MediaInvalidAvConfig,
    #[cfg(feature = "media_ffmpeg")]
    #[error("could not fill image")]
    MediaImageFillFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("could not create SWS context")]
    MediaInvalidSwsContext,
    #[cfg(feature = "media_ffmpeg")]
    #[error("could not scale via SWS context")]
    MediaSwsScaleFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("invalid AvSetOption")]
    MediaInvalidAvSetOption,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg could not send frame while encoding")]
    MediaEncodeSendFrameFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg could not receive frame while encoding")]
    MediaEncodeReceivePacketFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg could not send frame while decoding")]
    MediaDecodeSendFrameFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg could not receive frame while decoding")]
    MediaDecodeReceivePacketFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg initializing parser was failed")]
    MediaInitParserFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg parser failed")]
    MediaParserFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg was failed to open AvCodec")]
    MediaAvCodecOpenFailed,
    #[cfg(feature = "media_ffmpeg")]
    #[error("ffmpeg could not allocate memory for AvFormatContext")]
    MediaAvFormatContextAllocFailed,
    #[cfg(any(
        feature = "system_socket_client",
        feature = "system_socket_server",
        feature = "stream_quic"
    ))]
    #[error("could not parse the address")]
    SystemSocketAddressParseFailed,
    #[cfg(any(
        feature = "system_socket_client",
        feature = "system_socket_server",
        feature = "stream_quic"
    ))]
    #[error("could not create poll")]
    SystemPollFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("socket could not connect to the address")]
    SystemSocketConnectFailed,
    #[cfg(any(
        feature = "system_socket_client",
        feature = "system_socket_server",
        feature = "stream_quic"
    ))]
    #[error("could not bind to the socket")]
    SystemSocketBindFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not create boring SSL builder")]
    SystemSocketSSLBuilderFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not acceot ssl socket stream")]
    SystemSocketSSLAcceptFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not connect to the ssl socket stream")]
    SystemSocketSSLConnectFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not read from the tcp socket")]
    SystemTCPSocketReadFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not write to the tcp socket")]
    SystemTCPSocketWriteFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not read from the udp socket")]
    SystemUDPSocketReadFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not write to the udp socket")]
    SystemUDPSocketWriteFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not connect to the server")]
    SystemSocketConnectTimeout,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not handshake to the server")]
    SystemSocketSSLHandshakeTimeout,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not load local address")]
    SystemSocketLoadLocalAddressFailed,
    #[cfg(any(
        feature = "system_socket_client",
        feature = "system_socket_server",
        feature = "stream_quic"
    ))]
    #[error("could not register the poll")]
    SystemPollRegisterFailed,
    #[cfg(any(feature = "system_socket_client", feature = "system_socket_server"))]
    #[error("could not convert from UTF8")]
    SystemSocketUtf8Error,
    #[cfg(feature = "system_lz4")]
    #[error("invalid source size")]
    SystemLZ4InvalidSourceSize,
    #[cfg(feature = "system_lz4")]
    #[error("invalid destination size")]
    SystemLZ4InvalidDestinationSize,
    #[cfg(feature = "stream_grpc")]
    #[error("TLS certificate file was not found for Grpc")]
    StreamGrpcTlsCrtNotFound,
    #[cfg(feature = "stream_grpc")]
    #[error("TLS key file was not found for Grpc")]
    StreamGrpcTlsKeyNotFound,
    #[cfg(feature = "stream_grpc")]
    #[error("Address parser failed")]
    StreamGrpcAddrressParseFailed,
    #[cfg(feature = "stream_grpc")]
    #[error("Grpc Server could not build")]
    StreamGrpcServerBuildFailed,
    #[cfg(feature = "stream_grpc")]
    #[error("Invalid Grpc endpoint")]
    StreamGrpcServerInvalidUri,
    #[cfg(feature = "stream_grpc")]
    #[error("Grpc endpoint with invalid TLS config")]
    StreamGrpcInvalidTlsConfig,
    #[cfg(feature = "stream_grpc")]
    #[error("Grpc endpoint missing TLS config")]
    StreamGrpcMissingTlsConfig,
    #[cfg(feature = "stream_grpc")]
    #[error("Grpc endpoint could not create a channel")]
    StreamGrpcChannelError,
    #[error("unknown error")]
    Unknown,
}
