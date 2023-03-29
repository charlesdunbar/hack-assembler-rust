pub fn dest(input: Option<String>) -> String {
    match input.as_deref() {
        None => String::from("000"),
        Some("M") => String::from("001"),
        Some("D") => String::from("010"),
        Some("DM") | Some("MD") => String::from("011"),
        Some("A") => String::from("100"),
        Some("AM") | Some("MA") => String::from("101"),
        Some("AD") | Some("DA") => String::from("110"),
        Some("ADM") | Some("AMD") | Some("DMA") | Some("DAM") | Some("MDA") | Some("MAD") => {
            String::from("111")
        }
        Some(_) => String::from("error"),
    }
}

pub fn comp(input: Option<String>) -> String {
    match input.as_deref() {
        None => String::from("error"),
        Some("0")   => String::from("0101010"),
        Some("1")   => String::from("0111111"),
        Some("-1")  => String::from("0111010"),
        Some("D")   => String::from("0001100"),
        Some("A")   => String::from("0110000"),
        Some("M")   => String::from("1110000"),
        Some("!D")  => String::from("0001101"),
        Some("!A")  => String::from("0110001"),
        Some("!M")  => String::from("1110001"),
        Some("-D")  => String::from("0001111"),
        Some("-A")  => String::from("0110011"),
        Some("-M")  => String::from("1110011"),
        Some("D+1") => String::from("0011111"),
        Some("A+1") => String::from("0110111"),
        Some("M+1") => String::from("1110111"),
        Some("D-1") => String::from("0001110"),
        Some("A-1") => String::from("0110010"),
        Some("M-1") => String::from("1110010"),
        Some("D+A") => String::from("0000010"),
        Some("D+M") => String::from("1000010"),
        Some("D-A") => String::from("0010011"),
        Some("D-M") => String::from("1010011"),
        Some("A-D") => String::from("0000111"),
        Some("M-D") => String::from("1000111"),
        Some("D&A") => String::from("0000000"),
        Some("D&M") => String::from("1000000"),
        Some("D|A") => String::from("0010101"),
        Some("D|M") => String::from("1010101"),
        Some(_) => String::from("error"),
    }
}

pub fn jump(input: Option<String>) -> String {
    match input.as_deref() {
        None => String::from("000"),
        Some("JGT") => String::from("001"),
        Some("JEQ") => String::from("010"),
        Some("JGE") => String::from("011"),
        Some("JLT") => String::from("100"),
        Some("JNE") => String::from("101"),
        Some("JLE") => String::from("110"),
        Some("JMP") => String::from("111"),
        Some(_) => String::from("error"),
    }
}
