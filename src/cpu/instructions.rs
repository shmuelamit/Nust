mod utils;
use crate::cpu::*;

pub mod branch_opcodes;
pub mod imp_opcodes;
pub mod read_opcodes;
pub mod rmw_opcodes;
pub mod routine_opcodes;
pub mod write_opcodes;

fn make_opcode(
    name: &'static str,
    execute: fn(&mut Cpu, AddresingMode),
    addresing_mode: AddresingMode,
    cycle_count: u8,
) -> Opcode {
    Opcode {
        instr: Instruction { name, execute },
        addresing_mode,
        cycle_count,
    }
}

pub fn set_opcode_table(table: &mut [Opcode; 256]) {
    table[0x69] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::IMM, 2);
    table[0x65] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::ZPG, 3);
    table[0x75] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::ZPX, 4);
    table[0x6d] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::ABS, 4);
    table[0x7d] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::ABX, 4);
    table[0x79] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::ABY, 4);
    table[0x61] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::IDX, 6);
    table[0x71] = make_opcode("ADC", read_opcodes::instr_adc, AddresingMode::IDY, 5);

    table[0x29] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::IMM, 2);
    table[0x25] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::ZPG, 3);
    table[0x35] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::ZPX, 4);
    table[0x2d] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::ABS, 4);
    table[0x3d] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::ABX, 4);
    table[0x39] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::ABY, 4);
    table[0x21] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::IDX, 6);
    table[0x31] = make_opcode("AND", read_opcodes::instr_and, AddresingMode::IDY, 5);

    table[0x0a] = make_opcode("ASL", rmw_opcodes::instr_asl, AddresingMode::ACC, 2);
    table[0x06] = make_opcode("ASL", rmw_opcodes::instr_asl, AddresingMode::ZPG, 5);
    table[0x16] = make_opcode("ASL", rmw_opcodes::instr_asl, AddresingMode::ZPX, 6);
    table[0x0e] = make_opcode("ASL", rmw_opcodes::instr_asl, AddresingMode::ABS, 6);
    table[0x1e] = make_opcode("ASL", rmw_opcodes::instr_asl, AddresingMode::ABX, 6);

    table[0x90] = make_opcode("BCC", branch_opcodes::instr_bcc, AddresingMode::REL, 2);
    table[0xB0] = make_opcode("BCS", branch_opcodes::instr_bcs, AddresingMode::REL, 2);
    table[0xF0] = make_opcode("BEQ", branch_opcodes::instr_beq, AddresingMode::REL, 2);
    table[0x30] = make_opcode("BMI", branch_opcodes::instr_bmi, AddresingMode::REL, 2);
    table[0xD0] = make_opcode("BNE", branch_opcodes::instr_bne, AddresingMode::REL, 2);
    table[0x10] = make_opcode("BPL", branch_opcodes::instr_bpl, AddresingMode::REL, 2);
    table[0x50] = make_opcode("BVC", branch_opcodes::instr_bvc, AddresingMode::REL, 2);
    table[0x70] = make_opcode("BVS", branch_opcodes::instr_bvs, AddresingMode::REL, 2);

    table[0x24] = make_opcode("BIT", read_opcodes::instr_bit, AddresingMode::ZPG, 3);
    table[0x2c] = make_opcode("BIT", read_opcodes::instr_bit, AddresingMode::ABS, 4);
    table[0x89] = make_opcode("BIT", read_opcodes::instr_bit, AddresingMode::IMM, 2);
    table[0x34] = make_opcode("BIT", read_opcodes::instr_bit, AddresingMode::ZPX, 4);
    table[0x3c] = make_opcode("BIT", read_opcodes::instr_bit, AddresingMode::ABX, 4);

    table[0x00] = make_opcode("BRK", routine_opcodes::instr_brk, AddresingMode::IMP, 7);

    table[0x18] = make_opcode("CLC", imp_opcodes::instr_clc, AddresingMode::IMP, 2);
    table[0xd8] = make_opcode("CLD", imp_opcodes::instr_cld, AddresingMode::IMP, 2);
    table[0x58] = make_opcode("CLI", imp_opcodes::instr_cli, AddresingMode::IMP, 2);
    table[0xb8] = make_opcode("CLV", imp_opcodes::instr_clv, AddresingMode::IMP, 2);

    table[0xea] = make_opcode("NOP", imp_opcodes::instr_nop, AddresingMode::IMP, 2);

    table[0x48] = make_opcode("PHA", routine_opcodes::instr_pha, AddresingMode::IMP, 3);
    table[0x68] = make_opcode("PLA", routine_opcodes::instr_pla, AddresingMode::IMP, 4);
    table[0x08] = make_opcode("PHP", routine_opcodes::instr_php, AddresingMode::IMP, 3);
    table[0x28] = make_opcode("PLP", routine_opcodes::instr_plp, AddresingMode::IMP, 4);

    table[0x40] = make_opcode("RTI", routine_opcodes::instr_rti, AddresingMode::IMP, 6);
    table[0x60] = make_opcode("RTS", routine_opcodes::instr_rts, AddresingMode::IMP, 6);

    table[0x38] = make_opcode("SEC", imp_opcodes::instr_sec, AddresingMode::IMP, 2);
    table[0xf8] = make_opcode("SED", imp_opcodes::instr_sed, AddresingMode::IMP, 2);
    table[0x78] = make_opcode("SEI", imp_opcodes::instr_sei, AddresingMode::IMP, 2);

    table[0xaa] = make_opcode("TAX", imp_opcodes::instr_tax, AddresingMode::IMP, 2);
    table[0x8a] = make_opcode("TXA", imp_opcodes::instr_txa, AddresingMode::IMP, 2);
    table[0xa8] = make_opcode("TAY", imp_opcodes::instr_tay, AddresingMode::IMP, 2);
    table[0x98] = make_opcode("TYA", imp_opcodes::instr_tya, AddresingMode::IMP, 2);
    table[0xba] = make_opcode("TSX", imp_opcodes::instr_tsx, AddresingMode::IMP, 2);
    table[0x9a] = make_opcode("TXS", imp_opcodes::instr_txs, AddresingMode::IMP, 2);

    table[0xc9] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::IMM, 2);
    table[0xc5] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::ZPG, 3);
    table[0xd5] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::ZPX, 4);
    table[0xcd] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::ABS, 4);
    table[0xdd] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::ABX, 4);
    table[0xd9] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::ABY, 4);
    table[0xc1] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::IDX, 6);
    table[0xd1] = make_opcode("CMP", read_opcodes::instr_cmp, AddresingMode::IDY, 5);
    table[0xe0] = make_opcode("CPX", read_opcodes::instr_cpx, AddresingMode::IMM, 2);
    table[0xe4] = make_opcode("CPX", read_opcodes::instr_cpx, AddresingMode::ZPG, 3);
    table[0xec] = make_opcode("CPX", read_opcodes::instr_cpx, AddresingMode::ABS, 4);
    table[0xc0] = make_opcode("CPY", read_opcodes::instr_cpy, AddresingMode::IMM, 2);
    table[0xc4] = make_opcode("CPY", read_opcodes::instr_cpy, AddresingMode::ZPG, 3);
    table[0xcc] = make_opcode("CPY", read_opcodes::instr_cpy, AddresingMode::ABS, 4);

    table[0xc6] = make_opcode("DEC", rmw_opcodes::instr_dec, AddresingMode::ZPG, 5);
    table[0xd6] = make_opcode("DEC", rmw_opcodes::instr_dec, AddresingMode::ZPX, 6);
    table[0xce] = make_opcode("DEC", rmw_opcodes::instr_dec, AddresingMode::ABS, 6);
    table[0xde] = make_opcode("DEC", rmw_opcodes::instr_dec, AddresingMode::ABX, 7);
    table[0x3a] = make_opcode("DEC", rmw_opcodes::instr_dec, AddresingMode::ACC, 2);

    table[0xca] = make_opcode("DEX", imp_opcodes::instr_dex, AddresingMode::IMP, 2);
    table[0x88] = make_opcode("DEY", imp_opcodes::instr_dey, AddresingMode::IMP, 2);
    table[0xe8] = make_opcode("INX", imp_opcodes::instr_inx, AddresingMode::IMP, 2);
    table[0xc8] = make_opcode("INY", imp_opcodes::instr_iny, AddresingMode::IMP, 2);

    table[0x49] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::IMM, 2);
    table[0x45] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::ZPG, 3);
    table[0x55] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::ZPX, 4);
    table[0x4d] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::ABS, 4);
    table[0x5d] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::ABX, 4);
    table[0x59] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::ABY, 4);
    table[0x41] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::IDX, 6);
    table[0x51] = make_opcode("EOR", read_opcodes::instr_eor, AddresingMode::IDY, 5);

    table[0xe6] = make_opcode("INC", rmw_opcodes::instr_inc, AddresingMode::ZPG, 5);
    table[0xf6] = make_opcode("INC", rmw_opcodes::instr_inc, AddresingMode::ZPX, 6);
    table[0xee] = make_opcode("INC", rmw_opcodes::instr_inc, AddresingMode::ABS, 6);
    table[0xfe] = make_opcode("INC", rmw_opcodes::instr_inc, AddresingMode::ABX, 7);
    table[0x1a] = make_opcode("INC", rmw_opcodes::instr_inc, AddresingMode::ACC, 2);

    table[0x4c] = make_opcode("JMP", routine_opcodes::instr_jmp, AddresingMode::ABS, 3);
    table[0x6c] = make_opcode("JMP", routine_opcodes::instr_jmp, AddresingMode::IND, 5);
    table[0x7c] = make_opcode("JMP", routine_opcodes::instr_jmp, AddresingMode::ABX, 6);

    table[0x20] = make_opcode("JSR", routine_opcodes::instr_jsr, AddresingMode::ABS, 6);

    table[0xa9] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::IMM, 2);
    table[0xa5] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::ZPG, 3);
    table[0xb5] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::ZPX, 4);
    table[0xad] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::ABS, 4);
    table[0xbd] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::ABX, 4);
    table[0xb9] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::ABY, 4);
    table[0xa1] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::IDX, 6);
    table[0xb1] = make_opcode("LDA", read_opcodes::instr_lda, AddresingMode::IDY, 5);
    table[0xa2] = make_opcode("LDX", read_opcodes::instr_ldx, AddresingMode::IMM, 2);
    table[0xa6] = make_opcode("LDX", read_opcodes::instr_ldx, AddresingMode::ZPG, 3);
    table[0xb6] = make_opcode("LDX", read_opcodes::instr_ldx, AddresingMode::ZPY, 4);
    table[0xae] = make_opcode("LDX", read_opcodes::instr_ldx, AddresingMode::ABS, 4);
    table[0xbe] = make_opcode("LDX", read_opcodes::instr_ldx, AddresingMode::ABY, 4);
    table[0xa0] = make_opcode("LDY", read_opcodes::instr_ldy, AddresingMode::IMM, 2);
    table[0xa4] = make_opcode("LDY", read_opcodes::instr_ldy, AddresingMode::ZPG, 3);
    table[0xb4] = make_opcode("LDY", read_opcodes::instr_ldy, AddresingMode::ZPX, 4);
    table[0xac] = make_opcode("LDY", read_opcodes::instr_ldy, AddresingMode::ABS, 4);
    table[0xbc] = make_opcode("LDY", read_opcodes::instr_ldy, AddresingMode::ABX, 4);

    table[0x4a] = make_opcode("LSR", rmw_opcodes::instr_lsr, AddresingMode::ACC, 2);
    table[0x46] = make_opcode("LSR", rmw_opcodes::instr_lsr, AddresingMode::ZPG, 5);
    table[0x56] = make_opcode("LSR", rmw_opcodes::instr_lsr, AddresingMode::ZPX, 6);
    table[0x4e] = make_opcode("LSR", rmw_opcodes::instr_lsr, AddresingMode::ABS, 6);
    table[0x5e] = make_opcode("LSR", rmw_opcodes::instr_lsr, AddresingMode::ABX, 6);

    table[0x09] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::IMM, 2);
    table[0x05] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::ZPG, 3);
    table[0x15] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::ZPX, 4);
    table[0x0d] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::ABS, 4);
    table[0x1d] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::ABX, 4);
    table[0x19] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::ABY, 4);
    table[0x01] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::IDX, 6);
    table[0x11] = make_opcode("ORA", read_opcodes::instr_ora, AddresingMode::IDY, 5);

    table[0x2a] = make_opcode("ROL", rmw_opcodes::instr_rol, AddresingMode::ACC, 2);
    table[0x26] = make_opcode("ROL", rmw_opcodes::instr_rol, AddresingMode::ZPG, 5);
    table[0x36] = make_opcode("ROL", rmw_opcodes::instr_rol, AddresingMode::ZPX, 6);
    table[0x2e] = make_opcode("ROL", rmw_opcodes::instr_rol, AddresingMode::ABS, 6);
    table[0x3e] = make_opcode("ROL", rmw_opcodes::instr_rol, AddresingMode::ABX, 6);
    table[0x6a] = make_opcode("ROR", rmw_opcodes::instr_ror, AddresingMode::ACC, 2);
    table[0x66] = make_opcode("ROR", rmw_opcodes::instr_ror, AddresingMode::ZPG, 5);
    table[0x76] = make_opcode("ROR", rmw_opcodes::instr_ror, AddresingMode::ZPX, 6);
    table[0x7e] = make_opcode("ROR", rmw_opcodes::instr_ror, AddresingMode::ABS, 6);
    table[0x6e] = make_opcode("ROR", rmw_opcodes::instr_ror, AddresingMode::ABX, 6);

    table[0xe9] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::IMM, 2);
    table[0xe5] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::ZPG, 3);
    table[0xf5] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::ZPX, 4);
    table[0xed] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::ABS, 4);
    table[0xfd] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::ABX, 4);
    table[0xf9] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::ABY, 4);
    table[0xe1] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::IDX, 6);
    table[0xf1] = make_opcode("SBC", read_opcodes::instr_sbc, AddresingMode::IDY, 5);

    table[0x85] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::ZPG, 3);
    table[0x95] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::ZPX, 4);
    table[0x8d] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::ABS, 4);
    table[0x9d] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::ABX, 5);
    table[0x99] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::ABY, 5);
    table[0x81] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::IDX, 6);
    table[0x91] = make_opcode("STA", write_opcodes::instr_sta, AddresingMode::IDY, 6);
    table[0x86] = make_opcode("STX", write_opcodes::instr_stx, AddresingMode::ZPG, 3);
    table[0x96] = make_opcode("STX", write_opcodes::instr_stx, AddresingMode::ZPY, 4);
    table[0x8e] = make_opcode("STX", write_opcodes::instr_stx, AddresingMode::ABS, 4);
    table[0x84] = make_opcode("STY", write_opcodes::instr_sty, AddresingMode::ZPG, 3);
    table[0x94] = make_opcode("STY", write_opcodes::instr_sty, AddresingMode::ZPX, 4);
    table[0x8c] = make_opcode("STY", write_opcodes::instr_sty, AddresingMode::ABS, 4);
}
