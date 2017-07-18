use ::sys;
use super::*;

use std::slice;

pub struct InstanceCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkInstanceCreateInfo, pub(crate) PhantomData<&'static I>);
pub struct DeviceCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkDeviceCreateInfo, pub(crate) PhantomData<&'static I>);
pub struct DeviceQueueCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkDeviceQueueCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct SubmitInfos<'a, I: Impl>(pub(crate) &'a [sys::VkSubmitInfo], pub(crate) PhantomData<&'static I>);

pub struct MemoryAllocateInfo<'a, I: Impl>(pub(crate) &'a sys::VkMemoryAllocateInfo, pub(crate) PhantomData<&'static I>);
pub struct MappedMemoryRanges<'a, I: Impl>(pub(crate) &'a [sys::VkMappedMemoryRange], pub(crate) PhantomData<&'static I>);

pub struct BindSparseInfos<'a, I: Impl>(pub(crate) &'a [sys::VkBindSparseInfo], pub(crate) PhantomData<&'static I>);

pub struct BufferCreateInfo<'a, I:Impl>(pub(crate) &'a sys::VkBufferCreateInfo, pub(crate) PhantomData<&'static I>);
pub struct BufferViewCreateInfo<'a, I:Impl>(pub(crate) &'a sys::VkBufferViewCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct ImageCreateInfo<'a, I:Impl>(pub(crate) &'a sys::VkImageCreateInfo, pub(crate) PhantomData<&'static I>);
pub struct ImageViewCreateInfo<'a, I:Impl>(pub(crate) &'a sys::VkImageViewCreateInfo, pub(crate) PhantomData<&'static I>);
pub struct ImageSubresource<'a, I:Impl>(pub(crate) &'a sys::VkImageSubresource, pub(crate) PhantomData<&'static I>);

pub struct ShaderModuleCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkShaderModuleCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct GraphicsPipelineCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkGraphicsPipelineCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct PipelineCacheCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkPipelineCacheCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct FenceCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkFenceCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct SemaphoreCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkSemaphoreCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct EventCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkEventCreateInfo, pub(crate) PhantomData<&'static I>);

pub struct QueryPoolCreateInfo<'a, I: Impl>(pub(crate) &'a sys::VkQueryPoolCreateInfo, pub(crate) PhantomData<&'static I>);

impl<'a, I: Impl> DeviceCreateInfo<'a, I> {
    pub fn queues(&self) -> impl ExactSizeIterator<Item = DeviceQueueCreateInfo<'a, I>> {
        let slice = unsafe { slice::from_raw_parts(self.0.pQueueCreateInfos, self.0.queueCreateInfoCount as usize) };
        slice.iter().map(|x| DeviceQueueCreateInfo(x, PhantomData))
    }
}

impl<'a, I: Impl> DeviceQueueCreateInfo<'a, I> {
    pub fn family_index(&self) -> u32 { self.0.queueFamilyIndex }
    pub fn count(&self) -> u32 { self.0.queueCount }
}

macro_rules! vulkan_map_type {
    (VkBool32) => { bool };
    (VkFormatFeatureFlags) => {FormatFeatureFlags};
    (VkDeviceSize) => { u64 };
    (u32) => { u32 };
    (VkSparseImageFormatProperties) => { SparseImageFormatProperties };
    (VkImageAspectFlags) => {ImageAspectFlags};
    (VkExtent3D) => {Extent3D};
    (VkSparseImageFormatFlags) => {SparseImageFormatFlags};
}

macro_rules! vulkan_map_value_with_type {
    (VkBool32 => $x:expr) => { $x == sys::VK_TRUE };
    (VkBool32 <= $x:expr) => { if $x { sys::VK_TRUE } else { sys::VK_FALSE } };
    (VkFormatFeatureFlags => $x:expr) => { FormatFeatureFlags::from_bits_truncate($x) };
    (VkFormatFeatureFlags <= $x:expr) => { $x.bits() };
    (VkDeviceSize => $x:expr) => { $x };
    (VkDeviceSize <= $x:expr) => { $x };
    (u32 => $x:expr) => { $x };
    (u32 <= $x:expr) => { $x };
    (VkSparseImageFormatProperties => $x:expr) => { $x.into() };
    (VkSparseImageFormatProperties <= $x:expr) => { $x.into() };
    (VkImageAspectFlags => $x:expr) => { ImageAspectFlags::from_bits_truncate($x) };
    (VkImageAspectFlags <= $x:expr) => { $x.bits() };
    (VkExtent3D => $x:expr) => { $x.into() };
    (VkExtent3D <= $x:expr) => { $x.into() };
    (VkSparseImageFormatFlags => $x:expr) => { SparseImageFormatFlags::from_bits_truncate($x) };
    (VkSparseImageFormatFlags <= $x:expr) => { $x.bits() };
}

macro_rules! vulkan_struct_correspondence {
    (struct $name:ident => $vk_name:ident {
        $($member_name:ident => $vk_member_name:ident : $vk_type:ident ,)*
    }) => {
        pub struct $name {
            $(
                $member_name: vulkan_map_type!($vk_type),
            )*
        }

        impl From<sys::$vk_name> for $name {
            fn from(raw: sys::$vk_name) -> $name {
                $name {
                    $(
                        $member_name: vulkan_map_value_with_type! { $vk_type => raw.$vk_member_name },
                    )*
                }
            }
        }

        impl From<$name> for sys::$vk_name {
            fn from(raw: $name) -> sys::$vk_name {
                sys::$vk_name {
                    $(
                        $vk_member_name: vulkan_map_value_with_type! { $vk_type <= raw.$member_name },
                    )*
                }
            }
        }
    }
}

macro_rules! vulkan_enum_correspondence {
    (enum $name:ident => $vk_name:ident {
        $($case:ident => $vk_case_name:ident,)*
    }) => {
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($case,)*
            Other(i32),
        }

        impl From<sys::$vk_name> for $name {
            fn from(raw: sys::$vk_name) -> $name {
                match raw {
                    $(sys::$vk_name::$vk_case_name => $name::$case,)*
                    other => $name::Other(other as i32),
                }
            }
        }

        impl From<$name> for sys::$vk_name {
            fn from(raw: $name) -> sys::$vk_name {
                match raw {
                    $($name::$case => sys::$vk_name::$vk_case_name,)*
                    $name::Other(other) => unsafe { ::std::mem::transmute(other) },
                }
            }
        }
    }
}

vulkan_struct_correspondence! {
    struct MemoryRequirements => VkMemoryRequirements {
        size => size: VkDeviceSize,
        alignment => alignment: VkDeviceSize,
        memory_types => memoryTypeBits: u32,
    }
}

// TODO: snake case
vulkan_struct_correspondence! {
    struct PhysicalDeviceFeatures => VkPhysicalDeviceFeatures {
        robust_buffer_access => robustBufferAccess: VkBool32,
        full_draw_index_uint32 => fullDrawIndexUint32: VkBool32,
        image_cube_array => imageCubeArray: VkBool32,
        independent_blend => independentBlend: VkBool32,
        geometry_shader => geometryShader: VkBool32,
        tessellation_shader => tessellationShader: VkBool32,
        sample_rate_shading => sampleRateShading: VkBool32,
        dual_src_blend => dualSrcBlend: VkBool32,
        login_op => logicOp: VkBool32,
        multi_draw_indirect => multiDrawIndirect: VkBool32,
        draw_indirect_first_instance => drawIndirectFirstInstance: VkBool32,
        depth_clamp => depthClamp: VkBool32,
        depth_bias_clamp => depthBiasClamp: VkBool32,
        fill_mode_non_solid => fillModeNonSolid: VkBool32,
        depth_bounds => depthBounds: VkBool32,
        wide_linex => wideLines: VkBool32,
        large_points => largePoints: VkBool32,
        alpha_to_one => alphaToOne: VkBool32,
        multi_viewport => multiViewport: VkBool32,
        sampler_anisotropy => samplerAnisotropy: VkBool32,
        texture_compression_etc2 => textureCompressionETC2: VkBool32,
        texture_compression_astc_ldr => textureCompressionASTC_LDR: VkBool32,
        texture_compression_bc => textureCompressionBC: VkBool32,
        occlusion_query_precise => occlusionQueryPrecise: VkBool32,
        pipeline_statistics_query => pipelineStatisticsQuery: VkBool32,
        vertex_pipeline_stores_and_atomics => vertexPipelineStoresAndAtomics: VkBool32,
        fragment_stores_and_atomics => fragmentStoresAndAtomics: VkBool32,
        shader_tesselation_and_geometry_point_size => shaderTessellationAndGeometryPointSize: VkBool32,
        shader_image_gather_extended => shaderImageGatherExtended: VkBool32,
        shader_storage_image_extended_formats => shaderStorageImageExtendedFormats: VkBool32,
        shader_storage_image_multisample => shaderStorageImageMultisample: VkBool32,
        shader_storage_image_read_without_format => shaderStorageImageReadWithoutFormat: VkBool32,
        shader_storage_image_write_without_format => shaderStorageImageWriteWithoutFormat: VkBool32,
        shader_uniform_buffer_array_dynamic_indexing => shaderUniformBufferArrayDynamicIndexing: VkBool32,
        shader_sampled_image_array_dynamic_indexing => shaderSampledImageArrayDynamicIndexing: VkBool32,
        shader_storage_buffer_array_dynamic_indexing => shaderStorageBufferArrayDynamicIndexing: VkBool32,
        shader_storage_image_array_dynamic_indexing => shaderStorageImageArrayDynamicIndexing: VkBool32,
        shader_clip_distance => shaderClipDistance: VkBool32,
        shader_cull_distance => shaderCullDistance: VkBool32,
        shader_float64 => shaderFloat64: VkBool32,
        shader_int64 => shaderInt64: VkBool32,
        shader_int16 => shaderInt16: VkBool32,
        shader_resource_residency => shaderResourceResidency: VkBool32,
        shader_resources_min_lod => shaderResourceMinLod: VkBool32,
        sparse_binding => sparseBinding: VkBool32,
        sparse_residency_buffer => sparseResidencyBuffer: VkBool32,
        sparse_residency_image_2d => sparseResidencyImage2D: VkBool32,
        sparse_residency_image_3d => sparseResidencyImage3D: VkBool32,
        sparse_residency_2_samples => sparseResidency2Samples: VkBool32,
        sparse_residency_4_sample => sparseResidency4Samples: VkBool32,
        sparse_residency_8_sample => sparseResidency8Samples: VkBool32,
        sparse_residency_16_sample => sparseResidency16Samples: VkBool32,
        sparse_residency_aliased => sparseResidencyAliased: VkBool32,
        variable_multisample_rate => variableMultisampleRate: VkBool32,
        inherited_queries => inheritedQueries: VkBool32,
    }
}

vulkan_enum_correspondence! {
    enum Format => VkFormat {
        Unknown => VK_FORMAT_UNDEFINED,
        R4G4UnormPack8 => VK_FORMAT_R4G4_UNORM_PACK8,
        R4G4B4A4UnormPack16 => VK_FORMAT_R4G4B4A4_UNORM_PACK16,
        B4G4R4A4UnormPack16 => VK_FORMAT_B4G4R4A4_UNORM_PACK16,
        R5G6B5UnormPack16 => VK_FORMAT_R5G6B5_UNORM_PACK16,
        B5G6R5UnormPack16 => VK_FORMAT_B5G6R5_UNORM_PACK16,
        R5G5B5A1UnormPack16 => VK_FORMAT_R5G5B5A1_UNORM_PACK16,
        B5G5R5A1UnormPack16 => VK_FORMAT_B5G5R5A1_UNORM_PACK16,
        A1R5G5B5UnormPack16 => VK_FORMAT_A1R5G5B5_UNORM_PACK16,
        R8Unorm => VK_FORMAT_R8_UNORM,
        R8Snorm => VK_FORMAT_R8_SNORM,
        R8Uscaled => VK_FORMAT_R8_USCALED,
        R8Sscaled => VK_FORMAT_R8_SSCALED,
        R8Uint => VK_FORMAT_R8_UINT,
        R8Sint => VK_FORMAT_R8_SINT,
        R8Srgb => VK_FORMAT_R8_SRGB,
        R8G8Unorm => VK_FORMAT_R8G8_UNORM,
        R8G8Snorm => VK_FORMAT_R8G8_SNORM,
        R8G8Uscaled => VK_FORMAT_R8G8_USCALED,
        R8G8Sscaled => VK_FORMAT_R8G8_SSCALED,
        R8G8Uint => VK_FORMAT_R8G8_UINT,
        R8G8Sint => VK_FORMAT_R8G8_SINT,
        R8G8Srgb => VK_FORMAT_R8G8_SRGB,
        R8G8B8Unorm => VK_FORMAT_R8G8B8_UNORM,
        R8G8B8Snorm => VK_FORMAT_R8G8B8_SNORM,
        R8G8B8Uscaled => VK_FORMAT_R8G8B8_USCALED,
        R8G8B8Sscaled => VK_FORMAT_R8G8B8_SSCALED,
        R8G8B8Uint => VK_FORMAT_R8G8B8_UINT,
        R8G8B8Sint => VK_FORMAT_R8G8B8_SINT,
        R8G8B8Srgb => VK_FORMAT_R8G8B8_SRGB,
        B8G8R8Unorm => VK_FORMAT_B8G8R8_UNORM,
        B8G8R8Snorm => VK_FORMAT_B8G8R8_SNORM,
        B8G8R8Uscaled => VK_FORMAT_B8G8R8_USCALED,
        B8G8R8Sscaled => VK_FORMAT_B8G8R8_SSCALED,
        B8G8R8Uint => VK_FORMAT_B8G8R8_UINT,
        B8G8R8Sint => VK_FORMAT_B8G8R8_SINT,
        B8G8R8Srgb => VK_FORMAT_B8G8R8_SRGB,
        R8G8B8A8Unorm => VK_FORMAT_R8G8B8A8_UNORM,
        R8G8B8A8Snorm => VK_FORMAT_R8G8B8A8_SNORM,
        R8G8B8A8Uscaled => VK_FORMAT_R8G8B8A8_USCALED,
        R8G8B8A8Sscaled => VK_FORMAT_R8G8B8A8_SSCALED,
        R8G8B8A8Uint => VK_FORMAT_R8G8B8A8_UINT,
        R8G8B8A8Sint => VK_FORMAT_R8G8B8A8_SINT,
        R8G8B8A8Srgb => VK_FORMAT_R8G8B8A8_SRGB,
        B8G8R8A8Unorm => VK_FORMAT_B8G8R8A8_UNORM,
        B8G8R8A8Snorm => VK_FORMAT_B8G8R8A8_SNORM,
        B8G8R8A8Uscaled => VK_FORMAT_B8G8R8A8_USCALED,
        B8G8R8A8Sscaled => VK_FORMAT_B8G8R8A8_SSCALED,
        B8G8R8A8Uint => VK_FORMAT_B8G8R8A8_UINT,
        B8G8R8A8Sint => VK_FORMAT_B8G8R8A8_SINT,
        B8G8R8A8Srgb => VK_FORMAT_B8G8R8A8_SRGB,
        A8B8G8R8UnormPack32 => VK_FORMAT_A8B8G8R8_UNORM_PACK32,
        A8B8G8R8SnormPack32 => VK_FORMAT_A8B8G8R8_SNORM_PACK32,
        A8B8G8R8UscaledPack32 => VK_FORMAT_A8B8G8R8_USCALED_PACK32,
        A8B8G8R8SscaledPack32 => VK_FORMAT_A8B8G8R8_SSCALED_PACK32,
        A8B8G8R8UintPack32 => VK_FORMAT_A8B8G8R8_UINT_PACK32,
        A8B8G8R8SintPack32 => VK_FORMAT_A8B8G8R8_SINT_PACK32,
        A8B8G8R8SrgbPack32 => VK_FORMAT_A8B8G8R8_SRGB_PACK32,
        A2R10G10B10UnormPack32 => VK_FORMAT_A2R10G10B10_UNORM_PACK32,
        A2R10G10B10SnormPack32 => VK_FORMAT_A2R10G10B10_SNORM_PACK32,
        A2R10G10B10UscaledPack32 => VK_FORMAT_A2R10G10B10_USCALED_PACK32,
        A2R10G10B10SscaledPack32 => VK_FORMAT_A2R10G10B10_SSCALED_PACK32,
        A2R10G10B10UintPack32 => VK_FORMAT_A2R10G10B10_UINT_PACK32,
        A2R10G10B10SintPack32 => VK_FORMAT_A2R10G10B10_SINT_PACK32,
        A2B10G10R10UnormPack32 => VK_FORMAT_A2B10G10R10_UNORM_PACK32,
        A2B10G10R10SnormPack32 => VK_FORMAT_A2B10G10R10_SNORM_PACK32,
        A2B10G10R10UscaledPack32 => VK_FORMAT_A2B10G10R10_USCALED_PACK32,
        A2B10G10R10SscaledPack32 => VK_FORMAT_A2B10G10R10_SSCALED_PACK32,
        A2B10G10R10UintPack32 => VK_FORMAT_A2B10G10R10_UINT_PACK32,
        A2B10G10R10SintPack32 => VK_FORMAT_A2B10G10R10_SINT_PACK32,
        R16Unorm => VK_FORMAT_R16_UNORM,
        R16Snorm => VK_FORMAT_R16_SNORM,
        R16Uscaled => VK_FORMAT_R16_USCALED,
        R16Sscaled => VK_FORMAT_R16_SSCALED,
        R16Uint => VK_FORMAT_R16_UINT,
        R16Sint => VK_FORMAT_R16_SINT,
        R16Sfloat => VK_FORMAT_R16_SFLOAT,
        R16G16Unorm => VK_FORMAT_R16G16_UNORM,
        R16G16Snorm => VK_FORMAT_R16G16_SNORM,
        R16G16Uscaled => VK_FORMAT_R16G16_USCALED,
        R16G16Sscaled => VK_FORMAT_R16G16_SSCALED,
        R16G16Uint => VK_FORMAT_R16G16_UINT,
        R16G16Sint => VK_FORMAT_R16G16_SINT,
        R16G16Sfloat => VK_FORMAT_R16G16_SFLOAT,
        R16G16B16Unorm => VK_FORMAT_R16G16B16_UNORM,
        R16G16B16Snorm => VK_FORMAT_R16G16B16_SNORM,
        R16G16B16Uscaled => VK_FORMAT_R16G16B16_USCALED,
        R16G16B16Sscaled => VK_FORMAT_R16G16B16_SSCALED,
        R16G16B16Uint => VK_FORMAT_R16G16B16_UINT,
        R16G16B16Sint => VK_FORMAT_R16G16B16_SINT,
        R16G16B16Sfloat => VK_FORMAT_R16G16B16_SFLOAT,
        R16G16B16A16Unorm => VK_FORMAT_R16G16B16A16_UNORM,
        R16G16B16A16Snorm => VK_FORMAT_R16G16B16A16_SNORM,
        R16G16B16A16Uscaled => VK_FORMAT_R16G16B16A16_USCALED,
        R16G16B16A16Sscaled => VK_FORMAT_R16G16B16A16_SSCALED,
        R16G16B16A16Uint => VK_FORMAT_R16G16B16A16_UINT,
        R16G16B16A16Sint => VK_FORMAT_R16G16B16A16_SINT,
        R16G16B16A16Sfloat => VK_FORMAT_R16G16B16A16_SFLOAT,
        R32Uint => VK_FORMAT_R32_UINT,
        R32Sint => VK_FORMAT_R32_SINT,
        R32Sfloat => VK_FORMAT_R32_SFLOAT,
        R32G32Uint => VK_FORMAT_R32G32_UINT,
        R32G32Sint => VK_FORMAT_R32G32_SINT,
        R32G32Sfloat => VK_FORMAT_R32G32_SFLOAT,
        R32G32B32Uint => VK_FORMAT_R32G32B32_UINT,
        R32G32B32Sint => VK_FORMAT_R32G32B32_SINT,
        R32G32B32Sfloat => VK_FORMAT_R32G32B32_SFLOAT,
        R32G32B32A32Uint => VK_FORMAT_R32G32B32A32_UINT,
        R32G32B32A32Sint => VK_FORMAT_R32G32B32A32_SINT,
        R32G32B32A32Sfloat => VK_FORMAT_R32G32B32A32_SFLOAT,
        R64Uint => VK_FORMAT_R64_UINT,
        R64Sint => VK_FORMAT_R64_SINT,
        R64Sfloat => VK_FORMAT_R64_SFLOAT,
        R64G64Uint => VK_FORMAT_R64G64_UINT,
        R64G64Sint => VK_FORMAT_R64G64_SINT,
        R64G64Sfloat => VK_FORMAT_R64G64_SFLOAT,
        R64G64B64Uint => VK_FORMAT_R64G64B64_UINT,
        R64G64B64Sint => VK_FORMAT_R64G64B64_SINT,
        R64G64B64Sfloat => VK_FORMAT_R64G64B64_SFLOAT,
        R64G64B64A64Uint => VK_FORMAT_R64G64B64A64_UINT,
        R64G64B64A64Sint => VK_FORMAT_R64G64B64A64_SINT,
        R64G64B64A64Sfloat => VK_FORMAT_R64G64B64A64_SFLOAT,
        B10G11R11UfloatPack32 => VK_FORMAT_B10G11R11_UFLOAT_PACK32,
        E5B9G9R9UfloatPack32 => VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
        D16Unorm => VK_FORMAT_D16_UNORM,
        X8_D24UnormPack32 => VK_FORMAT_X8_D24_UNORM_PACK32,
        D32Sfloat => VK_FORMAT_D32_SFLOAT,
        S8Uint => VK_FORMAT_S8_UINT,
        D16Unorm_S8Uint => VK_FORMAT_D16_UNORM_S8_UINT,
        D24Unorm_S8Uint => VK_FORMAT_D24_UNORM_S8_UINT,
        D32Sfloat_S8Uint => VK_FORMAT_D32_SFLOAT_S8_UINT,
        BC1_RGBUnormBlock => VK_FORMAT_BC1_RGB_UNORM_BLOCK,
        BC1_RGBSrgbBlock => VK_FORMAT_BC1_RGB_SRGB_BLOCK,
        BC1_RGBAUnormBlock => VK_FORMAT_BC1_RGBA_UNORM_BLOCK,
        BC1_RGBASrgbBlock => VK_FORMAT_BC1_RGBA_SRGB_BLOCK,
        BC2UnormBlock => VK_FORMAT_BC2_UNORM_BLOCK,
        BC2SrgbBlock => VK_FORMAT_BC2_SRGB_BLOCK,
        BC3UnormBlock => VK_FORMAT_BC3_UNORM_BLOCK,
        BC3SrgbBlock => VK_FORMAT_BC3_SRGB_BLOCK,
        BC4UnormBlock => VK_FORMAT_BC4_UNORM_BLOCK,
        BC4SnormBlock => VK_FORMAT_BC4_SNORM_BLOCK,
        BC5UnormBlock => VK_FORMAT_BC5_UNORM_BLOCK,
        BC5SnormBlock => VK_FORMAT_BC5_SNORM_BLOCK,
        BC6HUfloatBlock => VK_FORMAT_BC6H_UFLOAT_BLOCK,
        BC6HSfloatBlock => VK_FORMAT_BC6H_SFLOAT_BLOCK,
        BC7UnormBlock => VK_FORMAT_BC7_UNORM_BLOCK,
        BC7SrgbBlock => VK_FORMAT_BC7_SRGB_BLOCK,
        ETC2_R8G8B8UnormBlock => VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
        ETC2_R8G8B8SrgbBlock => VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
        ETC2_R8G8B8A1UnormBlock => VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
        ETC2_R8G8B8A1SrgbBlock => VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
        ETC2_R8G8B8A8UnormBlock => VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
        ETC2_R8G8B8A8SrgbBlock => VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
        EAC_R11UnormBlock => VK_FORMAT_EAC_R11_UNORM_BLOCK,
        EAC_R11SnormBlock => VK_FORMAT_EAC_R11_SNORM_BLOCK,
        EAC_R11G11UnormBlock => VK_FORMAT_EAC_R11G11_UNORM_BLOCK,
        EAC_R11G11SnormBlock => VK_FORMAT_EAC_R11G11_SNORM_BLOCK,
        ASTC_4x4UnormBlock => VK_FORMAT_ASTC_4x4_UNORM_BLOCK,
        ASTC_4x4SrgbBlock => VK_FORMAT_ASTC_4x4_SRGB_BLOCK,
        ASTC_5x4UnormBlock => VK_FORMAT_ASTC_5x4_UNORM_BLOCK,
        ASTC_5x4SrgbBlock => VK_FORMAT_ASTC_5x4_SRGB_BLOCK,
        ASTC_5x5UnormBlock => VK_FORMAT_ASTC_5x5_UNORM_BLOCK,
        ASTC_5x5SrgbBlock => VK_FORMAT_ASTC_5x5_SRGB_BLOCK,
        ASTC_6x5UnormBlock => VK_FORMAT_ASTC_6x5_UNORM_BLOCK,
        ASTC_6x5SrgbBlock => VK_FORMAT_ASTC_6x5_SRGB_BLOCK,
        ASTC_6x6UnormBlock => VK_FORMAT_ASTC_6x6_UNORM_BLOCK,
        ASTC_6x6SrgbBlock => VK_FORMAT_ASTC_6x6_SRGB_BLOCK,
        ASTC_8x5UnormBlock => VK_FORMAT_ASTC_8x5_UNORM_BLOCK,
        ASTC_8x5SrgbBlock => VK_FORMAT_ASTC_8x5_SRGB_BLOCK,
        ASTC_8x6UnormBlock => VK_FORMAT_ASTC_8x6_UNORM_BLOCK,
        ASTC_8x6SrgbBlock => VK_FORMAT_ASTC_8x6_SRGB_BLOCK,
        ASTC_8x8UnormBlock => VK_FORMAT_ASTC_8x8_UNORM_BLOCK,
        ASTC_8x8SrgbBlock => VK_FORMAT_ASTC_8x8_SRGB_BLOCK,
        ASTC_10x5UnormBlock => VK_FORMAT_ASTC_10x5_UNORM_BLOCK,
        ASTC_10x5SrgbBlock => VK_FORMAT_ASTC_10x5_SRGB_BLOCK,
        ASTC_10x6UnormBlock => VK_FORMAT_ASTC_10x6_UNORM_BLOCK,
        ASTC_10x6SrgbBlock => VK_FORMAT_ASTC_10x6_SRGB_BLOCK,
        ASTC_10x8UnormBlock => VK_FORMAT_ASTC_10x8_UNORM_BLOCK,
        ASTC_10x8SrgbBlock => VK_FORMAT_ASTC_10x8_SRGB_BLOCK,
        ASTC_10x10UnormBlock => VK_FORMAT_ASTC_10x10_UNORM_BLOCK,
        ASTC_10x10SrgbBlock => VK_FORMAT_ASTC_10x10_SRGB_BLOCK,
        ASTC_12x10UnormBlock => VK_FORMAT_ASTC_12x10_UNORM_BLOCK,
        ASTC_12x10SrgbBlock => VK_FORMAT_ASTC_12x10_SRGB_BLOCK,
        ASTC_12x12UnormBlock => VK_FORMAT_ASTC_12x12_UNORM_BLOCK,
        ASTC_12x12SrgbBlock => VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
    }
}

vulkan_struct_correspondence! {
    struct FormatProperties => VkFormatProperties {
        linear_tiling_features => linearTilingFeatures: VkFormatFeatureFlags,
        optimal_tiling_features => optimalTilingFeatures: VkFormatFeatureFlags,
        buffer_features => bufferFeatures: VkFormatFeatureFlags,
    }
}

bitflags! {
    pub struct FormatFeatureFlags: u32 {
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001;
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002;
        const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004;
        const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008;
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010;
        const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020;
        const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040;
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080;
        const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100;
        const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200;
        const VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400;
        const VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800;
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000;
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000;
        const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR = 0x00004000;
        const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR = 0x00008000;
        const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = 0x00010000;
        const VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF;
    }
}

vulkan_struct_correspondence! {
    struct Extent3D => VkExtent3D {
        width => width: u32,
        height => height: u32,
        depth => depth: u32,
    }
}

bitflags! {
    pub struct MemoryMapFlags: u32 {
        const VK_MEMORY_MAP_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF;
    }
}

bitflags! {
    pub struct ImageAspectFlags: u32 {
        const VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001;
        const VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002;
        const VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004;
        const VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008;
        const VK_IMAGE_ASPECT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF;
    }
}

vulkan_struct_correspondence! {
    struct SubresourceLayout => VkSubresourceLayout {
        offset => offset: VkDeviceSize,
        size => size: VkDeviceSize,
        row_pitch => rowPitch: VkDeviceSize,
        array_pitch => arrayPitch: VkDeviceSize,
        depth_pitch => depthPitch: VkDeviceSize,
    }
}

bitflags! {
    pub struct SparseImageFormatFlags: u32 {
        const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001;
        const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002;
        const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004;
        const VK_SPARSE_IMAGE_FORMAT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF;
    }
}

vulkan_struct_correspondence! {
    struct SparseImageMemoryRequirements => VkSparseImageMemoryRequirements {
        format_properties => formatProperties: VkSparseImageFormatProperties,
        mip_tail_first_lod => imageMipTailFirstLod: u32,
        mip_tail_size => imageMipTailSize: VkDeviceSize,
        mip_tail_offset => imageMipTailOffset: VkDeviceSize,
        mip_tail_stride => imageMipTailStride: VkDeviceSize,
    }
}

vulkan_struct_correspondence! {
    struct SparseImageFormatProperties => VkSparseImageFormatProperties {
        aspect_mask => aspectMask: VkImageAspectFlags,
        image_granularity => imageGranularity: VkExtent3D,
        flags => flags: VkSparseImageFormatFlags,
    }
}

bitflags! {
    pub struct QueryResultFlags: u32 {
        const VK_QUERY_RESULT_64_BIT = 0x00000001;
        const VK_QUERY_RESULT_WAIT_BIT = 0x00000002;
        const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004;
        const VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008;
        const VK_QUERY_RESULT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum Error {
    NotReady = 1,
}

impl From<Error> for sys::VkResult {
    fn from(err: Error) -> sys::VkResult {
        unsafe {
            mem::transmute(err)
        }
    }
}

