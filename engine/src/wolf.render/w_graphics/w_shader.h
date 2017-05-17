/*
	Project			 : Wolf Engine. Copyright(c) Pooya Eimandar (http://PooyaEimandar.com) . All rights reserved.
	Source			 : Please direct any bug to https://github.com/PooyaEimandar/Wolf.Engine/issues
	Website			 : http://WolfSource.io
	Name			 : w_shader.h
	Description		 : A class which is responsible to manage shaders for materials
	Comment          :
*/

#ifndef __W_SHADER_H__
#define __W_SHADER_H__

#include "w_graphics_device_manager.h"
#include "w_texture.h"

enum w_shader_binding_type
{
    SAMPLER = 0,
    UNIFORM
};

enum w_shader_stage
{
	VERTEX_SHADER = VK_SHADER_STAGE_VERTEX_BIT,
#if defined(__DX12__) || defined(__DX11__)
	PIXEL_SHADER,
        HULL_SHADER,
	DOMAIN_SHADER,
#elif defined(__VULKAN__)
        FRAGMENT_SHADER = VK_SHADER_STAGE_FRAGMENT_BIT,
        TESSELATION_CONTROL,
        TESSELATION_EVALUATION,
#endif
	GEOMETRY_SHADER,

	COMPUTE_SHADER,
    ALL_STAGES
};

struct w_shader_binding_param
{
    uint32_t                    index;
    w_shader_binding_type       type;
    w_shader_stage              stage;
#ifdef  __VULKAN__
    VkDescriptorBufferInfo      uniform_info;
    VkDescriptorImageInfo       sampler_info;
#endif //  __VULKAN__
};

namespace wolf
{
	namespace graphics
	{
        class w_shader_pimp;
		class w_shader : public system::w_object
		{
		public:
			W_EXP w_shader();
			W_EXP virtual ~w_shader();

			//Create shader from binary file
			W_EXP HRESULT load(_In_ const std::shared_ptr<w_graphics_device>& pGDevice,
							   _In_z_ const std::wstring& pShaderBinaryPath,
				               _In_ const w_shader_stage pShaderStage,
                               _In_z_ const char* pMainFunctionName = "main");
            
        
			W_EXP virtual ULONG release() override;

#pragma region Getters

            W_EXP const std::vector<w_shader_binding_param> get_shader_binding_params() const;
            W_EXP const std::vector<VkPipelineShaderStageCreateInfo>* get_shader_stages() const;
            W_EXP const VkDescriptorSet get_descriptor_set() const;
            W_EXP const VkDescriptorSetLayout get_descriptor_set_layout_binding() const;
            
#pragma endregion

#pragma region Setters

            W_EXP HRESULT set_shader_binding_params(_In_ std::vector<w_shader_binding_param> pShaderBindingParams);
            
#pragma endregion

            W_EXP static HRESULT load_to_shared_shaders(_In_ const std::shared_ptr<w_graphics_device>& pGDevice,
                                                        _In_z_ const std::string& pName,
                                                        _In_z_ const std::wstring& pVertexShaderPath,
                                                        _In_z_ const std::wstring& pFragmentShaderPath,
                                                        _In_ const std::vector<w_shader_binding_param> pShaderBindingParams,
                                                        _Inout_ w_shader** pShader,
                                                        _In_z_ const char* pMainFunctionName = "main");

            W_EXP static w_shader* get_shader_from_shared(_In_z_ const std::string& pName);
            W_EXP static ULONG release_shared_shaders();

		private:
            typedef	system::w_object                                _super;
            w_shader_pimp*                                          _pimp;

            static std::map<std::string, w_shader*>                 _shared;
		};
	}
}

#endif
