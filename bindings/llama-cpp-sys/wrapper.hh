#include <ggml.h>
#include <gguf.h>
#include <llama.h>

#include <ggml-backend.h>
#include <ggml-cpu.h>
#include <ggml-cuda.h>
#include <ggml-metal.h>
#include <ggml-vulkan.h>

extern "C" {
static uint32_t llama_default_seed = LLAMA_DEFAULT_SEED;
static uint32_t ggml_kq_mask_pad = GGML_KQ_MASK_PAD;
}
