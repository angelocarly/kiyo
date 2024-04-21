use ash::ext::debug_utils;
use ash::{Entry, vk};
use ash::vk::{DebugUtilsMessengerEXT, PhysicalDevice};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::{ptr, vec};
use std::sync::Arc;
use ash::khr::surface;
use winit::raw_window_handle::RawDisplayHandle;
use crate::vulkan::surface::Surface;

struct ValidationInfo {
    required_validation_layers: Vec<CString>,
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
}

/// Vulkan instance. The root interface between the application and the graphics driver.
pub struct Instance {
    entry: Arc<Entry>,
    instance: ash::Instance,
    pub debug_utils: ash::ext::debug_utils::Instance,
    pub debug_utils_messenger: DebugUtilsMessengerEXT,
}

impl Instance {
    pub fn new(entry: Arc<Entry>, display_handle: RawDisplayHandle) -> Self {
        let app_name = CString::new("Akai").unwrap();
        let engine_name = CString::new("Akai Engine").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_version(0)
            .engine_name(engine_name.as_c_str())
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0))
            .application_name(app_name.as_c_str());

        let mut extension_names =
            ash_window::enumerate_required_extensions(display_handle)
                .unwrap()
                .to_vec();
        extension_names.push(debug_utils::NAME.as_ptr());

        #[cfg(target_os = "macos")]
        {
            extension_names.push(ash::khr::portability_enumeration::NAME.as_ptr());
            // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
            extension_names.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
        }

        let validation: ValidationInfo = ValidationInfo {
            required_validation_layers: vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()],
        };

        let c_ptr_validation_layers = validation
            .required_validation_layers
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect::<Vec<_>>();

        let create_flags = if cfg!(any(target_os = "macos")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::default()
        };

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names)
            .enabled_layer_names(&c_ptr_validation_layers)
            .flags(create_flags);

        println!("Creating instance");
        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        let debug_utils_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
            s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(vulkan_debug_utils_callback),
            p_user_data: ptr::null_mut(),
            _marker: Default::default(),
        };

        let debug_utils = debug_utils::Instance::new(&entry, &instance);
        let debug_utils_messenger =
            unsafe { debug_utils.create_debug_utils_messenger(&debug_utils_create_info, None) }
                .expect("Failed to create debug utils messenger");

        Self {
            entry,
            instance,
            debug_utils,
            debug_utils_messenger,
        }
    }

    pub fn create_physical_device(&self, surface: &Surface) -> (PhysicalDevice, u32) {
        let physical_devices = unsafe {
            self.instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices.")
        };
        let surface_loader = surface::Instance::new(&self.entry, &self.instance);
        let (physical_device, queue_family_index) = physical_devices
            .iter()
            .find_map(|physical_device| {
                unsafe {
                    self.instance.get_physical_device_queue_family_properties(*physical_device)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphics_and_surface =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && surface_loader.get_physical_device_surface_support(
                                    *physical_device,
                                    index as u32,
                                    *surface.get_vk_surface()
                                ).unwrap();
                            if supports_graphics_and_surface {
                                Some((*physical_device, index))
                            } else {
                                None
                            }
                        })
                }
            })
            .expect("Couldn't find a suitable device.");
        (physical_device, queue_family_index as u32)
    }

    pub fn get_vk_instance(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn get_entry(&self) -> Arc<Entry> {
        self.entry.clone()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            println!("Drop instance");
            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_utils_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
