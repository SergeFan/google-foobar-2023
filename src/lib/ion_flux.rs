#[derive(Debug)]
struct Node {
    root_label: isize,
    label: usize,
    height: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// solution(h, q)
pub fn find_ion_flux_labels(height: usize, labels: Vec<usize>) -> Vec<isize> {
    let mut root_node = Node {
        root_label: -1,
        label: 0,
        height: 1,
        left: None,
        right: None,
    };

    create_node(&mut root_node, height);
    set_label(&mut root_node);

    let mut relative_root_node_labels = Vec::new();

    for label in labels.iter() {
        match find_label(&root_node, *label) {
            None => {
                relative_root_node_labels.push(-1);
            }
            Some(root_node_label) => {
                relative_root_node_labels.push(root_node_label);
            }
        }
    }

    relative_root_node_labels
}

fn create_node(root_node: &mut Node, height: usize) {
    if root_node.height < height {
        let mut left_node = Node {
            root_label: -1,
            label: 0,
            height: root_node.height + 1,
            left: None,
            right: None,
        };

        create_node(&mut left_node, height);
        root_node.left = Some(Box::new(left_node));

        let mut right_node = Node {
            root_label: -1,
            label: 0,
            height: root_node.height + 1,
            left: None,
            right: None,
        };

        create_node(&mut right_node, height);
        root_node.right = Some(Box::new(right_node));
    }
}

fn set_label(root_node: &mut Node) -> usize {
    // post-order traversal
    //   3
    // 1   2
    match root_node.left.as_deref_mut() {
        None => {}
        Some(left_node) => {
            left_node.label = root_node.label;
            root_node.label = set_label(left_node);
        }
    }

    match root_node.right.as_deref_mut() {
        None => {}
        Some(right_node) => {
            right_node.label = root_node.label;
            root_node.label = set_label(right_node);
        }
    }

    root_node.label += 1;

    if root_node.left.as_deref_mut().is_some() {
        root_node.left.as_deref_mut().unwrap().root_label = root_node.label as isize;
    }

    if root_node.right.as_deref_mut().is_some() {
        root_node.right.as_deref_mut().unwrap().root_label = root_node.label as isize;
    }

    root_node.label
}

fn find_label(root_node: &Node, label: usize) -> Option<isize> {
    if root_node.label == label {
        return Some(root_node.root_label);
    }

    match root_node.left.as_deref() {
        None => {}
        Some(left_node) => match find_label(left_node, label) {
            None => {}
            Some(root_label) => return Some(root_label),
        },
    }

    match root_node.right.as_deref() {
        None => {}
        Some(right_node) => match find_label(right_node, label) {
            None => {}
            Some(root_label) => return Some(root_label),
        },
    }

    None
}

#[cfg(test)]
mod test_ion_flux {
    use super::*;

    #[test]
    fn test_creat_node() {
        let mut root_node = Node {
            root_label: -1,
            label: 0,
            height: 1,
            left: None,
            right: None,
        };

        create_node(&mut root_node, 3);

        assert!(root_node.left.as_deref().is_some());
        assert!(root_node.left.as_deref().unwrap().left.as_deref().is_some());
        assert!(root_node.right.as_deref().is_some());
        assert!(root_node
            .right
            .as_deref()
            .unwrap()
            .right
            .as_deref()
            .is_some());
    }

    #[test]
    fn test_set_label() {
        let mut root_node_1 = Node {
            root_label: -1,
            label: 0,
            height: 1,
            left: None,
            right: None,
        };

        create_node(&mut root_node_1, 3);
        set_label(&mut root_node_1);

        assert_eq!(root_node_1.label, 7);

        let mut root_node_2 = Node {
            root_label: -1,
            label: 0,
            height: 1,
            left: None,
            right: None,
        };

        create_node(&mut root_node_2, 4);
        set_label(&mut root_node_2);

        assert_eq!(root_node_2.label, 15);
    }

    #[test]
    fn test_find_ion_flux_labels() {
        assert_eq!(find_ion_flux_labels(3, vec![7, 3, 5, 1]), vec![-1, 7, 6, 3]);
        assert_eq!(find_ion_flux_labels(5, vec![19, 14, 28]), vec![21, 15, 29]);
    }
}
