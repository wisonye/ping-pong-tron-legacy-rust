use raylib::prelude::Color;
///
///
///
///
pub fn color_to_hex_str(color: &Color) -> String {
    let mut result = String::with_capacity(10);

    let r_hex = format!("{:02X}", color.r);
    let g_hex = format!("{:02X}", color.g);
    let b_hex = format!("{:02X}", color.b);
    let a_hex = format!("{:02X}", color.a);

    result.push_str("0x");
    result.push_str(&r_hex);
    result.push_str(&g_hex);
    result.push_str(&b_hex);
    result.push_str(&a_hex);

    result
}
