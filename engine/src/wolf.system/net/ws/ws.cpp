#include "ws.h"
#include "uws.hpp"

ws ws_init()
{
    try
    {
      return reinterpret_cast<ws>(new uws());
    }
    catch (...)
    {
      return nullptr;
    }
}

W_RESULT ws_run(_In_ ws pWS,
                _In_ const bool pSSL,
                _In_z_ const char* pCertFilePath,
                _In_z_ const char* pPrivateKeyFilePath,
                _In_z_ const char* pPassPhrase,
                _In_z_ const char* pRoot,
                _In_ const int pPort,
                _In_ const int pCompression,
                _In_ const int pMaxPayloadLength,
                _In_ const int pIdleTimeout,
                _In_ const int pMaxBackPressure,
                _In_ ws_on_listened_fn pOnListened,
                _In_ ws_on_opened_fn pOnOpened,
                _In_ ws_on_message_fn pOnMessage,
                _In_ ws_on_closed_fn pOnClosed)
{
    W_RESULT _rt;
    try
    {
        reinterpret_cast<uws*>(pWS)->run(pSSL,
                                         pCertFilePath,
                                         pPrivateKeyFilePath,
                                         pPassPhrase,
                                         pRoot,
                                         pPort,
                                         static_cast<uWS::CompressOptions>(pCompression),
                                         pMaxPayloadLength,
                                         pIdleTimeout,
                                         pMaxBackPressure,
                                         pOnListened,
                                         pOnOpened,
                                         pOnMessage,
                                         pOnClosed);
        _rt = W_SUCCESS;
    }
    catch (...)
    {
        _rt = W_FAILURE;
    }
    return _rt;
}

void ws_free(ws pWS)
{
    if (pWS)
    {
        try
        {
            delete reinterpret_cast<uws*>(pWS);
        }
        catch( ...)
        {
        }
    }
}