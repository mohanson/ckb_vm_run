use ckb_vm::{
    instructions::{extract_opcode, insts},
    Instruction,
};

pub fn instruction_cycles(i: Instruction) -> u64 {
    match extract_opcode(i) {
        // IMC
        insts::OP_JALR => 3,
        insts::OP_LD => 2,
        insts::OP_LW => 3,
        insts::OP_LH => 3,
        insts::OP_LB => 3,
        insts::OP_LWU => 3,
        insts::OP_LHU => 3,
        insts::OP_LBU => 3,
        insts::OP_SB => 3,
        insts::OP_SH => 3,
        insts::OP_SW => 3,
        insts::OP_SD => 2,
        insts::OP_BEQ => 3,
        insts::OP_BGE => 3,
        insts::OP_BGEU => 3,
        insts::OP_BLT => 3,
        insts::OP_BLTU => 3,
        insts::OP_BNE => 3,
        insts::OP_EBREAK => 500,
        insts::OP_ECALL => 500,
        insts::OP_JAL => 3,
        insts::OP_MUL => 5,
        insts::OP_MULW => 5,
        insts::OP_MULH => 5,
        insts::OP_MULHU => 5,
        insts::OP_MULHSU => 5,
        insts::OP_DIV => 32,
        insts::OP_DIVW => 32,
        insts::OP_DIVU => 32,
        insts::OP_DIVUW => 32,
        insts::OP_REM => 32,
        insts::OP_REMW => 32,
        insts::OP_REMU => 32,
        insts::OP_REMUW => 32,

        // MOP
        insts::OP_WIDE_MUL => 5,
        insts::OP_WIDE_MULU => 5,
        insts::OP_WIDE_MULSU => 5,
        insts::OP_WIDE_DIV => 32,
        insts::OP_WIDE_DIVU => 32,
        insts::OP_FAR_JUMP_REL => 3,
        insts::OP_FAR_JUMP_ABS => 3,
        _ => 1,
    }
}
