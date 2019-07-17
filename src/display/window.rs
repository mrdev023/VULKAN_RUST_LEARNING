use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::command_buffer::DynamicState;
use vulkano::swapchain::Surface;
use vulkano::instance::Instance;
use vulkano_win::VkSurfaceBuild;
use winit::{EventsLoop, WindowBuilder, Window};
use std::sync::Arc;
use std::option::Option;

pub struct Frame {
    pub events_loop : EventsLoop,
    pub surface : Arc<Surface<Window>>
}

impl Frame {
    pub fn create_window(instance : Arc<Instance>) -> Frame {
        let events_loop = EventsLoop::new();
        let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance).unwrap();
        Frame {
            events_loop,
            surface
        }
    }

    pub fn get_physical_dimensions(&self) -> Option<[u32; 2]>{
        if let Some(dimensions) = self.surface.window().get_inner_size() {
            let dimensions: (u32, u32) = dimensions.to_physical(self.surface.window().get_hidpi_factor()).into();
            Some([dimensions.0, dimensions.1])
        } else {
            None
        }
    }
}

pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
        ) as Arc<dyn FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}