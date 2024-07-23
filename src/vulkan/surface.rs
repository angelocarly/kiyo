use ash::khr::surface;
use ash::vk;
use ash::vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceKHR};
use crate::app::Window;
use crate::vulkan::Instance;

/// A presentation surface for rendering graphics to a window.
pub struct Surface {
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
}

impl Surface {
    pub fn new(entry: &ash::Entry, instance: &Instance, window: &Window) -> Surface {
        let surface_loader = surface::Instance::new(&entry, instance.handle());

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                instance.handle(),
                window.display_handle(),
                window.window_handle(),
                None,
            ).expect("Failed to get surface.")
        };

        Surface {
            surface,
            surface_loader,
        }
    }

    pub fn handle(&self) -> &SurfaceKHR {
        &self.surface
    }

    pub fn get_formats(&self, physical_device: &vk::PhysicalDevice) -> Vec<vk::SurfaceFormatKHR> {
        unsafe { self.surface_loader.get_physical_device_surface_formats(*physical_device, self.surface).unwrap() }
    }

    pub fn get_present_modes(&self, physical_device: &vk::PhysicalDevice) -> Vec<PresentModeKHR> {
        unsafe { self.surface_loader.get_physical_device_surface_present_modes(*physical_device, self.surface).unwrap() }
    }

    pub fn get_surface_capabilities(&self, physical_device: &vk::PhysicalDevice) -> SurfaceCapabilitiesKHR {
        unsafe { self.surface_loader.get_physical_device_surface_capabilities(*physical_device, self.surface).unwrap() }
    }

}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}