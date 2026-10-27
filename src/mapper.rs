use regex::Regex;

pub fn char_loopup(str_data: Vec<char>, end_pattern: &str, types: i64) -> String {
    let mut raw_str = String::new();
    let end_char = end_pattern.chars().next().unwrap_or_default();

    for cc in str_data {
        if types == 1 {
            if end_char == cc {
                break;
            }
            raw_str.push(cc);
        } else if types == 2 {
            let re = Regex::new(end_pattern).unwrap();
            if re.is_match(&cc.to_string()) {
                break;
            }
            raw_str.push(cc);
        }
    }

    raw_str
}

pub fn map_vector_loopup(_str_data: Vec<char>, vector_search: Vec<String>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let re = Regex::new(r"^\d{1,}..\d{1,}$").unwrap();

    for (_kk, vv) in vector_search.iter().enumerate() {
        if re.is_match(vv) {
            let range_parts: Vec<String> = vv.split(&"..".to_string()).map(str::to_owned).collect();

            let first_num: i64 = range_parts[0].parse().unwrap();
            let second_num: i64 = range_parts[1].parse().unwrap();
            for i in first_num..=second_num {
                tokens.push(i.to_string());
            }
        } else {
            tokens.push(vv.to_string());
        }
    }

    tokens
}

pub fn char_vector_loopup(str_data: Vec<char>, vector_search: Vec<String>) -> String {
    let tokens: Vec<String> = map_vector_loopup(str_data.clone(), vector_search);

    let mut prp_st = String::from("");
    let mut fin_st = String::from("");

    for (_kk, vv) in str_data.iter().enumerate() {
        prp_st.push_str(&vv.to_string());
        if tokens.contains(&String::from(prp_st.clone())) {
            fin_st = prp_st;
            break;
        }
    }
    fin_st.to_string()
}


