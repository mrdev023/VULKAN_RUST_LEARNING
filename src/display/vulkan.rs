use vulkano::instance::{Instance, PhysicalDevice};
use std::sync::Arc;
use crate::display::window::Frame;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode};
use winit::Window;
use vulkano::image::SwapchainImage;

/// Create Vulkan Instance
pub fn create_instance() -> Arc<Instance> {
    let extensions = vulkano_win::required_extensions();
    Instance::new(None, &extensions, None).expect("Failed to create vulkan instance")
}

/// Get First Physical Device
pub fn get_physical_device(instance : &Arc<Instance>) -> PhysicalDevice {
    let physical = PhysicalDevice::enumerate(instance).next().unwrap();
    println!("Device : {} (type: {:?})", physical.name(), physical.ty());
    println!("Vulkan version : {}", physical.api_version());
    println!("Driver version: {}", physical.driver_version());
    physical
}

/// Create Virtual Device and get first queue created
pub fn create_queue_and_device_from_physical_device(physical : PhysicalDevice, frame : &Frame) -> (Arc<Device>, Arc<Queue>) {
    println!("Queue Family : ");
    for family in physical.queue_families() {
        println!("\tSupport compute : {} | Support Graphics : {} | Support sparse binding : {} | {:?} queue(s)", family.supports_compute(), family.supports_graphics(), family.supports_sparse_binding(), family.queues_count());
    }

    // Recupere la famille de queue qui supporte le calcul graphique depuis le peripherique physique
    let queue_family = physical.queue_families().find(|&q| {
        q.supports_graphics() && frame.surface.is_supported(q).unwrap_or(false)
    }).unwrap();

    let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };

    // Creer le péripherique virtuel ainsi que les queues
    let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext,
                [(queue_family, 0.5)].iter().cloned()).unwrap();

    // Renvoie le péripherique virtuel ainsi que la premiere queue
    (device, queues.next().unwrap())
}

pub fn create_swapchain (physical : PhysicalDevice, device : &Arc<Device>, queue : &Arc<Queue>, frame : &Frame) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let caps = frame.surface.capabilities(physical).unwrap();

    let usage = caps.supported_usage_flags;

    let alpha = caps.supported_composite_alpha.iter().next().unwrap();

    let format = caps.supported_formats[0].0;

    let initial_dimensions = frame.get_physical_dimensions().unwrap();

    Swapchain::new(device.clone(), frame.surface.clone(), caps.min_image_count, format,
                   initial_dimensions, 1, usage, queue, SurfaceTransform::Identity, alpha,
                   PresentMode::Fifo, true, None).unwrap()
}