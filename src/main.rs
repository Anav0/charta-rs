use std::{collections::VecDeque, fs::File, io::Read};

#[derive(Debug, Clone)]
struct Node<'a> {
    pub weight: usize,
    pub text: Option<&'a str>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,
}

fn print_tree(node: &Node, nodes: &Vec<Node>, level: usize) {
    let mut space = String::from("");

    for _ in 0..level {
        space += "  ";
    }

    println!("|{}{} {}", space, node.weight, node.text.unwrap());

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
        parent: None,
        weight: 0,
    };
    nodes.push(root);

    let mut queque: VecDeque<usize> = VecDeque::new();
    queque.push_back(0);

    loop {
        let node_index_queque = queque.pop_front();
        if node_index_queque.is_none() {
            break;
        }

        let index = node_index_queque.unwrap();
        let chars = nodes[index].text.unwrap();

        let split_point = chars.len() / 2;

        if split_point <= 1 {
            break;
        }

        let split = chars.split_at(split_point - 1);

        let length = nodes.len();
        nodes[index].left = Some(length);
        nodes[index].right = Some(length + 1);
        nodes[index].weight = split.0.len();

        let left_node = Node {
            left: None,
            right: None,
            parent: Some(index),
            text: Some(split.0),
            weight: split.0.len(),
        };

        queque.push_back(nodes.len());
        nodes.push(left_node);

        let right_node = Node {
            left: None,
            right: None,
            parent: Some(index),
            text: Some(split.1),
            weight: split.1.len(),
        };

        queque.push_back(nodes.len());
        nodes.push(right_node);
    }

    nodes
}

fn index<'a>(node: &'a Node, i: usize, tree: &'a Vec<Node>) -> char {
    if node.left.is_none() {
        return node.text.unwrap().chars().collect::<Vec<char>>()[i]; //TODO: this can be better
    }

    let left = &tree[node.left.unwrap()];
    if node.weight <= i && node.right.is_some() {
        let right = &tree[node.right.unwrap()];
        return index(right, i - node.weight, tree);
    }

    return index(&left, i, tree);
}

fn main() {
    let mut file = File::open("sheks.txt").unwrap();

    let mut file_content = String::with_capacity(1024);

    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    let tree = build_tree(&file_content);
    println!("Nodes: {}", tree.len());
    println!("Char at {} = {}", 5, index(&tree[0], 5, &tree));
}
#[cfg(test)]
mod tests {
    use crate::{build_tree, index};

    #[test]
    fn index_works_0() {
        let text = String::from("Ciapcio");
        let tree = build_tree(&text);

        let char = index(&tree[0], 0, &tree);

        assert_eq!(char, 'C');
    }
    #[test]
    fn index_works_1() {
        let text = String::from("Ciapcio");
        let tree = build_tree(&text);

        let char = index(&tree[0], 1, &tree);

        assert_eq!(char, 'i');
    }
    #[test]
    fn index_works_5() {
        let text = String::from("Ciapcio");
        let tree = build_tree(&text);

        let char = index(&tree[0], 5, &tree);

        assert_eq!(char, 'i');
    }
}
