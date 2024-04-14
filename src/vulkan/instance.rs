use std::ffi::CString;
use ash::vk;
use winit::raw_window_handle::HasDisplayHandle;

pub struct Instance {
    _instance: ash::Instance,
}

impl Instance {
    pub fn new(entry: &ash::Entry, window: &winit::window::Window) -> Self {
        let instance = Instance::create_instance(&entry, window);
        Self {
            _instance: instance
        }
    }

    fn create_instance(entry: &ash::Entry, window: &winit::window::Window) -> ash::Instance {
        let app_name = CString::new("Lov").unwrap();
        let engine_name = CString::new("Lov Engine").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_version(0)
            .engine_name(engine_name.as_c_str())
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0))
            .application_name(app_name.as_c_str());

        let extension_names =
            ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .unwrap()
                .to_vec();

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        println!("Creating instance");
        let instance: ash::Instance = unsafe {
            entry.create_instance(&create_info, None).expect("Instance creation error")
        };

        instance
    }
}