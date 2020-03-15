pub fn count_bits(num: i32) -> Vec<i32> {
    let mut bits_count = vec![0; num as usize + 1];

    for i in 0..=num {
        if i == 0 {
            bits_count[i as usize] = 0;
        } else {
            if i & (i - 1) == 0 {
                bits_count[i as usize] = 1;
            } else {
                if i % 2 != 0{
                    bits_count[i as usize] = bits_count[(i - 1) as usize] + 1;
                } else {
                    let mut one_bit_serial = 0;
                    
                    loop {
                        if (i - 1) & (1 << one_bit_serial) <= 0 {
                            break;
                        } else {
                            one_bit_serial += 1;
                        }
                    }
                    println!("{}-{}", i, one_bit_serial);
                    let step = if bits_count[(i - 1) as usize] - one_bit_serial < 0 {
                        1
                    } else {
                        bits_count[(i - 1) as usize] - one_bit_serial
                    };
                    bits_count[i as usize] = step + 1;
                }
            }
        }
    }

    bits_count
}
