use std::sync::Arc;
use ash::vk;
use crate::vulkan::Device;
use crate::vulkan::device::DeviceInner;

pub struct Image {
    pub device_dep: Arc<DeviceInner>,
    pub image: vk::Image,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_image(self.image, None);
        }
    }
}

impl Image {
    pub fn new(device: &Device, width: u32, height: u32) -> Image {
        let create_info = vk::ImageCreateInfo::default()
            .extent(vk::Extent3D {
                width,
                height,
                depth: 1,
            })
            .samples(vk::SampleCountFlags::TYPE_1)
            .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .array_layers(1)
            .mip_levels(1)
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM);

        let image = unsafe {
            device.handle().create_image(&create_info, None)
                .expect("Failed to create image")
        };

        Image {
            image,
            device_dep: device.inner.clone(),
        }
    }
}