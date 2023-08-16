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
}

fn tag(category: &str, tag: &str) -> Option<Vec<u32>> {
    None
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
