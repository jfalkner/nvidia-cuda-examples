# NVIDIA CUDA Examples

A repo of notes and examples while working with NVIDIA CUDA on Ubuntu Linux with C and
Rust. It is intended to be helpful copy-pasta for later projects.

### GPU Info

GPU info can be found by using `nvidia-smi -L` and usage via `nvidia-smi`.

```
nvidia-smi -L
GPU 0: Quadro P2000 with Max-Q Design (UUID: GPU-5fdec6d4-9a10-efa2-d77c-58181d70ebcc)

nvidia-smi
Fri Jun 18 21:45:54 2021       
+-----------------------------------------------------------------------------+
| NVIDIA-SMI 460.80       Driver Version: 460.80       CUDA Version: 11.2     |
|-------------------------------+----------------------+----------------------+
| GPU  Name        Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
| Fan  Temp  Perf  Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
|                               |                      |               MIG M. |
|===============================+======================+======================|
|   0  Quadro P2000 wi...  Off  | 00000000:01:00.0 Off |                  N/A |
| N/A   56C    P3    N/A /  N/A |    876MiB /  4040MiB |      7%      Default |
|                               |                      |                  N/A |
+-------------------------------+----------------------+----------------------+
```

You have to look up NVIDIA's spec sheet to see more details about numbers of cores. The
P2000 has 1024 CUDA cores and 4GB of GPU memory.


### Examples

Code examples testing out CUDA performance.

* Example vector addition from the [cuda-tutorial.readthedocs.io](https://cuda-tutorial.readthedocs.io/en/latest/tutorials/tutorial01/)
