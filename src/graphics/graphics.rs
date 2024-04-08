use std::process::Command;
use pad::PadStr;


pub fn print_padded(string_to_pring:String, delimiter:char)
{
   print_custom_padded(string_to_pring, delimiter, 100, pad::Alignment::Left);
}

pub fn print_borderline(border_delimiter: char) {

   print_custom_padded(String::new(), border_delimiter, 100, pad::Alignment::Left);
}

pub fn print_custom_padded(string_to_pring:String, delimiter:char, padding_length:usize, alignment: pad::Alignment )
{
    println!(
        "{}",
        string_to_pring.pad( padding_length , delimiter, alignment, false)
    );
}

pub fn print_padded_to_left(string_to_pring:String, delimiter:char, padding_length:usize){
   print_custom_padded(string_to_pring, delimiter, padding_length, pad::Alignment::Left);
}

#[allow(dead_code)]
pub fn print_padded_to_right(string_to_pring:String, delimiter:char, padding_length:usize){
   print_custom_padded(string_to_pring, delimiter, padding_length, pad::Alignment::Right);
}
pub fn print_header(header_string: String) {
    print_borderline('#');
    print_custom_padded(header_string, ' ', 100, pad::Alignment::Middle);
    print_borderline('#');
}

pub fn clear_terminal_screen() {
   if cfg!(target_os = "windows") {
       Command::new("cmd")
           .args(["/c", "cls"])
           .spawn()
           .expect("cls command failed to start")
           .wait()
           .expect("failed to wait");
   } else {
       Command::new("clear")
           .spawn()
           .expect("clear command failed to start")
           .wait()
           .expect("failed to wait");
   };
}