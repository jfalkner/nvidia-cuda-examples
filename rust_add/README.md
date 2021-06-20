# Basic RustCUDA Example

Below is largely based on following the [RustCUDA quickstart](https://github.com/bheisler/RustaCUDA#quickstart) run on Ubuntu laptop with an NVIDA P2000 Max-Q.

This isn't 100% Rust code. The kernel loaded on the GPU is actually C-like CUDA code in
`resources/add.cu`. The Rust code loads it as a kernel along with data for process and
then fetches the result. Rust does everything but made the C/CUDA kernel itself.

### Compile

Example add vectors code is in [`resources/add.cu`](resources/add.cu) and it compiled
to [`resources/add.ptx`](resources/add.ptx) for loading as a kernel.

```
cd resources
nvcc --ptx resources/add.cu
```

Rust code that'll load the kernel, send data, run the kernel then return results is in
[`src/main.rs`](src/main.rs). This uses the RustCUDA code that exposes NVIDA's CUDA APIs.


```
# build the code
cargo build

   Compiling proc-macro2 v1.0.27
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.73
   Compiling bitflags v1.2.1
   Compiling cuda-sys v0.2.0
   Compiling rustacuda_core v0.1.2
   Compiling quote v1.0.9
   Compiling rustacuda_derive v0.1.2
   Compiling rustacuda v0.1.2
   Compiling rust_add v0.1.0 (/home/jfalkner/nvidia-cuda-examples/rust_add)
    Finished dev [unoptimized + debuginfo] target(s) in 7.59s

```

### Running

Runing is done same as a normal binary. The CUDA-specific calls allocate memory on the GPU, copy data and run the vector adding kernel.

```
# run the example on the GPU
cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rust_add`
Sum is 30
```

### Profiling

Here are the profiling results using `nvprof` on the results of `cargo build`.

```
sudo nvprof target/debug/rust_add

==1025456== NVPROF is profiling process 1025456, command: target/debug/rust_add
Sum is 30
==1025456== Profiling application: target/debug/rust_add
==1025456== Profiling result:
            Type  Time(%)      Time     Calls       Avg       Min       Max  Name
 GPU activities:   42.52%  4.0000us         3  1.3330us  1.2480us  1.5040us  [CUDA memcpy HtoD]
                   39.46%  3.7120us         1  3.7120us  3.7120us  3.7120us   sum
                   18.03%  1.6960us         1  1.6960us  1.6960us  1.6960us  [CUDA memcpy DtoH]
      API calls:   62.84%  230.71ms         1  230.71ms  230.71ms  230.71ms  cuCtxCreate
                   21.24%  77.995ms         1  77.995ms  77.995ms  77.995ms  cuCtxDestroy
                   15.80%  58.002ms         1  58.002ms  58.002ms  58.002ms  cuModuleLoadData
                    0.05%  184.43us         3  61.477us  2.6090us  178.42us  cuMemAlloc
                    0.03%  111.57us         3  37.188us  2.1850us  103.86us  cuMemFree
                    0.01%  30.154us         1  30.154us  30.154us  30.154us  cuModuleUnload
                    0.01%  29.947us         1  29.947us  29.947us  29.947us  cuDeviceGetPCIBusId
                    0.01%  24.795us         1  24.795us  24.795us  24.795us  cuStreamCreateWithPriority
                    0.01%  20.762us         3  6.9200us  5.4590us  9.4750us  cuMemcpyHtoD
                    0.00%  15.790us         1  15.790us  15.790us  15.790us  cuLaunchKernel
                    0.00%  14.271us         1  14.271us  14.271us  14.271us  cuMemcpyDtoH
                    0.00%  9.3690us         1  9.3690us  9.3690us  9.3690us  cuStreamSynchronize
                    0.00%  5.0340us         1  5.0340us  5.0340us  5.0340us  cuStreamDestroy
                    0.00%  4.1600us         2  2.0800us     509ns  3.6510us  cuDeviceGetCount
                    0.00%  3.0670us         3  1.0220us     643ns  1.6180us  cuDeviceGetAttribute
                    0.00%  2.3470us         2  1.1730us     490ns  1.8570us  cuDeviceGet
                    0.00%     508ns         1     508ns     508ns     508ns  cuModuleGetFunction
```
