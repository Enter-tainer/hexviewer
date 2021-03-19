use evalexpr::{Function, HashMapContext};

use crate::{bin, hex};
use evalexpr::Context;
use evalexpr::*;
pub fn set_cmd(context: &mut HashMapContext) -> Result<(), EvalexprError> {
    context.set_function(
        "hex".to_string(),
        Function::new(Box::new(|arg| {
            if let Ok(int) = arg.as_int() {
                Ok(Value::String(hex::print_hex(int)))
            } else if let Ok(_float) = arg.as_float() {
                Err(EvalexprError::expected_int(arg.clone()))
            // Ok(Value::String(hex::print_hex(float)))
            } else {
                Err(EvalexprError::expected_number(arg.clone()))
            }
        })),
    )?;
    context.set_function(
        "bin".to_string(),
        Function::new(Box::new(|arg| {
            if let Ok(int) = arg.as_int() {
                Ok(Value::String(bin::print_bin(int)))
            } else if let Ok(_float) = arg.as_float() {
                Err(EvalexprError::expected_int(arg.clone()))
            // Ok(Value::String(hex::print_hex(float)))
            } else {
                Err(EvalexprError::expected_number(arg.clone()))
            }
        })),
    )?;
    Ok(())
}
