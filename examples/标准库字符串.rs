fn  main(){
    let pangram :&'static str = "the quick brown fox jumps over the lazy dog ?";
    println!("Pang ram:{}",pangram);
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev(){

        println!("> {}",word);
    }
  //  复制字符串到一个vector ,排序并移除重复的值
    let mut chars :Vec<char> = pangram.chars().collect();
    println!("chars is {:?}",chars);
    chars.sort();
    println!("chars is {:?}",chars);
    chars.dedup();
    println!("chars is {:?}",chars);

    let mut string = String::new();
    for c in chars{
        string.push(c);
        string.push_str(", ")
    }
    println!("string :{}",string);

    let chars_to_trim:&[char] = &[' ',' ',','];
    let trimmed_str:&str = string.trim_matches(chars_to_trim);
    println!(" Used characters:{}",trimmed_str);
    let alice = String::from("I like dogs");
    let bob:String = alice.replace("dog","cat");
    println!("Alice says:{}",alice);
    println!("Bob says :{}",bob);



    // 转义
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {}",byte_escape);
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint,character_name);

    let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";

    println!("{}",long_string);

    let raw_str = r"Escapes don't word here:\x3F \u{211D} ";
    println!("{}",raw_str);
    // 如果要在原始字符串加" 请在两边加#
    let quotes = r#"And then I said:" There is no eacape!""#;
    println!("{}",quotes);
    let bytestring :&[u8;20] = b"this is a tytestring";
    println!("A bytestring :{:?}", bytestring);
    let escaped = b"\x52\x57\x73\x74 as bytes";
    println!("Some escaped bytes:{:?}", escaped);


    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}",raw_bytestring);
    // 把字符串转正&str 可能失败
    if let Ok(my_str) = std::str::from_utf8(raw_bytestring){
        println!("And the same as test:'{}'",my_str);
    };
    let quotes = br#"You can alse use "fancier" formatting,\
                            like with normal raw strings"#;
    println!("quotes:{:?}",quotes);
    let shift_jis = b"\x82\xe6\x83\xa8\x82\xb1\x82";
    match std::str::from_utf8(shift_jis){
        Ok(my_str)=>println!("Conversion successful: '{}'",my_str),
        Err(e)=>println!("Conversion failed:{:?}",e),
    }

}