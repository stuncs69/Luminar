use cranelift_codegen::ir::{AbiParam, InstBuilder, Signature};
use cranelift_codegen::ir::types::{I32};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

use crate::base::enums;
use crate::lib::main::{self, LibraryFunction};

fn execute_func_call_ir<'a>(
    module: &mut dyn Module,
    builder: &mut FunctionBuilder<'a>,
    func_name: &str,
    args: Vec<&str>
) -> cranelift_codegen::ir::Value {
    let int = module.target_config().pointer_type();
    let mut sig = Signature::new(module.target_config().default_call_conv);

    for _ in &args {
        sig.params.push(AbiParam::new(int));
    }

    sig.params.push(AbiParam::new(int));

    let func_id = module
        .declare_function(func_name, Linkage::Local, &sig)
        .expect("Function declaration failed");

    let func_ref = module
        .declare_func_in_func(func_id, &mut builder.func);

    let mut func_args = Vec::new();
    for arg in &args {
        let value = arg.parse::<i32>().expect("Invalid argument");
        let arg_val = builder.ins().iconst(int, value as i64);
        func_args.push(arg_val);
    }

    let call_inst = builder.ins().call(func_ref, &func_args);

    let call_result = builder.inst_results(call_inst)[0];

    call_result
}

pub fn runtime(tokens: Vec<enums::Token>) {
    let mut builder_ctx = FunctionBuilderContext::new();
    let mut jit_builder = cranelift_jit::JITBuilder::new(cranelift_module::default_libcall_names());
    let mut module = cranelift_jit::JITModule::new(jit_builder.unwrap());


    let mut ctx = module.make_context();
    let int = module.target_config().pointer_type();
    ctx.func.signature.returns.push(AbiParam::new(int));

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

    let block = builder.create_block();
    builder.switch_to_block(block);

    let func_name = "my_function";
    let args = vec!["42", "17"];

    let func_result = execute_func_call_ir(&mut module, &mut builder, func_name, args);

    builder.ins().return_(&[func_result]);

    builder.finalize();

    let func_id = module
        .declare_function("jit_fn", Linkage::Export, &ctx.func.signature)
        .expect("Function declaration failed");

    module.define_function(func_id, &mut ctx).expect("Function definition failed");
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    // Run the JIT function.
    let code = module.get_finalized_function(func_id);
    let func: extern "C" fn() -> i64 = unsafe { std::mem::transmute(code) };

    // Call the compiled function
    let result = func();
    println!("JIT result: {}", result);
}
