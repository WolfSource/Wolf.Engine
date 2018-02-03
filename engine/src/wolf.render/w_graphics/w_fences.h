/*
	Project			 : Wolf Engine. Copyright(c) Pooya Eimandar (http://PooyaEimandar.com) . All rights reserved.
	Source			 : Please direct any bug to https://github.com/PooyaEimandar/Wolf.Engine/issues
	Website			 : http://WolfSource.io
	Name			 : w_fences.h
	Description		 : Fence for graphics device
	Comment          :
*/

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __W_FENCES_H__
#define __W_FENCES_H__

#include "w_graphics_device_manager.h"

namespace wolf
{
	namespace graphics
	{
        class w_fences
        {
		public:
            //initialize fence
            W_EXP HRESULT initialize(_In_ const std::shared_ptr<w_graphics_device>& pGDevice, _In_ const uint32_t pNumberOfFences = 1);
            //wait for all fence
            W_EXP HRESULT wait(_In_ uint64_t pTimeOut = VK_TIMEOUT);
            //reset all fences
            W_EXP HRESULT reset();
            //get pointer to the first fence
            W_EXP VkFence* get();
            //get all fences
            W_EXP VkFence* get_all();
            //get number of fences
            W_EXP uint32_t get_count();
            //release resources of fence
            W_EXP ULONG release();
        private:
            
            std::shared_ptr<w_graphics_device> _gDevice;

#ifdef __VULKAN__
            std::vector<VkFence>        _fences;
#elif defined(__DX12__)
            ComPtr<ID3D12Fence>
#endif
        };
	}
}

#include "python_exporter/w_fences_py.h"

#endif
