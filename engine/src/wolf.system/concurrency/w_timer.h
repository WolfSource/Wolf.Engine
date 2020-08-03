/*
    Project          : Wolf Engine. Copyright(c) Pooya Eimandar (https://PooyaEimandar.github.io) . All rights reserved.
    Source           : Please direct any bug to https://github.com/WolfEngine/Wolf.Engine/issues
    Website          : https://WolfEngine.App
    Name             : w_timer.h
    Description      : a timer based on libev
    Comment          : https://metacpan.org/pod/distribution/EV/libev/ev.pod
*/

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include "wolf.h"
#include "libev/ev.h"

typedef ev_timer w_timer_base;
typedef ev_periodic w_timer_base_periodic;
typedef struct ev_loop w_timer_loop;
typedef void w_timer_callback(w_timer_loop*, w_timer_base*, int /*revents*/);
typedef void w_timer_periodic_callback(w_timer_loop*, w_timer_base_periodic*, int /*revents*/);
typedef double w_timer_periodic_scheduler_callback(w_timer_base_periodic*, double /*now*/);

typedef struct
{
    w_timer_base*               t;
    w_timer_loop*               l;
} w_timer;

typedef struct
{
    w_timer_base_periodic*      t;
    w_timer_loop*               l;
} w_timer_periodic;

/**
 * create a timer
 * @param pStartAfterSec time before start in seconds
 * @param pTimeOutInSec timeout in seconds
 * @param pCallBack callback function
 * @return pointer to timer
*/
W_SYSTEM_EXPORT
w_timer* w_timer_init(_In_ double pStartAfterSec,
                      _In_ double pTimeOutInSec,
                      _In_ w_timer_callback pCallBack);

/**
 * restart a timer
 * @param pTimer pointer to timer
*/
W_SYSTEM_EXPORT
void w_timer_restart(_In_ w_timer* pTimer);

/**
 * create a periodic timer
 * @param pStartAfterSec time before start in seconds
 * @param pIntervalInSec interval in seconds
 * @param pCallBack callback function
 * @param pSchedulerCallBack scheduler callback
 * @return pointer to timer
*/
W_SYSTEM_EXPORT
w_timer_periodic* w_timer_init_periodic(_In_ double pStartAfterSec,
                                        _In_ double pIntervalInSec,
                                        _In_ w_timer_periodic_callback pCallBack,
                                        _In_ w_timer_periodic_scheduler_callback pSchedulerCallBack);

/**
 * release a timer
 * @param pTimer timer to release
*/
W_SYSTEM_EXPORT
void w_timer_free(_In_ w_timer* pTimer);

/**
 * release a timer periodic
 * @param pTimer timer periodic to release
*/
W_SYSTEM_EXPORT
void w_timer_periodic_free(_In_ w_timer_periodic* pTimer);

#ifdef __cplusplus
}
#endif