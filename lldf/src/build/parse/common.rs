use super::*;


pub fn handle_arithmetic(
    module   : &ParsedModule,
    function : &mut ParsedFunction,
    dest_var : &str,
    operand0 : Value,
    operand1 : Value,
    op       : &str
) -> Result<(), Box<dyn Error>> {
    let operand0 = operand0.to_codevalue(module, function)?;
    let operand1 = operand1.to_codevalue(module, function)?;
    function.line.blocks.push(Codeblock::action("set_var", op, vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        operand0, operand1
    ], vec![ ]));
    Ok(())
}


pub fn handle_bitwise(
    module   : &ParsedModule,
    function : &mut ParsedFunction,
    dest_var : &str,
    operand0 : Value,
    operand1 : Value,
    op       : &str
) -> Result<(), Box<dyn Error>> {
    let operand0 = operand0.to_codevalue(module, function)?;
    let operand1 = operand1.to_codevalue(module, function)?;
    function.line.blocks.push(Codeblock::action("set_var", "Bitwise", vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        operand0, operand1
    ], vec![
        CodeValue::Actiontag { kind : "Operator".to_string(), value : op.to_string(), variable : None, block_override : None }
    ]));
    Ok(())
}


pub fn handle_icmp(
    module    : &ParsedModule,
    function  : &mut ParsedFunction,
    dest_var  : &str,
    predicate : IntPredicate,
    operand0  : Value,
    operand1  : Value
) -> Result<(), Box<dyn Error>> {
    let op = match (predicate) {
        IntPredicate::EQ => "=",
        IntPredicate::NE => "!=",
        IntPredicate::UGT | IntPredicate::SGT => ">",
        IntPredicate::UGE | IntPredicate::SGE => ">=",
        IntPredicate::ULT | IntPredicate::SLT => "<",
        IntPredicate::ULE | IntPredicate::SLE => "<=",
    };
    let params = vec![
        operand0.to_codevalue(module, function)?,
        operand1.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("if_var", op, params, vec![ ]));
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        CodeValue::Number(String::from("1"))
    ], vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    function.line.blocks.push(Codeblock::elses());
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        CodeValue::Number(String::from("0"))
    ], vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    Ok(())
}


pub fn handle_fcmp(
    module    : &ParsedModule,
    function  : &mut ParsedFunction,
    dest_var  : &str,
    predicate : FPPredicate,
    operand0  : Value,
    operand1  : Value
) -> Result<(), Box<dyn Error>> {
    let op = match (predicate) {
        FPPredicate::False => {
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
                CodeValue::Number(String::from("0"))
            ], vec![ ]));
            return Ok(());
        },
        FPPredicate::True | FPPredicate::ORD | FPPredicate::UNO => {
            function.line.blocks.push(Codeblock::action("set_var", "=", vec![
                CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
                CodeValue::Number(String::from("1"))
            ], vec![ ]));
            return Ok(());
        },
        FPPredicate::OEQ | FPPredicate::UEQ => "=",
        FPPredicate::OGT | FPPredicate::UGT => ">",
        FPPredicate::OGE | FPPredicate::UGE => ">=",
        FPPredicate::OLT | FPPredicate::ULT => "<",
        FPPredicate::OLE | FPPredicate::ULE => "<=",
        FPPredicate::ONE | FPPredicate::UNE => "!="
    };
    let params = vec![
        operand0.to_codevalue(module, function)?,
        operand1.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("if_var", op, params, vec![ ]));
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        CodeValue::Number(String::from("1"))
    ], vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    function.line.blocks.push(Codeblock::elses());
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    function.line.blocks.push(Codeblock::action("set_var", "=", vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        CodeValue::Number(String::from("0"))
    ], vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    Ok(())
}


pub fn handle_select(
    module      : &ParsedModule,
    function    : &mut ParsedFunction, 
    dest_var    : &str,
    condition   : Value,
    true_value  : Value,
    false_value : Value
) -> Result<(), Box<dyn Error>> {
    let params = vec![
        condition.to_codevalue(module, function)?,
        CodeValue::Number("0".to_string())
    ];
    function.line.blocks.push(Codeblock::action("if_var", "=", params, vec![ ]));
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    let params = vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        false_value.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    function.line.blocks.push(Codeblock::elses());
    function.line.blocks.push(Codeblock::OPEN_COND_BRACKET);
    let params = vec![
        CodeValue::Variable { name : dest_var.to_string(), scope: VariableScope::Local },
        true_value.to_codevalue(module, function)?
    ];
    function.line.blocks.push(Codeblock::action("set_var", "=", params, vec![ ]));
    function.line.blocks.push(Codeblock::CLOSE_COND_BRACKET);
    Ok(())
}
