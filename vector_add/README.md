# Basic CUDA Example

Below is largely based on following the CUDA tutorial on [readthedocs.io](https://cuda-tutorial.readthedocs.io/en/latest/tutorials/tutorial01/) run on Ubuntu laptop with an NVIDA P2000 Max-Q.

### Compile

Example code to add vectors of values is in [`vector_add.c`](vector_add.c), similar to the tutorial's
example.

```
nvcc vector_add.cu -o vector_add
```

### Running

Runing is done same as a normal binary. The CUDA-specific calls allocate memory on the GPU, copy data and run the vector adding kernel.

```
time ./vector_add
out[0] = 3.000000
PASSED
./vector_add  1.92s user 0.29s system 94% cpu 2.341 total
```

### Profiling

Here are the profiling results using `nvprof`. 

```
sudo nvprof ./vector_add

==970283== NVPROF is profiling process 970283, command: ./vector_add
out[0] = 3.000000
PASSED

==970283== Profiling application: ./vector_add
==970283== Profiling result:
            Type  Time(%)      Time     Calls       Avg       Min       Max  Name
 GPU activities:   97.82%  1.58634s         1  1.58634s  1.58634s  1.58634s  vector_add(float*, float*, float*, int)
                    1.18%  19.082ms         1  19.082ms  19.082ms  19.082ms  [CUDA memcpy DtoH]
                    1.00%  16.238ms         2  8.1188ms  7.0825ms  9.1550ms  [CUDA memcpy HtoD]
      API calls:   83.29%  1.62249s         3  540.83ms  7.1400ms  1.60596s  cudaMemcpy
                   16.51%  321.60ms         3  107.20ms  152.94us  321.29ms  cudaMalloc
                    0.15%  2.8922ms         3  964.08us  216.46us  1.3731ms  cudaFree
                    0.02%  471.06us        97  4.8560us     424ns  214.95us  cuDeviceGetAttribute
                    0.02%  338.44us         1  338.44us  338.44us  338.44us  cuDeviceTotalMem
                    0.00%  75.495us         1  75.495us  75.495us  75.495us  cuDeviceGetName
                    0.00%  58.404us         1  58.404us  58.404us  58.404us  cudaLaunchKernel
                    0.00%  31.653us         1  31.653us  31.653us  31.653us  cuDeviceGetPCIBusId
                    0.00%  5.6920us         3  1.8970us     454ns  3.1350us  cuDeviceGetCount
                    0.00%  2.6850us         2  1.3420us     666ns  2.0190us  cuDeviceGet
                    0.00%     730ns         1     730ns     730ns     730ns  cuDeviceGetUuid
```
