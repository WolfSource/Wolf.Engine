/*
	Project			 : Wolf Engine. Copyright(c) Pooya Eimandar (https://PooyaEimandar.github.io) . All rights reserved.
	Source			 : Please direct any bug to https://github.com/WolfEngine/Wolf.Engine/issues
	Website			 : https://WolfEngine.App
	Name			 : wolf.h
	Description		 : The main header of wolf.engine
	Comment          :
*/

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __WOLF_H__
#define __WOLF_H__

#include <w_ireleasable.h>
#include "w_system_pch.h"
#include "w_game_time.h"

#ifdef _WIN32

#define WOLF_MAIN()																		            \
int APIENTRY WinMain(HINSTANCE pHInstance, HINSTANCE pPrevHInstance, PSTR pSTR, int pCmdshow)

#elif defined(__linux) || defined(__APPLE__)

#define WOLF_MAIN()		                                                                            \
int main(int pArgc, const char * pArgv[])

#endif

#endif //__WOLF_H__


