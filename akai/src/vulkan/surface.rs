use std::sync::Arc;
use ash::khr::surface;
use ash::vk::SurfaceKHR;
use crate::vulkan::Instance;
use crate::window::Window;

/// A presentation surface for rendering graphics to a window.
pub struct Surface {
    _instance: Arc<Instance>,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
}

impl Surface {
    pub fn new(instance: Arc<Instance>, window: &Window) -> Surface {
        let surface_loader = surface::Instance::new(&instance.get_entry(), instance.get_vk_instance());

        let surface = unsafe {
            ash_window::create_surface(
                &instance.get_entry(),
                instance.get_vk_instance(),
                window.display_handle(),
                window.window_handle(),
                None,
            ).expect("Failed to get surface.")
        };

        Surface {
            _instance: instance,
            surface,
            surface_loader,
        }
    }

    pub fn get_vk_surface(&self) -> &SurfaceKHR {
        &self.surface
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}