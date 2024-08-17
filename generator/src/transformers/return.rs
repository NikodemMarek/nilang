use nilang_parser::nodes::Node;

use crate::transformers::transform;

pub fn transform_return(a: &Node) -> (Vec<String>, Vec<String>) {
    if let Node::Return(inner) = a {
        let this = transform(inner);
        (this.0, {
            let mut r = this.1;
            r.append(&mut Vec::from([
                String::from("pop %rax"),
                String::from("movl %eax, %ebx"),
            ]));
            r
        })
    } else {
        panic!("Unexpected node: {:?}", a)
    }
}
