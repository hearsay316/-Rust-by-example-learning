#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    // 错误! 不提供隐藏转换
    let _integer: u8 = decimal as u8;
    // 可以显示转换
    let integer = decimal as u32;
    let character = integer as u8 as char;
    println!("Casting : {} -> {} -> {}", decimal, integer, character);
    // 当把类型转换为无符号类型T 时, 会不断加上或减去 (std::T::Max+1);
    // 直到值 位于新类型T的范围之内.
    println!("100 as u16 is : {}", 1000 as u16);
    // 1000 - 256 -256 -256  = 232   256 = 255+1
    println!("1000 as a u8 is : {}", 1000i32 as u8);
    println!("-1 as a u8: {}", (-1i8) as u8); // -1+256 = 255
                                              // 对于正数,这就和取模一样
    println!("1000 mod 256 is: {}", 1000 % 256);
    println!(" -1 as u16 : {}", (-1i8) as u16);
    // 如果数值已经在目标类型的范围内, 就直接把它放进去
    println!("128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8); // -128   128-128
                                                 // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
                                                 // In boundary case 128 value in 8-bit two's complement representation is -128
    println!(" 128 as a i8 is : {}", std::i8::MIN);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    // 正转负  负转正
    // Unless it already fits, of course.
    println!("1000 as a u8 is :{}", 1000 as u8);
    println!("1000 as a i8 is :{}", -1000 as i8);
    //error[E0604]: only `u8` can be cast as `char`, not `f32`
    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
}
