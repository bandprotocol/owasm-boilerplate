use owasm::core::{decode_cmds, encode_outputs, execute_with_local_env};
use std::env;
use std::fs;
use wasmer_runtime::{error, imports, instantiate, Value};

fn main() -> error::Result<()> {
    let code = fs::read(&env::args().collect::<Vec<String>>()[1]).unwrap();
    let input_raw: Vec<u8>;
    let output_raw: Vec<u8>;
    {
        let instance = instantiate(&code, &imports! {})?;
        let ptr = instance.call("__prepare", &[])?[0].to_u128() as i32;
        let memory = instance.context().memory(0);
        let sz: i32 = memory.view()[(ptr / 4) as usize].get();
        input_raw = memory.view()[(ptr + 4) as usize..(ptr + sz + 4) as usize]
            .iter()
            .map(|cell| cell.get())
            .collect();
    }
    {
        let output = vec![
            encode_outputs(execute_with_local_env(decode_cmds(&input_raw).unwrap())).unwrap(),
            encode_outputs(execute_with_local_env(decode_cmds(&input_raw).unwrap())).unwrap(),
            encode_outputs(execute_with_local_env(decode_cmds(&input_raw).unwrap())).unwrap(),
        ];
        let instance = instantiate(&code, &imports! {})?;
        let memory = instance.context().memory(0);
        let outer_ptr = instance.call("__allocate", &[Value::I32(output.len() as i32 * 4 + 4)])?[0]
            .to_u128() as i32;
        memory.view()[(outer_ptr / 4) as usize].set(output.len() as i32);
        for each in output.iter() {
            let sz = each.len() as i32;
            let ptr = instance.call("__allocate", &[Value::I32(sz + 4)])?[0].to_u128() as i32;
            memory.view()[(outer_ptr / 4 + 1) as usize].set(ptr);
            memory.view()[(ptr / 4) as usize].set(sz);
            for (idx, ch) in each.iter().enumerate() {
                memory.view()[(ptr + (idx as i32) + 4) as usize].set(*ch);
            }
        }
        let o = instance.call("__execute", &[Value::I32(outer_ptr)])?[0].to_u128() as i32;
        let sz: i32 = memory.view()[(o / 4) as usize].get();
        output_raw = memory.view()[(o + 4) as usize..(o + sz + 4) as usize]
            .iter()
            .map(|cell| cell.get())
            .collect();
    }
    println!(
        "0x{}",
        output_raw.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
    );
    Ok(())
}
