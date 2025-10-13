#![allow(clippy::needless_continue, clippy::redundant_else)]

pub fn dangerous_url(s: &[u8]) -> Option<usize> {
    let mut cursor = 0;
    let mut marker = 0;
    let len = s.len();

    {
        #[allow(unused_assignments)]
        let mut yych: u8 = 0;
        let mut yyaccept: usize = 0;
        let mut yystate: usize = 0;
        'yyl: loop {
            match yystate {
                0 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    cursor += 1;
                    match yych {
                        0x44 | 0x64 => {
                            yystate = 3;
                            continue 'yyl;
                        }
                        0x46 | 0x66 => {
                            yystate = 4;
                            continue 'yyl;
                        }
                        0x4A | 0x6A => {
                            yystate = 5;
                            continue 'yyl;
                        }
                        0x56 | 0x76 => {
                            yystate = 6;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 1;
                            continue 'yyl;
                        }
                    }
                }
                1 => {
                    yystate = 2;
                    continue 'yyl;
                }
                2 => {
                    return None;
                }
                3 => {
                    yyaccept = 0;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x41 | 0x61 => {
                            cursor += 1;
                            yystate = 7;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 2;
                            continue 'yyl;
                        }
                    }
                }
                4 => {
                    yyaccept = 0;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x49 | 0x69 => {
                            cursor += 1;
                            yystate = 9;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 2;
                            continue 'yyl;
                        }
                    }
                }
                5 => {
                    yyaccept = 0;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x41 | 0x61 => {
                            cursor += 1;
                            yystate = 10;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 2;
                            continue 'yyl;
                        }
                    }
                }
                6 => {
                    yyaccept = 0;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x42 | 0x62 => {
                            cursor += 1;
                            yystate = 11;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 2;
                            continue 'yyl;
                        }
                    }
                }
                7 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x54 | 0x74 => {
                            cursor += 1;
                            yystate = 12;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                8 => {
                    cursor = marker;
                    if yyaccept == 0 {
                        yystate = 2;
                        continue 'yyl;
                    } else {
                        yystate = 20;
                        continue 'yyl;
                    }
                }
                9 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x4C | 0x6C => {
                            cursor += 1;
                            yystate = 13;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                10 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x56 | 0x76 => {
                            cursor += 1;
                            yystate = 14;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                11 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x53 | 0x73 => {
                            cursor += 1;
                            yystate = 15;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                12 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x41 | 0x61 => {
                            cursor += 1;
                            yystate = 16;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                13 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x45 | 0x65 => {
                            cursor += 1;
                            yystate = 17;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                14 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x41 | 0x61 => {
                            cursor += 1;
                            yystate = 11;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                15 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x43 | 0x63 => {
                            cursor += 1;
                            yystate = 18;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                16 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x3A => {
                            cursor += 1;
                            yystate = 19;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                17 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x3A => {
                            cursor += 1;
                            yystate = 21;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                18 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x52 | 0x72 => {
                            cursor += 1;
                            yystate = 22;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                19 => {
                    yyaccept = 1;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x49 | 0x69 => {
                            cursor += 1;
                            yystate = 23;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 20;
                            continue 'yyl;
                        }
                    }
                }
                20 => {
                    return Some(cursor);
                }
                21 => {
                    yystate = 20;
                    continue 'yyl;
                }
                22 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x49 | 0x69 => {
                            cursor += 1;
                            yystate = 24;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                23 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x4D | 0x6D => {
                            cursor += 1;
                            yystate = 25;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                24 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x50 | 0x70 => {
                            cursor += 1;
                            yystate = 26;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                25 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x41 | 0x61 => {
                            cursor += 1;
                            yystate = 27;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                26 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x54 | 0x74 => {
                            cursor += 1;
                            yystate = 17;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                27 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x47 | 0x67 => {
                            cursor += 1;
                            yystate = 28;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                28 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x45 | 0x65 => {
                            cursor += 1;
                            yystate = 29;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                29 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x2F => {
                            cursor += 1;
                            yystate = 30;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                30 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x47 | 0x67 => {
                            cursor += 1;
                            yystate = 31;
                            continue 'yyl;
                        }
                        0x4A | 0x6A => {
                            cursor += 1;
                            yystate = 32;
                            continue 'yyl;
                        }
                        0x50 | 0x70 => {
                            cursor += 1;
                            yystate = 33;
                            continue 'yyl;
                        }
                        0x57 | 0x77 => {
                            cursor += 1;
                            yystate = 34;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                31 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x49 | 0x69 => {
                            cursor += 1;
                            yystate = 35;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                32 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x50 | 0x70 => {
                            cursor += 1;
                            yystate = 36;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                33 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x4E | 0x6E => {
                            cursor += 1;
                            yystate = 37;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                34 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x45 | 0x65 => {
                            cursor += 1;
                            yystate = 38;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                35 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x46 | 0x66 => {
                            cursor += 1;
                            yystate = 39;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                36 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x45 | 0x65 => {
                            cursor += 1;
                            yystate = 37;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                37 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x47 | 0x67 => {
                            cursor += 1;
                            yystate = 39;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                38 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x42 | 0x62 => {
                            cursor += 1;
                            yystate = 40;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                39 => {
                    return None;
                }
                40 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x50 | 0x70 => {
                            cursor += 1;
                            yystate = 39;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 8;
                            continue 'yyl;
                        }
                    }
                }
                _ => {
                    eprintln!("Warning: Unexpected token in lexer");
                    return None;
                }
            }
        }
    }
}
