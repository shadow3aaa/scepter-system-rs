use llama_cpp_sys_4::{
    llama_context_default_params, llama_context_params, llama_model_default_params,
    llama_model_params,
};

#[derive(Clone, Copy)]
pub struct Params {
    pub n_ctx: u32,
    pub n_gpu_layers: i32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            n_ctx: 2048,
            n_gpu_layers: 99,
        }
    }
}

impl From<Params> for llama_model_params {
    fn from(params: Params) -> Self {
        Self {
            n_gpu_layers: params.n_gpu_layers,
            ..unsafe { llama_model_default_params() }
        }
    }
}

impl From<Params> for llama_context_params {
    fn from(params: Params) -> Self {
        Self {
            n_ctx: params.n_ctx,
            ..unsafe { llama_context_default_params() }
        }
    }
}
