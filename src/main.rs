use std::io::Read;
use std::{
    fs::{self, File},
    path::Path,
};

use query::{Node, Nodes, Tokens};
use simple16::{dump_with_size, load_with_size};

fn main() {
    let mut buf = vec![];
    let path = Path::new(r"C:\Users\sumiy\Desktop\work\tagdb\data\all_comp_size.bin");
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mut data1 = vec![];
    unsafe { load_with_size(&buf, &mut data1) }.unwrap();
    let path = Path::new(r"C:\Users\sumiy\Desktop\work\tagdb\data\f_comp_size.bin");
    buf.clear();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mut data2 = vec![];
    unsafe { load_with_size(&buf, &mut data2) }.unwrap();

    println!("{:?}", data1.len());
    println!("{:?}", data2.len());
    let count = 1000;

    let start = std::time::Instant::now();
    let mut i = 0;
    for _ in 0..count {
        let res = simple_and(&[&data1, &data2]);
        i += res.capacity();
    }
    println!(
        "{:?}ms {:?}",
        start.elapsed().as_millis() as f64 / count as f64,
        i
    );

    let start = std::time::Instant::now();
    let mut i = 0;
    for _ in 0..count {
        let res = simple_and_temp(&[&data1, &data2]);
        i += res.capacity();
    }
    println!(
        "{:?}ms {:?}",
        start.elapsed().as_millis() as f64 / count as f64,
        i
    );

    let start = std::time::Instant::now();
    let mut i = 0;
    let mut buf = Vec::with_capacity(52568);
    for _ in 0..count {
        let res = unsafe { and2(&data1, &data2, &mut buf) };
        i += buf.capacity();
    }
    println!(
        "{:?}ms {:?}",
        start.elapsed().as_millis() as f64 / count as f64,
        i
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum Data {
    Data(Vec<u32>),
    Not(Vec<u32>),
    None,
    ALL,
}

fn tag(category: &str, tag: &str) -> Option<Vec<u32>> {
    None
}

pub unsafe fn check_range(data: &[&Vec<u32>], indexes: &[usize]) -> bool {
    for i in 0..indexes.len() {
        if *indexes.get_unchecked(i) >= data.get_unchecked(i).len() {
            return false;
        }
    }
    true
}

unsafe fn and2(data1: &[u32], data2: &[u32], result: &mut Vec<u32>) {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut index = 0;
    let len1 = data1.len();
    let len2 = data2.len();
    while i1 < len1 && i2 < len2 {
        if data1.get_unchecked(i1) == data2.get_unchecked(i2) {
            *result.get_unchecked_mut(index) = *data1.get_unchecked(i1);
            i1 += 1;
            i2 += 1;
            index += 1;
        } else if data1.get_unchecked(i1) < data2.get_unchecked(i2) {
            i1 += 1;
        } else {
            i2 += 1;
        }
    }
    result.set_len(index);
}

fn simple_and_temp(datas: &[&Vec<u32>]) -> Vec<u32> {
    if datas.is_empty() {
        return Vec::new();
    } else if datas.len() == 1 {
        return datas[0].clone();
    } else if datas.len() == 2 {
        let mut result = Vec::with_capacity(std::cmp::min(datas[0].len(), datas[1].len()));
        unsafe { and2(datas[0], datas[1], &mut result) };
        return result;
    }
    let len = datas.iter().map(|d| d.len()).max().unwrap();
    let mut buf1 = Vec::with_capacity(len);
    let mut buf2 = Vec::with_capacity(len);
    unsafe { and2(datas[0], datas[1], &mut buf1) };
    for i in 2..datas.len() {
        unsafe { and2(datas[i], buf1.as_slice(), &mut buf2) };
        (buf1, buf2) = (buf2, buf1);
    }
    buf1
}

fn simple_and(datas: &[&Vec<u32>]) -> Vec<u32> {
    let mut indexes = vec![0; datas.len()];
    let mut result = Vec::new();
    if !unsafe { check_range(datas, indexes.as_slice()) } {
        return result;
    }
    let mut max = datas[0][0];
    loop {
        let mut flag = true;
        'a: for i in 0..datas.len() {
            unsafe {
                let d = *datas.get_unchecked(i);
                let index = indexes.get_unchecked_mut(i);
                while *d.get_unchecked(*index) < max {
                    *index += 1;
                    if *index >= d.len() {
                        *index -= 1;
                        if *d.get_unchecked(*index) == max {
                            continue 'a;
                        } else {
                            return result;
                        }
                    }
                }
                if *d.get_unchecked(*index) > max {
                    max = *d.get_unchecked(*index);
                    flag = false;
                }
            }
        }
        if flag {
            result.push(max);
            for (j, i) in indexes.iter_mut().enumerate() {
                *i += 1;
                if let Some(v) = unsafe { datas.get_unchecked(j).get(*i) } {
                    if max < *v {
                        max = *v;
                    }
                } else {
                    return result;
                }
            }
        } else if !unsafe { check_range(datas, indexes.as_slice()) } {
            break;
        }
    }
    result
}

pub fn and_beta2(datas: &[Data]) -> Data {
    if datas.is_empty() {
        return Data::None;
    }
    let mut positive = Vec::with_capacity(datas.len());
    let mut negative = Vec::with_capacity(datas.len());
    for data in datas {
        match data {
            Data::Data(d) => positive.push(d),
            Data::Not(d) => negative.push(d),
            Data::None => return Data::None,
            Data::ALL => {
                if !datas.iter().any(|d| d != &Data::ALL) {
                    return Data::ALL;
                }
            }
        }
    }
    if positive.is_empty() {
        return Data::Not(simple_and(&negative));
    } else if negative.is_empty() {
        return Data::Data(simple_and(&positive));
    }

    Data::ALL // TODO
}

fn word(word: &str) -> Option<Vec<u32>> {
    None
}

fn and(data: &[Vec<u32>]) -> Vec<u32> {
    vec![]
}

fn or(data: &[Vec<u32>]) -> Vec<u32> {
    vec![]
}

fn calc(node: &Node, values: &[String]) -> Option<Vec<u32>> {
    match node {
        Node::Value(n) => word(values[*n].as_str()),
        Node::Tag(category, name) => tag(values[*category].as_str(), values[*name].as_str()),
        Node::And(nodes) => {
            let mut data = Vec::new();
            for node in nodes {
                data.push(calc(node, values)?);
            }
            Some(and(&data))
        }
        Node::Or(nodes) => {
            let mut data = Vec::new();
            for node in nodes {
                if let Some(d) = calc(node, values) {
                    data.push(d);
                }
            }
            Some(or(&data))
        }
        Node::None => None,
        Node::ALL => None,
        Node::Not(_) => None,
    }
}
