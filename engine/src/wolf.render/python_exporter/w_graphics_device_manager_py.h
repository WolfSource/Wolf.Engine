/*
    Project          : Wolf Engine. Copyright(c) Pooya Eimandar (http://PooyaEimandar.com) . All rights reserved.
    Source           : Please direct any bug to https://github.com/PooyaEimandar/Wolf.Engine/issues
    Website          : http://WolfSource.io
    Name             : w_graphics_device_manager_py.h
    Description      : The python exporter for w_graphics_device_manager class and structs
    Comment          :
 */

#ifdef __PYTHON__

#ifndef __W_GRAPHICS_DEVICE_MANAGER_PY_H__
#define __W_GRAPHICS_DEVICE_MANAGER_PY_H__

namespace pywolf
{
	//it will be declared in w_graphics_device_manager.cpp
	static std::vector<std::shared_ptr<wolf::graphics::w_graphics_device>>  py_graphics_devices;

	static void w_graphics_device_manager_py_export()
	{
		using namespace boost::python;
		using namespace wolf::graphics;

		//export w_graphics_device_manager_configs class
		class_<w_graphics_device_manager_configs>("w_graphics_device_manager_configs", init<>())
			.def_readwrite("debug_gpu", &w_graphics_device_manager_configs::debug_gpu, "debug_gpu")
			.def_readwrite("off_screen_mode", &w_graphics_device_manager_configs::off_screen_mode, "off_screen_mode")
			;

		//export w_viewport class
		class_<w_viewport>("w_viewport", init<>())
			.def_readwrite("x", &w_viewport::x, "x")
			.def_readwrite("y", &w_viewport::y, "y")
			.def_readwrite("width", &w_viewport::width, "width")
			.def_readwrite("height", &w_viewport::height, "height")
			.def_readwrite("minDepth", &w_viewport::minDepth, "minDepth")
			.def_readwrite("maxDepth", &w_viewport::maxDepth, "maxDepth")
			;

		//export w_viewport_scissor class
		class_<w_viewport_scissor>("w_viewport_scissor", init<>())
			.add_property("offset", &w_viewport_scissor::py_get_offset, &w_viewport_scissor::py_set_offset, "offset")
			.add_property("extent", &w_viewport_scissor::py_get_extent, &w_viewport_scissor::py_set_extent, "extent")
			;

		//define w_format enum
		enum_<w_format>("w_format")
			.value("W_FORMAT_UNDEFINED", w_format::W_FORMAT_UNDEFINED)
			.value("W_FORMAT_R4G4_UNORM_PACK8", w_format::W_FORMAT_R4G4_UNORM_PACK8)
			.value("W_FORMAT_R4G4B4A4_UNORM_PACK16", w_format::W_FORMAT_R4G4B4A4_UNORM_PACK16)
			.value("W_FORMAT_B4G4R4A4_UNORM_PACK16", w_format::W_FORMAT_B4G4R4A4_UNORM_PACK16)
			.value("W_FORMAT_R5G6B5_UNORM_PACK16", w_format::W_FORMAT_R5G6B5_UNORM_PACK16)
			.value("W_FORMAT_B5G6R5_UNORM_PACK16", w_format::W_FORMAT_B5G6R5_UNORM_PACK16)
			.value("W_FORMAT_R5G5B5A1_UNORM_PACK16", w_format::W_FORMAT_R5G5B5A1_UNORM_PACK16)
			.value("W_FORMAT_B5G5R5A1_UNORM_PACK16", w_format::W_FORMAT_B5G5R5A1_UNORM_PACK16)
			.value("W_FORMAT_A1R5G5B5_UNORM_PACK16", w_format::W_FORMAT_A1R5G5B5_UNORM_PACK16)
			.value("W_FORMAT_R8_UNORM", w_format::W_FORMAT_R8_UNORM)
			.value("W_FORMAT_R8_SNORM", w_format::W_FORMAT_R8_SNORM)
			.value("W_FORMAT_R8_USCALED", w_format::W_FORMAT_R8_USCALED)
			.value("W_FORMAT_R8_SSCALED", w_format::W_FORMAT_R8_SSCALED)
			.value("W_FORMAT_R8_UINT", w_format::W_FORMAT_R8_UINT)
			.value("W_FORMAT_R8_SINT", w_format::W_FORMAT_R8_SINT)
			.value("W_FORMAT_R8_SRGB", w_format::W_FORMAT_R8_SRGB)
			.value("W_FORMAT_R8G8_UNORM", w_format::W_FORMAT_R8G8_UNORM)
			.value("W_FORMAT_R8G8_SNORM", w_format::W_FORMAT_R8G8_SNORM)
			.value("W_FORMAT_R8G8_USCALED", w_format::W_FORMAT_R8G8_USCALED)
			.value("W_FORMAT_R8G8_SSCALED", w_format::W_FORMAT_R8G8_SSCALED)
			.value("W_FORMAT_R8G8_UINT", w_format::W_FORMAT_R8G8_UINT)
			.value("W_FORMAT_R8G8_SINT", w_format::W_FORMAT_R8G8_SINT)
			.value("W_FORMAT_R8G8_SRGB", w_format::W_FORMAT_R8G8_SRGB)
			.value("W_FORMAT_R8G8B8_UNORM", w_format::W_FORMAT_R8G8B8_UNORM)
			.value("W_FORMAT_R8G8B8_SNORM", w_format::W_FORMAT_R8G8B8_SNORM)
			.value("W_FORMAT_R8G8B8_USCALED", w_format::W_FORMAT_R8G8B8_USCALED)
			.value("W_FORMAT_R8G8B8_SSCALED", w_format::W_FORMAT_R8G8B8_SSCALED)
			.value("W_FORMAT_R8G8B8_UINT", w_format::W_FORMAT_R8G8B8_UINT)
			.value("W_FORMAT_R8G8B8_SINT", w_format::W_FORMAT_R8G8B8_SINT)
			.value("W_FORMAT_R8G8B8_SRGB", w_format::W_FORMAT_R8G8B8_SRGB)
			.value("W_FORMAT_B8G8R8_UNORM", w_format::W_FORMAT_B8G8R8_UNORM)
			.value("W_FORMAT_B8G8R8_SNORM", w_format::W_FORMAT_B8G8R8_SNORM)
			.value("W_FORMAT_B8G8R8_USCALED", w_format::W_FORMAT_B8G8R8_USCALED)
			.value("W_FORMAT_B8G8R8_SSCALED", w_format::W_FORMAT_B8G8R8_SSCALED)
			.value("W_FORMAT_B8G8R8_UINT", w_format::W_FORMAT_B8G8R8_UINT)
			.value("W_FORMAT_B8G8R8_SINT", w_format::W_FORMAT_B8G8R8_SINT)
			.value("W_FORMAT_B8G8R8_SRGB", w_format::W_FORMAT_B8G8R8_SRGB)
			.value("W_FORMAT_R8G8B8A8_UNORM", w_format::W_FORMAT_R8G8B8A8_UNORM)
			.value("W_FORMAT_R8G8B8A8_SNORM", w_format::W_FORMAT_R8G8B8A8_SNORM)
			.value("W_FORMAT_R8G8B8A8_USCALED", w_format::W_FORMAT_R8G8B8A8_USCALED)
			.value("W_FORMAT_R8G8B8A8_SSCALED", w_format::W_FORMAT_R8G8B8A8_SSCALED)
			.value("W_FORMAT_R8G8B8A8_UINT", w_format::W_FORMAT_R8G8B8A8_UINT)
			.value("W_FORMAT_R8G8B8A8_SINT", w_format::W_FORMAT_R8G8B8A8_SINT)
			.value("W_FORMAT_R8G8B8A8_SRGB", w_format::W_FORMAT_R8G8B8A8_SRGB)
			.value("W_FORMAT_B8G8R8A8_UNORM", w_format::W_FORMAT_B8G8R8A8_UNORM)
			.value("W_FORMAT_B8G8R8A8_SNORM", w_format::W_FORMAT_B8G8R8A8_SNORM)
			.value("W_FORMAT_B8G8R8A8_USCALED", w_format::W_FORMAT_B8G8R8A8_USCALED)
			.value("W_FORMAT_B8G8R8A8_SSCALED", w_format::W_FORMAT_B8G8R8A8_SSCALED)
			.value("W_FORMAT_B8G8R8A8_UINT", w_format::W_FORMAT_B8G8R8A8_UINT)
			.value("W_FORMAT_B8G8R8A8_SINT", w_format::W_FORMAT_B8G8R8A8_SINT)
			.value("W_FORMAT_B8G8R8A8_SRGB", w_format::W_FORMAT_B8G8R8A8_SRGB)
			.value("W_FORMAT_A8B8G8R8_UNORM_PACK32", w_format::W_FORMAT_A8B8G8R8_UNORM_PACK32)
			.value("W_FORMAT_A8B8G8R8_SNORM_PACK32", w_format::W_FORMAT_A8B8G8R8_SNORM_PACK32)
			.value("W_FORMAT_A8B8G8R8_USCALED_PACK32", w_format::W_FORMAT_A8B8G8R8_USCALED_PACK32)
			.value("W_FORMAT_A8B8G8R8_SSCALED_PACK32", w_format::W_FORMAT_A8B8G8R8_SSCALED_PACK32)
			.value("W_FORMAT_A8B8G8R8_UINT_PACK32", w_format::W_FORMAT_A8B8G8R8_UINT_PACK32)
			.value("W_FORMAT_A8B8G8R8_SINT_PACK32", w_format::W_FORMAT_A8B8G8R8_SINT_PACK32)
			.value("W_FORMAT_A8B8G8R8_SRGB_PACK32", w_format::W_FORMAT_A8B8G8R8_SRGB_PACK32)
			.value("W_FORMAT_A2R10G10B10_UNORM_PACK32", w_format::W_FORMAT_A2R10G10B10_UNORM_PACK32)
			.value("W_FORMAT_A2R10G10B10_SNORM_PACK32", w_format::W_FORMAT_A2R10G10B10_SNORM_PACK32)
			.value("W_FORMAT_A2R10G10B10_USCALED_PACK32", w_format::W_FORMAT_A2R10G10B10_USCALED_PACK32)
			.value("W_FORMAT_A2R10G10B10_SSCALED_PACK32", w_format::W_FORMAT_A2R10G10B10_SSCALED_PACK32)
			.value("W_FORMAT_A2R10G10B10_UINT_PACK32", w_format::W_FORMAT_A2R10G10B10_UINT_PACK32)
			.value("W_FORMAT_A2R10G10B10_SINT_PACK32", w_format::W_FORMAT_A2R10G10B10_SINT_PACK32)
			.value("W_FORMAT_A2B10G10R10_UNORM_PACK32", w_format::W_FORMAT_A2B10G10R10_UNORM_PACK32)
			.value("W_FORMAT_A2B10G10R10_SNORM_PACK32", w_format::W_FORMAT_A2B10G10R10_SNORM_PACK32)
			.value("W_FORMAT_A2B10G10R10_USCALED_PACK32", w_format::W_FORMAT_A2B10G10R10_USCALED_PACK32)
			.value("W_FORMAT_A2B10G10R10_SSCALED_PACK32", w_format::W_FORMAT_A2B10G10R10_SSCALED_PACK32)
			.value("W_FORMAT_A2B10G10R10_UINT_PACK32", w_format::W_FORMAT_A2B10G10R10_UINT_PACK32)
			.value("W_FORMAT_A2B10G10R10_SINT_PACK32", w_format::W_FORMAT_A2B10G10R10_SINT_PACK32)
			.value("W_FORMAT_R16_UNORM", w_format::W_FORMAT_R16_UNORM)
			.value("W_FORMAT_R16_SNORM", w_format::W_FORMAT_R16_SNORM)
			.value("W_FORMAT_R16_USCALED", w_format::W_FORMAT_R16_USCALED)
			.value("W_FORMAT_R16_SSCALED", w_format::W_FORMAT_R16_SSCALED)
			.value("W_FORMAT_R16_UINT", w_format::W_FORMAT_R16_UINT)
			.value("W_FORMAT_R16_SINT", w_format::W_FORMAT_R16_SINT)
			.value("W_FORMAT_R16_SFLOAT", w_format::W_FORMAT_R16_SFLOAT)
			.value("W_FORMAT_R16G16_UNORM", w_format::W_FORMAT_R16G16_UNORM)
			.value("W_FORMAT_R16G16_SNORM", w_format::W_FORMAT_R16G16_SNORM)
			.value("W_FORMAT_R16G16_USCALED", w_format::W_FORMAT_R16G16_USCALED)
			.value("W_FORMAT_R16G16_SSCALED", w_format::W_FORMAT_R16G16_SSCALED)
			.value("W_FORMAT_R16G16_UINT", w_format::W_FORMAT_R16G16_UINT)
			.value("W_FORMAT_R16G16_SINT", w_format::W_FORMAT_R16G16_SINT)
			.value("W_FORMAT_R16G16_SFLOAT", w_format::W_FORMAT_R16G16_SFLOAT)
			.value("W_FORMAT_R16G16B16_UNORM", w_format::W_FORMAT_R16G16B16_UNORM)
			.value("W_FORMAT_R16G16B16_SNORM", w_format::W_FORMAT_R16G16B16_SNORM)
			.value("W_FORMAT_R16G16B16_USCALED", w_format::W_FORMAT_R16G16B16_USCALED)
			.value("W_FORMAT_R16G16B16_SSCALED", w_format::W_FORMAT_R16G16B16_SSCALED)
			.value("W_FORMAT_R16G16B16_UINT", w_format::W_FORMAT_R16G16B16_UINT)
			.value("W_FORMAT_R16G16B16_SINT", w_format::W_FORMAT_R16G16B16_SINT)
			.value("W_FORMAT_R16G16B16_SFLOAT", w_format::W_FORMAT_R16G16B16_SFLOAT)
			.value("W_FORMAT_R16G16B16A16_UNORM", w_format::W_FORMAT_R16G16B16A16_UNORM)
			.value("W_FORMAT_R16G16B16A16_SNORM", w_format::W_FORMAT_R16G16B16A16_SNORM)
			.value("W_FORMAT_R16G16B16A16_USCALED", w_format::W_FORMAT_R16G16B16A16_USCALED)
			.value("W_FORMAT_R16G16B16A16_SSCALED", w_format::W_FORMAT_R16G16B16A16_SSCALED)
			.value("W_FORMAT_R16G16B16A16_UINT", w_format::W_FORMAT_R16G16B16A16_UINT)
			.value("W_FORMAT_R16G16B16A16_SINT", w_format::W_FORMAT_R16G16B16A16_SINT)
			.value("W_FORMAT_R16G16B16A16_SFLOAT", w_format::W_FORMAT_R16G16B16A16_SFLOAT)
			.value("W_FORMAT_R32_UINT", w_format::W_FORMAT_R32_UINT)
			.value("W_FORMAT_R32_SINT", w_format::W_FORMAT_R32_SINT)
			.value("W_FORMAT_R32_SFLOAT", w_format::W_FORMAT_R32_SFLOAT)
			.value("W_FORMAT_R32G32_UINT", w_format::W_FORMAT_R32G32_UINT)
			.value("W_FORMAT_R32G32_SINT", w_format::W_FORMAT_R32G32_SINT)
			.value("W_FORMAT_R32G32_SFLOAT", w_format::W_FORMAT_R32G32_SFLOAT)
			.value("W_FORMAT_R32G32B32_UINT", w_format::W_FORMAT_R32G32B32_UINT)
			.value("W_FORMAT_R32G32B32_SINT", w_format::W_FORMAT_R32G32B32_SINT)
			.value("W_FORMAT_R32G32B32_SFLOAT", w_format::W_FORMAT_R32G32B32_SFLOAT)
			.value("W_FORMAT_R32G32B32A32_UINT", w_format::W_FORMAT_R32G32B32A32_UINT)
			.value("W_FORMAT_R32G32B32A32_SINT", w_format::W_FORMAT_R32G32B32A32_SINT)
			.value("W_FORMAT_R32G32B32A32_SFLOAT", w_format::W_FORMAT_R32G32B32A32_SFLOAT)
			.value("W_FORMAT_R64_UINT", w_format::W_FORMAT_R64_UINT)
			.value("W_FORMAT_R64_SINT", w_format::W_FORMAT_R64_SINT)
			.value("W_FORMAT_R64_SFLOAT", w_format::W_FORMAT_R64_SFLOAT)
			.value("W_FORMAT_R64G64_UINT", w_format::W_FORMAT_R64G64_UINT)
			.value("W_FORMAT_R64G64_SINT", w_format::W_FORMAT_R64G64_SINT)
			.value("W_FORMAT_R64G64_SFLOAT", w_format::W_FORMAT_R64G64_SFLOAT)
			.value("W_FORMAT_R64G64B64_UINT", w_format::W_FORMAT_R64G64B64_UINT)
			.value("W_FORMAT_R64G64B64_SINT", w_format::W_FORMAT_R64G64B64_SINT)
			.value("W_FORMAT_R64G64B64_SFLOAT", w_format::W_FORMAT_R64G64B64_SFLOAT)
			.value("W_FORMAT_R64G64B64A64_UINT", w_format::W_FORMAT_R64G64B64A64_UINT)
			.value("W_FORMAT_R64G64B64A64_SINT", w_format::W_FORMAT_R64G64B64A64_SINT)
			.value("W_FORMAT_R64G64B64A64_SFLOAT", w_format::W_FORMAT_R64G64B64A64_SFLOAT)
			.value("W_FORMAT_B10G11R11_UFLOAT_PACK32", w_format::W_FORMAT_B10G11R11_UFLOAT_PACK32)
			.value("W_FORMAT_E5B9G9R9_UFLOAT_PACK32", w_format::W_FORMAT_E5B9G9R9_UFLOAT_PACK32)
			.value("W_FORMAT_D16_UNORM", w_format::W_FORMAT_D16_UNORM)
			.value("W_FORMAT_X8_D24_UNORM_PACK32", w_format::W_FORMAT_X8_D24_UNORM_PACK32)
			.value("W_FORMAT_D32_SFLOAT", w_format::W_FORMAT_D32_SFLOAT)
			.value("W_FORMAT_S8_UINT", w_format::W_FORMAT_S8_UINT)
			.value("W_FORMAT_D16_UNORM_S8_UINT", w_format::W_FORMAT_D16_UNORM_S8_UINT)
			.value("W_FORMAT_D24_UNORM_S8_UINT", w_format::W_FORMAT_D24_UNORM_S8_UINT)
			.value("W_FORMAT_D32_SFLOAT_S8_UINT", w_format::W_FORMAT_D32_SFLOAT_S8_UINT)
			.value("W_FORMAT_BC1_RGB_UNORM_BLOCK", w_format::W_FORMAT_BC1_RGB_UNORM_BLOCK)
			.value("W_FORMAT_BC1_RGB_SRGB_BLOCK", w_format::W_FORMAT_BC1_RGB_SRGB_BLOCK)
			.value("W_FORMAT_BC1_RGBA_UNORM_BLOCK", w_format::W_FORMAT_BC1_RGBA_UNORM_BLOCK)
			.value("W_FORMAT_BC1_RGBA_SRGB_BLOCK", w_format::W_FORMAT_BC1_RGBA_SRGB_BLOCK)
			.value("W_FORMAT_BC2_UNORM_BLOCK", w_format::W_FORMAT_BC2_UNORM_BLOCK)
			.value("W_FORMAT_BC2_SRGB_BLOCK", w_format::W_FORMAT_BC2_SRGB_BLOCK)
			.value("W_FORMAT_BC3_UNORM_BLOCK", w_format::W_FORMAT_BC3_UNORM_BLOCK)
			.value("W_FORMAT_BC3_SRGB_BLOCK", w_format::W_FORMAT_BC3_SRGB_BLOCK)
			.value("W_FORMAT_BC4_UNORM_BLOCK", w_format::W_FORMAT_BC4_UNORM_BLOCK)
			.value("W_FORMAT_BC5_UNORM_BLOCK", w_format::W_FORMAT_BC5_UNORM_BLOCK)
			.value("W_FORMAT_BC5_SNORM_BLOCK", w_format::W_FORMAT_BC5_SNORM_BLOCK)
			.value("W_FORMAT_BC6H_UFLOAT_BLOCK", w_format::W_FORMAT_BC6H_UFLOAT_BLOCK)
			.value("W_FORMAT_BC6H_SFLOAT_BLOCK", w_format::W_FORMAT_BC6H_SFLOAT_BLOCK)
			.value("W_FORMAT_BC7_UNORM_BLOCK", w_format::W_FORMAT_BC7_UNORM_BLOCK)
			.value("W_FORMAT_BC7_SRGB_BLOCK", w_format::W_FORMAT_BC7_SRGB_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8_UNORM_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8_UNORM_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8_SRGB_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8_SRGB_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK)
			.value("W_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK", w_format::W_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK)
			.value("W_FORMAT_EAC_R11_UNORM_BLOCK", w_format::W_FORMAT_EAC_R11_UNORM_BLOCK)
			.value("W_FORMAT_EAC_R11_SNORM_BLOCK", w_format::W_FORMAT_EAC_R11_SNORM_BLOCK)
			.value("W_FORMAT_EAC_R11G11_UNORM_BLOCK", w_format::W_FORMAT_EAC_R11G11_UNORM_BLOCK)
			.value("W_FORMAT_EAC_R11G11_SNORM_BLOCK", w_format::W_FORMAT_EAC_R11G11_SNORM_BLOCK)
			.value("W_FORMAT_ASTC_4x4_UNORM_BLOCK", w_format::W_FORMAT_ASTC_4x4_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_4x4_SRGB_BLOCK", w_format::W_FORMAT_ASTC_4x4_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_5x4_UNORM_BLOCK", w_format::W_FORMAT_ASTC_5x4_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_5x4_SRGB_BLOCK", w_format::W_FORMAT_ASTC_5x4_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_5x5_UNORM_BLOCK", w_format::W_FORMAT_ASTC_5x5_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_5x5_SRGB_BLOCK", w_format::W_FORMAT_ASTC_5x5_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_6x5_UNORM_BLOCK", w_format::W_FORMAT_ASTC_6x5_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_6x5_SRGB_BLOCK", w_format::W_FORMAT_ASTC_6x5_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_6x6_UNORM_BLOCK", w_format::W_FORMAT_ASTC_6x6_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_6x6_SRGB_BLOCK", w_format::W_FORMAT_ASTC_6x6_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_8x5_UNORM_BLOCK", w_format::W_FORMAT_ASTC_8x5_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_8x5_SRGB_BLOCK", w_format::W_FORMAT_ASTC_8x5_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_8x6_UNORM_BLOCK", w_format::W_FORMAT_ASTC_8x6_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_8x6_SRGB_BLOCK", w_format::W_FORMAT_ASTC_8x6_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_8x8_UNORM_BLOCK", w_format::W_FORMAT_ASTC_8x8_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_8x8_SRGB_BLOCK", w_format::W_FORMAT_ASTC_8x8_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_10x5_UNORM_BLOCK", w_format::W_FORMAT_ASTC_10x5_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_10x5_SRGB_BLOCK", w_format::W_FORMAT_ASTC_10x5_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_10x6_UNORM_BLOCK", w_format::W_FORMAT_ASTC_10x6_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_10x6_SRGB_BLOCK", w_format::W_FORMAT_ASTC_10x6_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_10x8_UNORM_BLOCK", w_format::W_FORMAT_ASTC_10x8_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_10x8_SRGB_BLOCK", w_format::W_FORMAT_ASTC_10x8_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_10x10_UNORM_BLOCK", w_format::W_FORMAT_ASTC_10x10_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_10x10_SRGB_BLOCK", w_format::W_FORMAT_ASTC_10x10_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_12x10_UNORM_BLOCK", w_format::W_FORMAT_ASTC_12x10_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_12x10_SRGB_BLOCK", w_format::W_FORMAT_ASTC_12x10_SRGB_BLOCK)
			.value("W_FORMAT_ASTC_12x12_UNORM_BLOCK", w_format::W_FORMAT_ASTC_12x12_UNORM_BLOCK)
			.value("W_FORMAT_ASTC_12x12_SRGB_BLOCK", w_format::W_FORMAT_ASTC_12x12_SRGB_BLOCK)
			.value("W_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG", w_format::W_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG)
			.value("W_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG", w_format::W_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG)
			.value("W_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG", w_format::W_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG)
			.value("W_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG", w_format::W_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG)
			.value("W_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG", w_format::W_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG)
			.value("W_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG", w_format::W_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG)
			.value("W_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG", w_format::W_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG)
			.value("W_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG", w_format::W_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG)
			.value("W_FORMAT_G8B8G8R8_422_UNORM_KHR", w_format::W_FORMAT_G8B8G8R8_422_UNORM_KHR)
			.value("W_FORMAT_B8G8R8G8_422_UNORM_KHR", w_format::W_FORMAT_B8G8R8G8_422_UNORM_KHR)
			.value("W_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR", w_format::W_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR)
			.value("W_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR", w_format::W_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR)
			.value("W_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR", w_format::W_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR)
			.value("W_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR", w_format::W_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR)
			.value("W_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR", w_format::W_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR)
			.value("W_FORMAT_R10X6_UNORM_PACK16_KHR", w_format::W_FORMAT_R10X6_UNORM_PACK16_KHR)
			.value("W_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR", w_format::W_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR)
			.value("W_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR", w_format::W_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR)
			.value("W_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR", w_format::W_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR)
			.value("W_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR", w_format::W_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR)
			.value("W_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR", w_format::W_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR", w_format::W_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR", w_format::W_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR", w_format::W_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR", w_format::W_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR)
			.value("W_FORMAT_R12X4_UNORM_PACK16_KHR", w_format::W_FORMAT_R12X4_UNORM_PACK16_KHR)
			.value("W_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR", w_format::W_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR)
			.value("W_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR", w_format::W_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR)
			.value("W_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR", w_format::W_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR)
			.value("W_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR", w_format::W_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR)
			.value("W_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR", w_format::W_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR", w_format::W_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR", w_format::W_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR", w_format::W_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR", w_format::W_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR)
			.value("W_FORMAT_G16B16G16R16_422_UNORM_KHR", w_format::W_FORMAT_G16B16G16R16_422_UNORM_KHR)
			.value("W_FORMAT_B16G16R16G16_422_UNORM_KHR", w_format::W_FORMAT_B16G16R16G16_422_UNORM_KHR)
			.value("W_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR", w_format::W_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR)
			.value("W_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR", w_format::W_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR)
			.value("W_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR", w_format::W_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR)
			.value("W_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR", w_format::W_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR)
			.value("W_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR", w_format::W_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR)
			.export_values()
			;
	}
}

#endif//__W_GAME_PY_H__

#endif//__PYTHON__
