#![allow(non_snake_case)]

use super::sys;

use std::borrow::{Borrow, BorrowMut};
use std::ops::Range;
use std::cmp;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;

mod data;
pub use self::data::*;

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait Impl: Sized + Sync + 'static {
    type Instance: Instance<Self>;
    type PhysicalDevice: PhysicalDevice<Self>;
    type Device: Device<Self>;
    type Queue: Queue<Self>;
    type CommandBuffer: CommandBuffer<Self>;

    type DeviceMemory: DeviceMemory<Self>;
    type Buffer: Buffer<Self>;
    type Image: Image<Self>;

    type Fence;
    
    type PipelineCache;
}

pub trait Instance<I: Impl>: Sized + Send + Sync + 'static {
    fn create(info: InstanceCreateInfo<I>) -> Result<Self>;

    fn physical_device_count(&self) -> Result<u32>;
    fn enumerate_physical_devices(&self, capacity: u32) -> Result<Vec<I::PhysicalDevice>>;
}

pub trait PhysicalDevice<I: Impl>: Sized + Send + Sync + 'static {
    fn get_features(&self) -> PhysicalDeviceFeatures;
    fn get_format_properties(&self, format: Format) -> FormatProperties;

    fn create_device(&self, create_info: DeviceCreateInfo<I>) -> Result<I::Device>;
}

pub trait Device<I: Impl>: Sized + Send + Sync + 'static {
    fn wait_idle(&self) -> Result<()>;

    fn allocate_memory(&self, info: MemoryAllocateInfo<I>) -> Result<I::DeviceMemory>;
    fn flush_mapped_memory_ranges(&self, ranges: MappedMemoryRanges<I>) -> Result<()>;
    fn invalidate_mapped_memory_ranges(&self, ranges: MappedMemoryRanges<I>) -> Result<()>;
}

pub trait Queue<I: Impl>: Sized + Send + Sync + 'static {
    fn submit(&mut self, submits: SubmitInfos<I>, fence: Option<&mut I::Fence>);
    fn wait_idle(&self) -> Result<()>;

    fn bind_sparse(&mut self, infos: BindSparseInfos<I>, fence: Option<&mut I::Fence>);
}

pub trait CommandBuffer<I: Impl>: Sized + Send + 'static {
}

pub trait DeviceMemory<I: Impl>: Sized + Send + Sync + 'static {
    fn map(&mut self, device: &I::Device, offset: u64, size: u64, flags: MemoryMapFlags) -> Result<*mut u8>;
    fn unmap(&mut self, device: &I::Device);

    fn get_memory_commitment(&self, device: &I::Device);

    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait Buffer<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: BufferCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn get_memory_requirements(&self, device: &I::Device) -> MemoryRequirements;
    fn bind_memory(&mut self, device: &I::Device, memory: &I::DeviceMemory, offset: u64) -> Result<()>;
}

pub trait BufferView<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: BufferViewCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait Image<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: ImageCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn get_memory_requirements(&self, device: &I::Device) -> MemoryRequirements;
    fn get_sparse_memory_requirements(&self, device: &I::Device, capacity: u32) -> Vec<SparseImageMemoryRequirements>;
    fn bind_memory(&mut self, device: &I::Device, memory: &I::DeviceMemory, offset: u64) -> Result<()>;

    fn get_subresource_layout(&self, subresource: ImageSubresource<I>) -> Result<SubresourceLayout>;
}

pub trait ImageView<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: ImageViewCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait ShaderModule<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: ShaderModuleCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait Pipeline<I: Impl>: Sized + Send + Sync + 'static {
    fn create_graphics(
        device: &I::Device, 
        pipeline_cache: Option<&I::PipelineCache>, 
        create_info: &[GraphicsPipelineCreateInfo<I>],
    ) -> Vec<Self>;
    fn create_compute(
        device: &I::Device, 
        pipeline_cache: Option<&I::PipelineCache>, 
        create_info: &[GraphicsPipelineCreateInfo<I>],
    ) -> Vec<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait PipelineCache<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: PipelineCacheCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn get_data_size(&self, device: &I::Device) -> Result<usize>;
    fn get_data(&self, device: &I::Device, data: &mut [u8]) -> Result<usize>;

    fn merge<C>(&self, device: &I::Device, sources: &[C]) -> Result<()> where
        C: Borrow<Self>;
}

pub trait Fence<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: FenceCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn reset<F>(device: &I::Device, fences: &mut [F]) where
        F: BorrowMut<Self>;
    fn wait<F>(device: &I::Device, fences: &[F]) where
        F: Borrow<Self>;
    fn get_status(&self, device: &I::Device) -> Result<()>;
}

pub trait Semaphore<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: SemaphoreCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }
}

pub trait Event<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: EventCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn get_status(&self, device: &I::Device) -> Result<()>;
    fn set(&mut self, device: &I::Device) -> Result<()>;
    fn reset(&mut self, _device: &I::Device) -> Result<()>;
}

pub trait QueryPool<I: Impl>: Sized + Send + Sync + 'static {
    fn new(device: &I::Device, info: QueryPoolCreateInfo<I>) -> Result<Self>;
    fn destroy(self, _device: &I::Device) { mem::drop(self) }

    fn get_results(&self, device: &I::Device, queries: Range<u32>, data: &mut [u8], stride: u64, flags: QueryResultFlags) -> Result<()>;
}

macro_rules! vulkan_try {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(err) => return sys::VkResult::from(err),
        }
    }
}

macro_rules! vk_get_proc {
    {
        match $name:tt {
            functions { $($func_name:ident,)* }
            _ => $default:expr,
        }
    } => {
        use sys::*;
        match $name {
            $(
                stringify!($func_name) => mem::transmute::<concat_idents!(PFN_, $func_name), _>(Some(*&$func_name::<I>)),
            )*
            _ => $default,
        }
    }
}

#[doc(hidden)]
#[inline]
pub unsafe fn get_instance_proc_addr<I: Impl>(_instance: *mut c_void, name: *const c_char) -> Option<unsafe extern "C" fn()> {
    let name = CStr::from_ptr(name);

    let name = if let Ok(name) = name.to_str() {
        name
    } else {
        warn!("invalid UTF8 in name passed to vk_icdGetInstanceProcAddr: {:?}", name);
        return None;
    };

    vk_get_proc! { match name {
        functions {
            vkCreateInstance,
            vkDestroyInstance,
            vkEnumeratePhysicalDevices,
            vkGetPhysicalDeviceFeatures,
            vkGetPhysicalDeviceFormatProperties,
            //vkGetPhysicalDeviceImageFormatProperties,
            //vkGetPhysicalDeviceProperties,
            //vkGetPhysicalDeviceQueueFamilyProperties,
            //vkGetPhysicalDeviceMemoryProperties,
            //vkGetInstanceProcAddr,
            //vkGetDeviceProcAddr,
            vkCreateDevice,
            //vkEnumerateInstanceExtensionProperties,
            //vkEnumerateDeviceExtensionProperties,
            //vkEnumerateInstanceLayerProperties,
            //vkEnumerateDeviceLayerProperties,
            //vkGetPhysicalDeviceSparseImageFormatProperties,
        }
        _ => {
            warn!("unknown name passed to vk_icdGetInstanceProcAddr: {:?}", name);
            None
        },
    } }
}

macro_rules! vulkan_dispatch_cast {
    ($x:expr => $name:ident) => {
        &(*($x as *const dispatch::$name<I>)).imp
    }
}

macro_rules! vulkan_dispatch_form {
    ($x:expr => $name:ident with $alloc:expr) => {
        // TODO: alloc
        {
        mem::drop($alloc); // To prevent unused variable warning
        vulkan_dispatch_form!($x => $name)
        }
    };
    ($x:expr => $name:ident) => {
        Box::into_raw(Box::new(dispatch::$name::<I> {
            loader: sys::ICD_LOADER_MAGIC as usize,
            imp: $x,
        })) as *mut _
    };
}

macro_rules! vulkan_dispatch_drop {
    ($x:expr => $name:ident with $alloc:expr) => {
        // TODO: alloc
        mem::drop($alloc); // To prevent unused variable warning
        vulkan_dispatch_drop!($x => $name)
    };
    ($x:expr => $name:ident) => {
        mem::drop(Box::from_raw($x as *mut dispatch::$name<I>));
    };
}

macro_rules! vulkan_simple_destroy_impl {
    ($raw_name:ident as $name:ident  => $destructor_name:ident with alloc) => {
        unsafe extern "C" fn $destructor_name<I: Impl>(
            object: sys::$raw_name,
            alloc: *const sys::VkAllocationCallbacks,
        ) {
            vulkan_dispatch_drop!(object => $name with alloc);
        }
    }
}

const OK: sys::VkResult = sys::VkResult::VK_SUCCESS;

unsafe extern "C" fn vkCreateInstance<I: Impl>(
    create_info: *const sys::VkInstanceCreateInfo,
    alloc: *const sys::VkAllocationCallbacks,
    instance: *mut sys::VkInstance,
) -> sys::VkResult {
    let icd_inst = vulkan_try!(I::Instance::create(
        InstanceCreateInfo(&*create_info, PhantomData),
    ));

    *instance = vulkan_dispatch_form!(icd_inst => Instance with alloc);

    OK
}

vulkan_simple_destroy_impl!(VkInstance as Instance => vkDestroyInstance with alloc);

unsafe extern "C" fn vkEnumeratePhysicalDevices<I: Impl>(
    instance: sys::VkInstance,
    device_count: *mut u32,
    physical_devices: *mut sys::VkPhysicalDevice,
) -> sys::VkResult {
    let instance = vulkan_dispatch_cast!(instance => Instance);

    let actual_device_count = vulkan_try!(instance.physical_device_count());

    if physical_devices.is_null() {
        *device_count = actual_device_count;
        return OK;
    }

    let buffer_size = *device_count;

    let icd_devs = vulkan_try!(instance.enumerate_physical_devices(buffer_size));

    *device_count = cmp::min(icd_devs.len() as u32, buffer_size);

    for (i, icd_dev) in icd_devs.into_iter().take(buffer_size as usize).enumerate() {
        *physical_devices.offset(i as isize) = vulkan_dispatch_form!(icd_dev => PhysicalDevice);
    }

    if buffer_size >= actual_device_count {
        OK
    } else {
        sys::VkResult::VK_INCOMPLETE
    }
}

unsafe extern "C" fn vkGetPhysicalDeviceFeatures<I: Impl>(
    physical_device: sys::VkPhysicalDevice,
    features: *mut sys::VkPhysicalDeviceFeatures,
) {
    *features = vulkan_dispatch_cast!(physical_device => PhysicalDevice)
        .get_features()
        .into();
}

unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties<I: Impl>(
    physical_device: sys::VkPhysicalDevice,
    format: sys::VkFormat,
    properties: *mut sys::VkFormatProperties,
) {
    *properties = vulkan_dispatch_cast!(physical_device => PhysicalDevice)
        .get_format_properties(format.into())
        .into();
}

unsafe extern "C" fn vkCreateDevice<I: Impl>(
    physical_device: sys::VkPhysicalDevice,
    create_info: *const sys::VkDeviceCreateInfo,
    alloc: *const sys::VkAllocationCallbacks,
    device: *mut sys::VkDevice,
) -> sys::VkResult {
    let icd_dev = vulkan_try!(I::PhysicalDevice::create_device(
        vulkan_dispatch_cast!(physical_device => PhysicalDevice),
        DeviceCreateInfo(&*create_info, PhantomData),
    ));

    *device = vulkan_dispatch_form!(icd_dev => Device with alloc);

    OK
}

macro_rules! vulkan_dispatch_impls {
    ($($n:ident)*) => {
        #[allow(unused_imports)]
        mod dispatch {
            use super::{Impl, sys};

            $(
                #[repr(C)]
                pub struct $n<I: Impl> {
                    pub loader: usize,
                    pub imp: I::$n,
                }
            )*
        }
    }
}

vulkan_dispatch_impls! {
    Instance
    PhysicalDevice
    Device
}

#[macro_export]
macro_rules! vulkan_icd_dispatch {
    ($target:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn vk_icdGetInstanceProcAddr(instance: *mut ::std::os::raw::c_void, name: *const ::std::os::raw::c_char) 
            -> Option<unsafe extern "C" fn()>
        {
            $crate::icd::get_instance_proc_addr::<$target>(instance, name)
        }
    }
}

