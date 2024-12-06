pub mod u1;
pub mod u10;
pub mod u11;
pub mod u12;
pub mod u13;
pub mod u14;
pub mod u15;
pub mod u16;
pub mod u17;
pub mod u18;
pub mod u19;
pub mod u2;
pub mod u20;
pub mod u21;
pub mod u22;
pub mod u23;
pub mod u24;
pub mod u25;
pub mod u26;
pub mod u27;
pub mod u28;
pub mod u29;
pub mod u3;
pub mod u30;
pub mod u31;
pub mod u32;
pub mod u33;
pub mod u34;
pub mod u35;
pub mod u36;
pub mod u37;
pub mod u38;
pub mod u39;
pub mod u4;
pub mod u40;
pub mod u41;
pub mod u42;
pub mod u43;
pub mod u44;
pub mod u45;
pub mod u46;
pub mod u47;
pub mod u48;
pub mod u5;
pub mod u6;
pub mod u7;
pub mod u8;
pub mod u9;
mod utils;

fn main() {
    let ulohy = vec![
        u1::main,
        u2::main,
        u3::main,
        u4::main,
        u5::main,
        u6::main,
        u7::main,
        u8::main,
        u9::main,
        u10::main,
        u11::main,
        u12::main,
        u13::main,
        u14::main,
        u15::main,
        u16::main,
        u17::main,
        u18::main,
        u19::main,
        u20::main,
        u21::main,
        u22::main,
        u23::main,
        u24::main,
        u25::main,
        u26::main,
        u27::main,
        u28::main,
        u29::main,
        u30::main,
        u31::main,
        u32::main,
        u33::main,
        u34::main,
        u35::main,
        u36::main,
        u37::main,
        u38::main,
        u39::main,
        u40::main,
        u41::main,
        u42::main,
        u43::main,
        u44::main,
        u45::main,
        u46::main,
        u47::main,
        u48::main,
    ];
    println!("Zadaj cislo ulohy:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let i = input.trim().parse::<usize>().unwrap();
    ulohy[i - 1]()
}
