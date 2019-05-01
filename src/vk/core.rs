////////////////////////////////////////
//  CRATES
////////////////////////////////////////

extern crate vulkano;

////////////////////////////////////////
//  MODULES
////////////////////////////////////////


////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::device::Device;
use vulkano::device::Queue;

////////////////////////////////////////
//  GLOBAL FUNCTIONS
////////////////////////////////////////

//  create instance
fn create_instance() -> Arc<Instance> {
    
    use vulkano::instance::InstanceExtensions;

    Instance::new( None, &InstanceExtensions::none(), None )
        .expect("failed to create instance")
}

//  create device
fn create_device( instance: &Arc<Instance> ) -> ( Arc<Device>, Arc<Queue> ) {

    //  retrieve physical device handle
    use vulkano::instance::PhysicalDevice;

    //  force to use the first GPU
    let gpu_idx: usize = 0;

    let physical_device = PhysicalDevice::from_index( &instance, gpu_idx )
        .expect("out-of-range GPU index");

    //  query queue families on this physical device
    // for family in physical_device.queue_families() {
    //     println!("Found a queue family with {:?} queue(s)", family.queues_count());
    // }

    let queue_family = physical_device.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    
    use vulkano::device::Device;
    use vulkano::device::DeviceExtensions;
    use vulkano::device::Features;

    let (device, mut queues) = {
        Device::new(physical_device, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    //  get queue for submitting commands (same as stream in CUDA)
    let queue = queues.next().unwrap();

    ( device, queue )
}

////////////////////////////////////////
//  STRUCTS DECLARATION
////////////////////////////////////////

//  ToDo : multiple devices and multiple queues
pub struct VkCore {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

////////////////////////////////////////
//  STRUCTS IMPLEMENTATION
////////////////////////////////////////

impl VkCore {
    //  construct vulkan core
    pub fn new() -> VkCore {
        let instance = create_instance();
        let ( device, queue ) = create_device( &instance );

        VkCore { 
            instance: instance,
            device: device,
            queue: queue,
        }
    }
}
