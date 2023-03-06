pub fn clean_version(name: &str, version: &str) -> String {
    let mut chars = version.chars();
    let mut result = String::new();

    if version == "*" || version == "latest" {
        let url = format!("https://registry.npmjs.org/{}/latest", name);
        let response = reqwest::blocking::get(&url).unwrap().json::<serde_json::Value>().unwrap();
        return response["version"].as_str().unwrap().to_string();
    }

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            result.push(c);
            break;
        } else if c == '^' || c == '~' {
            continue;
        } else if c == '>' || c == '<' || c == '=' {
            result.push(c);
            if let Some('=') = chars.clone().next() {
                result.push('=');
                chars.next();
            }
        } else if c == '|' {
            result.push_str("||");
            break;
        } else if c == ' ' {
            continue;
        }
    }
    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            result.push(c);
        } else if c == '-' {
            result.push_str("-");
            break;
        } else {
            break;
        }
    }
    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            result.push(c);
        } else {
            break;
        }
    }
    while let Some(c) = chars.next() {
        if c == '|' {
            result.push_str("||");
            break;
        } else if c == ',' {
            result.push_str(",");
        } else if c == ' ' {
            continue;
        } else if c == '&' {
            result.push_str("&&");
        } else if c == '-' {
            result.push_str("-");
        } else if c == '>' || c == '<' || c == '=' {
            result.push(c);
            if let Some('=') = chars.clone().next() {
                result.push('=');
                chars.next();
            }
        }
    }

    // If string is empty return original string
    if result.is_empty() {
        return version.to_string();
    } else {
        return result.replace(">=", "");
    }
}