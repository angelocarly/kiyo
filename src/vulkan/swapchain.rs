use std::sync::Arc;
use ash::khr::swapchain;
use ash::vk;
use ash::vk::{CompositeAlphaFlagsKHR, ImageUsageFlags, PresentModeKHR, SharingMode, SurfaceFormatKHR, SwapchainKHR};
use log::info;
use crate::app::Window;
use crate::vulkan::{Device, Instance, Surface, LOG_TARGET};
use crate::vulkan::device::DeviceInner;

/// Vulkan does not have a concept of a "default framebuffer". Instead, we need a framework that "owns" the images that will eventually be presented to the screen.
/// The general purpose of the swapchain is to synchronize the presentation of images with the refresh rate of the screen.
pub struct SwapchainInner {
    device_dep: Arc<DeviceInner>,
    swapchain_loader: swapchain::Device,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    extent: vk::Extent2D,
    format: SurfaceFormatKHR
}

impl Drop for SwapchainInner {
    fn drop(&mut self) {
        unsafe {
            for &image_view in self.image_views.iter() {
                self.device_dep.device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None)
        }
    }
}

pub struct Swapchain {
    pub inner: Arc<SwapchainInner>,
}

impl Swapchain {
    pub fn new(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
        device: &Device,
        window: &Window,
        surface: &Surface,
        preferred_present_mode: PresentModeKHR
    ) -> Swapchain {
        let swapchain_loader = swapchain::Device::new(instance.handle(), device.handle());

        let available_formats = surface.get_formats(physical_device);
        let surface_format = available_formats.iter()
            .find(|f| f == &&vk::SurfaceFormatKHR {
                format: vk::Format::R8G8B8A8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            })
            .unwrap_or(available_formats.first().expect("No surface format found"));

        info!(target: LOG_TARGET, "Using swapchain surface format: {:?}", surface_format);

        let surface_capabilities = surface.get_surface_capabilities(physical_device);

        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        // Max image count can be 0
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let pre_transform = if surface_capabilities.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY) {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = surface.get_present_modes(physical_device);
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == preferred_present_mode)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let extent = match surface_capabilities.current_extent.width {
            u32::MAX => window.get_extent(),
            _ => surface_capabilities.current_extent
        };

        let create_info = vk::SwapchainCreateInfoKHR::default()
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT | ImageUsageFlags::TRANSFER_DST)
            .image_extent(extent)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(pre_transform)
            .present_mode(present_mode)
            .min_image_count(desired_image_count)
            .surface(*surface.handle())
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None).unwrap() };

        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

        let mut image_views = Vec::new();
        for &image in images.iter() {
            let image_view_create_info = vk::ImageViewCreateInfo::default()
                .flags(vk::ImageViewCreateFlags::empty())
                .format(surface_format.format)
                .view_type(vk::ImageViewType::TYPE_2D)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image(image);

            let imageview = unsafe { device.handle().create_image_view(&image_view_create_info, None).unwrap() };
            image_views.push(imageview);
        }

        let swapchain_inner = SwapchainInner {
            device_dep: device.inner.clone(),
            swapchain_loader,
            swapchain,
            images,
            image_views,
            extent,
            format: *surface_format
        };

        Self {
            inner: Arc::new(swapchain_inner)
        }
    }

    pub fn get_images(&self) -> &Vec<vk::Image> {
        &self.inner.images
    }

    pub fn get_image_views(&self) -> &Vec<vk::ImageView> {
        &self.inner.image_views
    }

    pub fn get_image_count(&self) -> u32 {
        self.inner.images.len() as u32
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        self.inner.extent
    }

    pub fn get_format(&self) -> SurfaceFormatKHR {
        self.inner.format
    }

    pub fn handle(&self) -> SwapchainKHR {
        self.inner.swapchain
    }

    /// Queue an image for presentation.
    ///
    /// - `semaphore` - A semapore to wait on before issuing the present info.
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html
    pub fn queue_present(&self, queue: vk::Queue, wait_semaphore: vk::Semaphore, image_index: u32) {
        let mut result = [vk::Result::SUCCESS];
        unsafe {
            let swapchains = [self.handle()];
            let indices = [image_index];
            let semaphores = [wait_semaphore];
            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&semaphores)
                .swapchains(&swapchains)
                .image_indices(&indices)
                .results(&mut result);
            self.inner.swapchain_loader.queue_present(queue, &present_info)
                .expect("Failed to present queue");
        }
    }

    /// Acquire the next image in the swapchain.
    /// * `semaphore` - A semaphore to signal when the image is available.
    ///
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html
    pub fn acquire_next_image(&self, semaphore: vk::Semaphore) -> u32 {
        unsafe {
            let (image_index, _) = self.inner.swapchain_loader
                .acquire_next_image(
                    self.handle(),
                    u64::MAX,
                    semaphore,
                    vk::Fence::null()
                )
                .expect("Failed to acquire next image");
            image_index
        }
    }
}
