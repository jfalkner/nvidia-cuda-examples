# RustCUDA Vector Addition Example

Below is largely based on following the [RustCUDA](https://github.com/bheisler/RustaCUDA) vector addition example and run on Ubuntu laptop with an NVIDA P2000 Max-Q.

See the notes for the single thread example in [../rust_add/README.md](../rust_add) for compiling the C/CUDA code and more details.

```

### Running

Runing is done same as a normal binary. The CUDA-specific calls allocate memory on the GPU, copy data and run the vector adding kernel.

```
# build the code
cargo build

# run the example on the GPU
time cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rust_add_vector`
Launched kernel successfully.
cargo run  0.06s user 0.22s system 74% cpu 0.373 total
```

Notice that above is approximately as fast as calculating one value from the `../rust_add` example even though 1024 are done in parallel.

```
time cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rust_add`
Sum is 30
cargo run  0.06s user 0.24s system 61% cpu 0.479 total
```

### Profiling

Here are the profiling results using `nvprof` on the results of `cargo build`. Notice
that compared to the single thread example the `sum` function is taking the clear majority of time.

```
sudo nvprof target/debug/rust_add_vector

==1043196== NVPROF is profiling process 1043196, command: target/debug/rust_add_vector
Launched kernel successfully.
==1043196== Profiling application: target/debug/rust_add_vector
==1043196== Profiling result:
            Type  Time(%)      Time     Calls       Avg       Min       Max  Name
 GPU activities:   44.24%  3.9360us         3  1.3120us  1.2160us  1.5040us  [CUDA memcpy HtoD]
                   36.33%  3.2320us         1  3.2320us  3.2320us  3.2320us   sum
                   19.42%  1.7280us         1  1.7280us  1.7280us  1.7280us  [CUDA memcpy DtoH]
      API calls:   75.63%  229.50ms         1  229.50ms  229.50ms  229.50ms  cuCtxCreate
                   24.22%  73.484ms         1  73.484ms  73.484ms  73.484ms  cuCtxDestroy
                    0.04%  120.22us         3  40.074us  1.6540us  116.25us  cuMemAlloc
                    0.03%  100.22us         1  100.22us  100.22us  100.22us  cuModuleLoadData
                    0.03%  96.415us         3  32.138us  1.5850us  89.883us  cuMemFree
                    0.01%  29.303us         1  29.303us  29.303us  29.303us  cuModuleUnload
                    0.01%  26.649us         1  26.649us  26.649us  26.649us  cuDeviceGetPCIBusId
                    0.01%  22.911us         3  7.6370us  6.7330us  9.2040us  cuMemcpyHtoD
                    0.00%  14.503us         1  14.503us  14.503us  14.503us  cuMemcpyDtoH
                    0.00%  14.358us         1  14.358us  14.358us  14.358us  cuLaunchKernel
                    0.00%  8.9650us         1  8.9650us  8.9650us  8.9650us  cuStreamSynchronize
                    0.00%  8.7860us         1  8.7860us  8.7860us  8.7860us  cuStreamCreateWithPriority
                    0.00%  5.3050us         3  1.7680us  1.2110us  2.0620us  cuDeviceGetAttribute
                    0.00%  4.3200us         1  4.3200us  4.3200us  4.3200us  cuStreamDestroy
                    0.00%  4.1780us         2  2.0890us  1.0080us  3.1700us  cuDeviceGet
                    0.00%  4.0370us         2  2.0180us     415ns  3.6220us  cuDeviceGetCount
                    0.00%  3.8750us         1  3.8750us  3.8750us  3.8750us  cuModuleGetFunction
```
