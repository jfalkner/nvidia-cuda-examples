#[macro_use]
extern crate rustacuda;

use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;

fn main() -> Result<(), Box<dyn Error>> {
    // Set up the context, load the module, and create a stream to run kernels in.
    rustacuda::init(CudaFlags::empty())?;
    let device = Device::get_device(0)?;
    let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    let ptx = CString::new(include_str!("../resources/add.ptx"))?;
    let module = Module::load_from_string(&ptx)?;
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    // Create buffers for data
    let mut in_x = DeviceBuffer::from_slice(&[1.0f32; 1024])?;
    let mut in_y = DeviceBuffer::from_slice(&[2.0f32; 1024])?;
    let mut out = DeviceBuffer::from_slice(&[0.0f32; 1024])?;

    // This kernel adds each element in `in_x` and `in_y` and writes the result into `out`.
    unsafe {
        // Launch the kernel again using the `function` form:
        let function_name = CString::new("sum")?;
        let sum = module.get_function(&function_name)?;
        // Launch with 1x1x1 (1) blocks of 10x1x1 (10) threads, to show that you can use tuples to
        // configure grid and block size.
        let result = launch!(sum<<<(1, 1, 1), (1024, 1, 1), 0, stream>>>(
            in_x.as_device_ptr(),
            in_y.as_device_ptr(),
            out.as_device_ptr(),
            out.len()
        ));
        result?;
    }

    // Kernel launches are asynchronous, so we wait for the kernels to finish executing.
    stream.synchronize()?;

    // Copy the results back to host memory
    let mut out_host = [0.0f32; 1024];
    out.copy_to(&mut out_host[0..1024])?;

    for x in out_host.iter() {
        assert_eq!(3.0 as u32, *x as u32);
    }

    println!("Launched kernel successfully.");
    Ok(())
}
