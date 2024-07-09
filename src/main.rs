use std::io;
use prettytable::{format, row, Table};
use regex::Regex;

fn main() {
    display();
    let version = is_new_version();
    let group_by = parse_group_by();
    let grouping_sets = parse_grouping_sets();
    computer_grouping_id(version, group_by, grouping_sets);
    enter_any_exit();
}


fn is_new_version() -> bool {
    // 接受一行命令行输入
    eprint!("是否启动新版本计算逻辑[y/n]: ");
    let mut is_version_str = String::new();
    match io::stdin().read_line(&mut is_version_str) {
        Ok(_) => {}
        Err(err) => {
            panic!("[{}] - {}", err.kind(), err)
        }
    }
    is_version_str = is_version_str.trim().to_lowercase();
    if is_version_str == "y" {
        return true;
    } else if is_version_str == "n" {
        return false;
    } else {
        panic!("期望 y or n but get {}", is_version_str)
    }
}

fn parse_group_by() -> Vec<String> {
    eprint!("输入 group by 后的列表[以;结尾]: ");
    let group_by_str = read_with_char(";");
    let mut vec: Vec<String> = Vec::new();
    for x in group_by_str.split(",") {
        vec.push(x.trim().to_lowercase())
    }
    return vec;
}

fn parse_grouping_sets() -> Vec<Vec<String>> {
    println!("输入 grouping sets (*)的列表[以;结尾，按回车分割]: ");
    // 使用正则提取
    let re = Regex::new(r"\(([^)]*)\)").unwrap();
    let mut result = Vec::new();
    let grouping_sets = read_with_char(";");
    for x in grouping_sets.split("\n") {
        for cap in re.captures_iter(x) {
            let mut vec = Vec::new();
            for set in cap[1].split(",") {
                vec.push(set.trim().to_lowercase())
            }
            result.push(vec)
        }
    }
    return result;
}

fn computer_grouping_id(is_new_version: bool, group_by: Vec<String>, grouping_sets: Vec<Vec<String>>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    // 设置表头
    table.set_titles(row!["grouping__id","grouping sets"]);
    for set in grouping_sets {
        let grouping_id = _computer_grouping_id(is_new_version, &group_by, &set);
        table.add_row(row![grouping_id, format!("{:?}",set)]);
    }
    table.printstd();
}

fn _computer_grouping_id(is_new_version: bool, group_by: &Vec<String>, grouping_set: &Vec<String>) -> isize {
    let mut grouping_id_str = String::new();
    for x in group_by {
        let exist = grouping_set.contains(&x);
        if is_new_version {
            if exist {
                grouping_id_str.push_str("0")
            } else {
                grouping_id_str.push_str("1")
            }
        } else {
            if exist {
                grouping_id_str.push_str("1")
            } else {
                grouping_id_str.push_str("0")
            }
        }
    }
    if !is_new_version {
        // 反转字符串
        grouping_id_str = reverse_string(grouping_id_str.as_str())
    }

    return isize::from_str_radix(&grouping_id_str, 2).unwrap();
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn read_with_char(end: &str) -> String {
    let mut result = String::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                result.push_str(line.as_str());
                if line.trim().ends_with(end) {
                    result = result.replace(end, "");
                    break;
                }
            }
            Err(err) => {
                panic!("[{}] - {}", err.kind(), err)
            }
        }
    }
    return result;
}

fn enter_any_exit() {
    println!("按任意键结束...");
    let mut flag = String::new();
    io::stdin().read_line(&mut flag).unwrap();
}


fn display() {
    println!("fast-gid v1.0 for rust");
    println!("快速计算 hsql/sparkSql 高阶聚合的 grouping__id");
    println!();
}