use crate::Value;

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Op {
    // 2 stack operands
    Add = 0,
    Sub,
    Eq,
    Lte,
    // 2 stack operands
    Intersect,
    // 2 stack operands
    Union,
    // N stack operands
    Print,
    // 1 instr operand
    Constant,
    Pop,
    // N stack operands (args) + 1 stack operand (fn ident) + 1 instr for count
    Call,
    TailCall,
    // 2 args, 2 jump instrs
    Extends,
    PanicExtends,
    Jump,
    Number,
    String,
    PopCallFrame,
    // next instr is fields
    MakeObj,
    SetLocal,
    GetLocal,
    SetGlobal,
    GetGlobal,
}

const OP_MAX_VAL: u8 = Op::GetGlobal as u8;

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        return unsafe { std::mem::transmute(value) };
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            code: Default::default(),
            constants: Default::default(),
        }
    }
}

impl Chunk {
    pub fn push_op(&mut self, op: Op) {
        self.push_u8(op as u8)
    }

    pub fn push_u8(&mut self, val: u8) {
        self.code.push(val)
    }

    pub fn push_constant(&mut self, val: Value) -> usize {
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }

    pub fn push_bytes(&mut self, a: u8, b: u8) {
        self.push_u8(a);
        self.push_u8(b);
    }

    pub fn debug_code(&self) {
        let mut i: usize = 0;
        if self.code.is_empty() {
            return;
        }

        while i < self.code.len() {
            let op: Op = self.code[i].into();
            i += 1;
            match op {
                Op::Lte => {
                    println!("{} LTE", i);
                }
                Op::Eq => {
                    println!("{} EQ", i);
                }
                Op::Number => {
                    println!("{} Number", i);
                }
                Op::String => {
                    println!("{} String", i);
                }
                Op::Add => {
                    println!("{} ADD", i);
                }
                Op::Sub => {
                    println!("{} SUB", i);
                }
                Op::Intersect => {
                    let count = self.code[i];
                    i += 1;
                    println!("{} INTERSECT: {}", i, count)
                }
                Op::Union => todo!(),
                Op::Print => {
                    let count = self.code[i];
                    i += 1;
                    println!("{} Print: {}", i, count)
                }
                Op::Constant => {
                    let idx = self.code[i];
                    i += 1;
                    println!("{} CONST: {:?}", i, self.constants[idx as usize]);
                }
                Op::Pop => todo!(),
                Op::TailCall | Op::Call => {
                    let count = self.code[i];
                    i += 1;
                    let name_idx = self.code[i];
                    i += 1;
                    println!(
                        "{} {:?} {:?} {:?}",
                        i, op, count, self.constants[name_idx as usize]
                    )
                }
                Op::SetLocal => {
                    let idx = self.code[i];
                    i += 1;
                    println!("{} Set local {:?}", i, idx);
                }
                Op::GetLocal => {
                    let idx = self.code[i];
                    i += 1;
                    println!("{} Get local {:?}", i, idx);
                }
                Op::SetGlobal => {
                    let idx = self.code[i];
                    i += 1;
                    println!("{} SET GLOBAL {:?}", i, self.constants[idx as usize]);
                }
                Op::GetGlobal => {
                    let idx = self.code[i];
                    i += 1;
                    println!("{} GET GLOBAL {:?}", i, self.constants[idx as usize]);
                }
                Op::PanicExtends | Op::Extends => {
                    let skip_then = ((self.code[i] as u16) << 8) | (self.code[i + 1] as u16);
                    i += 2;
                    println!(
                        "{} {} EXTENDS (skip_then={})",
                        i,
                        if op == Op::PanicExtends { "panic" } else { "" },
                        skip_then
                    )
                }
                Op::Jump => {
                    let offset = ((self.code[i] as u16) << 8) | (self.code[i + 1] as u16);
                    i += 2;
                    println!("{} JUMP {}", i, offset)
                }
                Op::PopCallFrame => {
                    println!("{} POP CALL FRAME", i);
                }
                Op::MakeObj => {
                    let count = self.code[i];
                    i += 1;
                    println!("{} Make obj {:?}", i, count);
                }
            }
        }
    }
}
