use query::{Node, Nodes, Tokens};
use simple16::{dump, load};

fn main() {
    let query = r#"category:tag (category1 & category2):(tag1 | tag2 & tag3) word | -word2"#;
    let tokens = Tokens::new(query);
    let nodes = tokens.parse().unwrap();
    println!("{}", nodes);

    let v = (0..14).map(|x| x * 3).collect::<Vec<u32>>();
    let mut buf = Vec::new();
    dump(&v, &mut buf);
    println!("{:?}", buf);

    let mut v2 = Vec::new();
    load(&mut buf.as_slice(), &mut v2).unwrap();
    println!("{:?}", v2);

    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = vec![1, 3, 5, 7, 9, 11, 13, 15, 17];
    let v3 = vec![1, 4, 7, 10, 13, 16, 19, 22, 25];
    let res = and_beta(&[&v1, &v2, &v3]);
    println!("{:?}", res);
}

enum Data<'a> {
    Data(&'a [u32]),
    Not(&'a [u32]),
    None,
    ALL,
}

fn tag(category: &str, tag: &str) -> Option<Vec<u32>> {
    None
}

pub fn and_beta(data: &[&[u32]]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut indexes = data.iter().map(|_| 0).collect::<Vec<usize>>();
    let lens = data.iter().map(|d| d.len()).collect::<Vec<usize>>();

    while indexes.iter().zip(lens.iter()).all(|(i, l)| i < l) {
        let max = indexes
            .iter()
            .zip(data.iter())
            .map(|(i, d)| d[*i])
            .max()
            .unwrap();
        let mut flag = true;
        'a: for (i, d) in indexes.iter_mut().zip(data.iter()) {
            while d[*i] < max {
                flag = false;
                *i += 1;
                if *i >= d.len() {
                    if d[*i - 1] == max {
                        continue 'a;
                    } else {
                        return result;
                    }
                }
            }
        }
        if flag {
            result.push(max);
            for i in indexes.iter_mut() {
                *i += 1;
            }
        }
    }
    result
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
