use instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub pc: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        eprintln!("opcode = {:#?}", opcode);
        self.pc += 1;
        return opcode;
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }


    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::DIV => {
                let dividend =  self.registers[self.next_8_bits() as usize];
                let divisor =  self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = dividend / divisor;
                self.remainder = (dividend % divisor) as u32;
            },
            Opcode::MUL => {
                let mul1 = self.registers[self.next_8_bits() as usize];
                let mul2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = mul1 * mul2;
            },
            Opcode::SUB => {
                let s1 = self.registers[self.next_8_bits() as usize];
                let s2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = s1 - s2;
            },
            Opcode::GT => {
                let s1 = self.registers[self.next_8_bits() as usize];
                let s2 = self.registers[self.next_8_bits() as usize];
                if s1 > s2 {
                    self.pc = self.registers[self.next_8_bits() as usize] as usize;
                }
            }
        }
        false
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_load_opcode() {
      let mut test_vm = VM::new();
      test_vm.program = vec![1, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
      test_vm.run();
      assert_eq!(test_vm.registers[0], 500);
    }
}
