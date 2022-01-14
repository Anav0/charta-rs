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

        let split = chars.split_at(chars.len() / 2);

        if split.0 == "" {
            continue;
        }

        let length = nodes.len();
        nodes[index].left = Some(length);
        nodes[index].right = Some(length + 1);
        nodes[index].weight = split.0.len();

        let left_node = Node {
            left: None,
            right: None,
            parent: Some(index),
            text: Some(split.0),
            weight: 0,
        };

        queque.push_back(nodes.len());
        nodes.push(left_node);

        let right_node = Node {
            left: None,
            right: None,
            parent: Some(index),
            text: Some(split.1),
            weight: 0,
        };

        queque.push_back(nodes.len());
        nodes.push(right_node);
    }

    nodes
}

fn main() {
    let mut file = File::open("tekst3.txt").unwrap();

    let mut file_content = String::with_capacity(1024);

    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    let tree = build_tree(&file_content);
    print_tree(&tree[0], &tree, 0);
}
#[cfg(test)]
mod tests {
    use crate::build_tree;

    #[test]

    fn build_tree_works() {
        let text = String::from("Igor");
        let tree = build_tree(&text);
        assert_eq!(tree[0].text.unwrap(), "Igor");
        assert_eq!(tree[1].text.unwrap(), "Ig");
        assert_eq!(tree[2].text.unwrap(), "or");
        assert_eq!(tree[3].text.unwrap(), "I");
        assert_eq!(tree[4].text.unwrap(), "g");
        assert_eq!(tree[5].text.unwrap(), "o");
        assert_eq!(tree[6].text.unwrap(), "r");
    }
    #[test]
    fn build_tree_works_uneven() {
        let text = String::from("Ciapcio");
        let tree = build_tree(&text);
        assert_eq!(tree[0].text.unwrap(), "Ciapcio");
        assert_eq!(tree[1].text.unwrap(), "Cia");
        assert_eq!(tree[2].text.unwrap(), "pcio");
        assert_eq!(tree[3].text.unwrap(), "C");
        assert_eq!(tree[4].text.unwrap(), "ia");
        assert_eq!(tree[5].text.unwrap(), "pc");
        assert_eq!(tree[6].text.unwrap(), "io");
        assert_eq!(tree[7].text.unwrap(), "i");
        assert_eq!(tree[8].text.unwrap(), "a");
        assert_eq!(tree[9].text.unwrap(), "p");
        assert_eq!(tree[10].text.unwrap(), "c");
        assert_eq!(tree[11].text.unwrap(), "i");
        assert_eq!(tree[12].text.unwrap(), "o");
    }

    #[test]
    fn weight_calculation_works() {
        let text = String::from("Igor");
        let tree = build_tree(&text);

        assert_eq!(tree[0].weight, 2); //Igor
        assert_eq!(tree[1].weight, 1); //Ig
        assert_eq!(tree[2].weight, 1); //or
        assert_eq!(tree[3].weight, 0); //I
        assert_eq!(tree[4].weight, 0); //g
        assert_eq!(tree[5].weight, 0); //o
        assert_eq!(tree[6].weight, 0); //r
    }
}
