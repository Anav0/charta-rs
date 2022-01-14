use std::{collections::VecDeque, fs::File, io::Read};

#[derive(Debug, Clone)]
struct Node<'a> {
    pub weight: usize,
    pub text: Option<&'a str>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

fn print_tree(node: &Node, nodes: &Vec<Node>, level: usize) {
    let mut space = String::from("");

    for _ in 0..level {
        space += "  ";
    }

    println!("|{}{:?}", space, node.text);

    if node.left.is_some() {
        let left = &nodes[node.left.unwrap()];
        print_tree(left, &nodes, level + 1);
    }
    if node.right.is_some() {
        let right = &nodes[node.right.unwrap()];
        print_tree(right, &nodes, level + 1);
    }
}

fn build_tree<'a>(text: &'a String) -> Vec<Node<'a>> {
    let mut nodes = vec![];
    let root: Node<'a> = Node {
        text: Some(text),
        right: None,
        left: None,
        weight: text.len(),
    };
    nodes.push(root);

    let mut queque: VecDeque<usize> = VecDeque::new();
    queque.push_back(0);

    loop {
        let node_index_queque = queque.pop_front();
        let index = node_index_queque.unwrap();
        let chars = nodes[index].text.unwrap();

        let split = chars.split_at(chars.len() / 2);

        if split.0 == "" {
            break;
        }

        let length = nodes.len();
        nodes[index].left = Some(length);
        nodes[index].right = Some(length + 1);

        let left_node = Node {
            left: None,
            right: None,
            text: Some(split.0),
            weight: split.0.len(),
        };

        if left_node.text.is_some() {
            queque.push_back(nodes.len());
            nodes.push(left_node);
        }

        let right_node = Node {
            left: None,
            right: None,
            text: Some(split.1),
            weight: split.1.len(),
        };

        if right_node.text.is_some() {
            queque.push_back(nodes.len());
            nodes.push(right_node);
        }
    }
    for node in &nodes {
        println!("{:?}", node);
    }

    print_tree(&nodes[0], &nodes, 0);

    nodes
}

fn main() {
    let mut file = File::open("tekst3.txt").unwrap();

    let mut file_content = String::with_capacity(1024);

    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    let tree = build_tree(&file_content);
}
#[cfg(test)]
mod tests {
    use crate::{build_tree, Node};

    #[test]

    fn build_tree_works() {
        let text = String::from("Igor");
        let tree = build_tree(&text);
        assert_eq!(tree[0].text.unwrap(), "Igor");
    }
}
