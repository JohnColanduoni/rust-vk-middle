#[macro_use] extern crate vk_middle;

extern crate gfx_corell;

#[cfg(feature = "metal")]
extern crate gfx_device_metalll as backend;

use std::borrow::{Borrow, BorrowMut};
use std::mem;

use vk_middle::icd;
use vk_middle::icd::*;

use gfx_corell::{Instance as GfxInstance, Adapter as GfxAdapter, Device as GfxDevice};
use backend::Resources as R;

vulkan_icd_dispatch!(I);

enum I {
}

impl icd::Impl for I {
    type Instance = Instance;
    type PhysicalDevice = PhysicalDevice;
    type Device = Device;
    type Queue = Queue;
    type CommandBuffer = CommandBuffer;

    type DeviceMemory = DeviceMemory;
    type Buffer = Buffer;
    type Image = Image;

    type Fence = Fence;
    
    type PipelineCache = PipelineCache;
}

struct Instance {
    inner: backend::Instance,
}

impl icd::Instance<I> for Instance {
    fn create(create_info: InstanceCreateInfo<I>) -> Result<Self> {
        let instance = backend::Instance::create();

        Ok(Instance {
            inner: instance,
        })
    }

    fn physical_device_count(&self) -> Result<u32> {
        Ok(self.inner.enumerate_adapters().len() as u32)
    }
    fn enumerate_physical_devices(&self, capacity: u32) -> Result<Vec<PhysicalDevice>> {
        Ok(
            self.inner.enumerate_adapters().into_iter()
                .take(capacity as usize)
                .map(|adapter| PhysicalDevice { inner: adapter })
                .collect()
        )
    }
}

struct PhysicalDevice {
    inner: <backend::Instance as GfxInstance>::Adapter,
}

// TODO: consider this
unsafe impl Send for PhysicalDevice {
}
unsafe impl Sync for PhysicalDevice {
}

impl icd::PhysicalDevice<I> for PhysicalDevice {
    fn get_features(&self) -> PhysicalDeviceFeatures {
        unimplemented!()
    }

    fn get_format_properties(&self, format: Format) -> FormatProperties {
        unimplemented!()
    }

    fn create_device(&self, create_info: DeviceCreateInfo<I>) -> Result<Device> {
        let queues = create_info.queues().map(|queue_info| {
            let family = self.inner.get_queue_families().nth(queue_info.family_index() as usize).expect("invalid queue index");
            (family, queue_info.count())
        });
        Ok(Device {
            inner: self.inner.open(queues),
        })
    }
}

struct Device {
    inner: GfxDevice<R, backend::Factory, backend::CommandQueue>,
}

// TODO: consider this
unsafe impl Send for Device {
}
unsafe impl Sync for Device {
}

impl icd::Device<I> for Device {
    fn wait_idle(&self) -> Result<()> {
        unimplemented!()
    }

    fn allocate_memory(&self, info: MemoryAllocateInfo<I>) -> Result<DeviceMemory> {
        unimplemented!()
    }
    fn flush_mapped_memory_ranges(&self, ranges: MappedMemoryRanges<I>) -> Result<()> {
        unimplemented!()
    }
    fn invalidate_mapped_memory_ranges(&self, ranges: MappedMemoryRanges<I>) -> Result<()> {
        unimplemented!()
    }
}

struct Queue {
}

impl icd::Queue<I> for Queue {
    fn submit(&mut self, submits: SubmitInfos<I>, fence: Option<&mut Fence>) {
        unimplemented!()
    }
    fn wait_idle(&self) -> Result<()> {
        unimplemented!()
    }

    fn bind_sparse(&mut self, infos: BindSparseInfos<I>, fence: Option<&mut Fence>) {
        unimplemented!()
    }
}

struct CommandBuffer {
}

impl icd::CommandBuffer<I> for CommandBuffer {
}

struct DeviceMemory {
}

impl icd::DeviceMemory<I> for DeviceMemory {
    fn map(&mut self, device: &Device, offset: u64, size: u64, flags: MemoryMapFlags) -> Result<*mut u8> {
        unimplemented!()
    }
    fn unmap(&mut self, device: &Device) {
        unimplemented!()
    }

    fn get_memory_commitment(&self, device: &Device) {
        unimplemented!()
    }

    fn destroy(self, _device: &Device) { mem::drop(self) }
}

struct Buffer {
}

impl icd::Buffer<I> for Buffer {
    fn new(device: &Device, info: BufferCreateInfo<I>) -> Result<Self> {
        unimplemented!()
    }
    fn destroy(self, _device: &Device) { mem::drop(self) }

    fn get_memory_requirements(&self, device: &Device) -> MemoryRequirements {
        unimplemented!()
    }
    fn bind_memory(&mut self, device: &Device, memory: &DeviceMemory, offset: u64) -> Result<()> {
        unimplemented!()
    }
}

struct Image {
}

impl icd::Image<I> for Image {
    fn new(device: &Device, info: ImageCreateInfo<I>) -> Result<Self> {
        unimplemented!()
    }
    fn destroy(self, _device: &Device) { mem::drop(self) }

    fn get_memory_requirements(&self, device: &Device) -> MemoryRequirements {
        unimplemented!()
    }
    fn get_sparse_memory_requirements(&self, device: &Device, capacity: u32) -> Vec<SparseImageMemoryRequirements> {
        unimplemented!()
    }
    fn bind_memory(&mut self, device: &Device, memory: &DeviceMemory, offset: u64) -> Result<()> {
        unimplemented!()
    }

    fn get_subresource_layout(&self, subresource: ImageSubresource<I>) -> Result<SubresourceLayout> {
        unimplemented!()
    }
}

struct Fence {
}

impl icd::Fence<I> for Fence {
    fn new(device: &Device, info: FenceCreateInfo<I>) -> Result<Self> {
        unimplemented!()
    }
    fn destroy(self, _device: &Device) { mem::drop(self) }

    fn reset<F>(device: &Device, fences: &mut [F]) where
        F: BorrowMut<Self> {
        unimplemented!()
    }
    fn wait<F>(device: &Device, fences: &[F]) where
        F: Borrow<Self> {
        unimplemented!()
    }
    fn get_status(&self, device: &Device) -> Result<()> {
        unimplemented!()
    }
}

struct PipelineCache {
}

impl icd::PipelineCache<I> for PipelineCache {
    fn new(device: &Device, info: PipelineCacheCreateInfo<I>) -> Result<Self> {
        unimplemented!()
    }
    fn destroy(self, _device: &Device) { mem::drop(self) }

    fn get_data_size(&self, device: &Device) -> Result<usize> {
        unimplemented!()
    }
    fn get_data(&self, device: &Device, data: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    fn merge<C>(&self, device: &Device, sources: &[C]) -> Result<()> where
        C: Borrow<Self> {
        unimplemented!()
    }
}
