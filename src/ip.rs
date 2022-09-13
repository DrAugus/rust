
enum IpAddress {
    Ipv4Address(String),
    Ipv6Address(String),
}

fn check_ipv4_address(address: &String) -> ([u8; 4], bool) {
    let mut is_ipv4 = false;
    if address.contains('.') { is_ipv4 = true }
    if !is_ipv4 { return ([0, 0, 0, 0], false); }

    let mut temp = address;
    let mut index_arr = [0; 3];
    let mut index = 0;
    for (i, char) in temp.chars().enumerate() {
        if char == '.' {
            index_arr[index] = i;
            index = index + 1;
        }
    }
    dbg!(index_arr);
    let mut value_ipv4 = [
        temp[0..index_arr[0]].parse().unwrap(),
        temp[index_arr[0] + 1..index_arr[1]].parse().unwrap(),
        temp[index_arr[1] + 1..index_arr[2]].parse().unwrap(),
        temp[index_arr[2] + 1..temp.len()].parse().unwrap(),
    ];
    dbg!(value_ipv4);
    for i in value_ipv4 {
        if i > 255 || i < 1 {
            ([0, 0, 0, 0], false);
        }
    }
    (value_ipv4, true)
}

pub(crate) fn ip_switch(address: &String, standard: bool) -> String {
    let check_ipv4 = check_ipv4_address(address);
    let is_ipv4 = check_ipv4.1;
    let ipv4_value = check_ipv4.0;

    let mut res: String = "".to_string();

    if is_ipv4 == true {
        if standard == false {
            res = "::".to_string() + address;
            dbg!(&res);
        } else {
            for i in ipv4_value {
                res += &*format!("{:02X}", i);
            }
            res.insert(4, ':');
            res.insert_str(0, "::");
            dbg!(&res);
        }
    } else {
        dbg!("ipv6 waiting");
    }

    return res;
}
