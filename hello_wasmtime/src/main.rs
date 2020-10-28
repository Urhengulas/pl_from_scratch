use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // All wasm objects operate within the context of a "store"
    let store = Store::default();

    // Modules can be compiled through either the text or binary format
    let wat = r#"
        (module
            (import "" "" (func $println_num (param i32)))

            (func (export "main")
                i32.const 42
                call $println_num)
        )
    "#;
    let module = Module::new(store.engine(), wat)?;

    // Host functions can be defined which take/return wasm values and
    // execute arbitrary code on the host.
    let println_num = Func::wrap(&store, |param: i32| {
        println!("{}", param);
    });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get0`.
    let instance = Instance::new(&store, &module, &[println_num.into()])?;
    let hello = instance
        .get_func("main")
        .ok_or(anyhow::format_err!("failed to find `main` function export"))?
        .get0::<()>()?;

    // And finally we can call the wasm as if it were a Rust function!
    hello()?;

    Ok(())
}
