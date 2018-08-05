#[derive(Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    HLT = 0,
    LOAD = 1,
    ADD = 2,
    SUB = 3,
    DIV = 4,
    MUL = 5,
    GT = 6,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match num::FromPrimitive::from_u8(v) {
            Some(opcode) => opcode,
            None => panic!("Illegal opcode."),
        }
    }
}

impl Instruction {
    pub fn new(opcode: Opcode, byte1: u8, byte2: u8, byte3: u8) -> Instruction {
        Instruction {
            opcode,
            byte1,
            byte2,
            byte3,
        }
    }
}

pub fn instructions_to_bytes(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut byte_vec: Vec<u8> = vec![];
    for instruction in instructions {
        let instr_bytes = vec![instruction.opcode as u8, instruction.byte1, instruction.byte2, instruction.byte3];
        byte_vec.extend(instr_bytes);
    }

    return byte_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm::VM;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    #[should_panic]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        let instrs = vec![
            Instruction::new(Opcode::LOAD, 1, 0, 2),
            Instruction::new(Opcode::LOAD, 3, 0, 5),
            Instruction::new(Opcode::ADD, 1, 3, 6),
        ];
        let test_bytes = instructions_to_bytes(instrs);

        test_vm.program = test_bytes;
        test_vm.run();

        assert_eq!(test_vm.registers[6], 7)
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        let instrs = vec![
            Instruction::new(Opcode::LOAD, 1, 0, 2),
            Instruction::new(Opcode::LOAD, 3, 0, 5),
            Instruction::new(Opcode::MUL, 1, 3, 6),
        ];
        let test_bytes = instructions_to_bytes(instrs);

        test_vm.program = test_bytes;
        test_vm.run();

        assert_eq!(test_vm.registers[6], 10)
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        let instrs = vec![
            Instruction::new(Opcode::LOAD, 1, 0, 25),
            Instruction::new(Opcode::LOAD, 3, 0, 17),
            Instruction::new(Opcode::SUB, 1, 3, 6),
        ];
        let test_bytes = instructions_to_bytes(instrs);

        test_vm.program = test_bytes;
        test_vm.run();

        assert_eq!(test_vm.registers[6], 8)
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        let instrs = vec![
            Instruction::new(Opcode::LOAD, 1, 0, 130),
            Instruction::new(Opcode::LOAD, 3, 0, 16),
            Instruction::new(Opcode::GT, 1, 3, 3),
            Instruction::new(Opcode::LOAD, 4, 0, 19),
            Instruction::new(Opcode::LOAD, 18, 0, 25),
        ];
        let test_bytes = instructions_to_bytes(instrs);

        test_vm.program = test_bytes;
        test_vm.run();

        assert_ne!(test_vm.registers[4], 19);
        assert_eq!(test_vm.registers[18], 25);
    }
}
