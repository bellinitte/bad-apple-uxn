pub fn encode(frame: Vec<bool>) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    let mut run_length: u8 = 0;
    let mut previous_color = false;

    for current_color in frame {
        if current_color == previous_color {
            match run_length.checked_add(1) {
                Some(new_run_length) => run_length = new_run_length,
                None => {
                    encoded.push(u8::MAX);
                    encoded.push(0);
                    run_length = 1;
                }
            }
        } else {
            previous_color = current_color;
            encoded.push(run_length);
            run_length = 1;
        }
    }

    encoded.push(run_length);

    return encoded;
}
