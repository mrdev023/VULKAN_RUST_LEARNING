use vulkano::instance::{Instance, PhysicalDevice};
use std::sync::Arc;

pub fn create_instance() -> Arc<Instance> {
    let extensions = vulkano_win::required_extensions();
    Instance::new(None, &extensions, None).unwrap()
}

pub fn create_physical_device (instance : &Arc<Instance>) -> PhysicalDevice {
    let physical = PhysicalDevice::enumerate(instance).next().unwrap();
    println!("Device : {} (type: {:?})", physical.name(), physical.ty());
    println!("Vulkan version : {}", physical.api_version());
    println!("Driver version: {}", physical.driver_version());
    physical
}