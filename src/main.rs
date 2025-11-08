fn main() {
    let enc_lookup_table: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // let input_bytes = vec![0x53, 0x75, 0x6E, 0x53, 0x75];
    let mut sb = String::new();

    println!("Enter text:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end();
    let input_bytes = input.as_bytes().to_vec();

    let idx_last_triple = (input_bytes.len() / 3) * 3;
    for i in (0..idx_last_triple).step_by(3) {
        let idx1 = (input_bytes[i] >> 2) as usize;
        let idx2 = (((input_bytes[i] & 0x03) << 4) | ((input_bytes[i + 1] & 0xF0) >> 4)) as usize;
        let idx3 =
            (((input_bytes[i + 1] & 0x0F) << 2) | ((input_bytes[i + 2] & 0xC0) >> 6)) as usize;
        let idx4 = (input_bytes[i + 2] & 0x3F) as usize;
        sb.push(enc_lookup_table.chars().nth(idx1).unwrap());
        sb.push(enc_lookup_table.chars().nth(idx2).unwrap());
        sb.push(enc_lookup_table.chars().nth(idx3).unwrap());
        sb.push(enc_lookup_table.chars().nth(idx4).unwrap());
    }
    if idx_last_triple < input_bytes.len() {
        let idx1 = input_bytes[idx_last_triple];
        let idx2 = if idx_last_triple + 1 < input_bytes.len() {
            input_bytes[idx_last_triple + 1]
        } else {
            0
        };
        sb.push(enc_lookup_table.chars().nth((idx1 >> 2) as usize).unwrap());
        sb.push(
            enc_lookup_table
                .chars()
                .nth(((idx1 & 0x03) << 4 | ((idx2 & 0xF0) >> 4)) as usize)
                .unwrap(),
        );

        if idx_last_triple + 1 < input_bytes.len() {
            sb.push(
                enc_lookup_table
                    .chars()
                    .nth(((idx2 & 0x0F) << 2) as usize)
                    .unwrap(),
            );
        } else {
            sb.push('=');
        }

        sb.push('=');
    }
    println!("Base64 Output:");
    println!("{}", sb);
}
