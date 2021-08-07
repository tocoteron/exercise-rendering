use std::{env, fs::File, io::Read};

use exercise_rendering::{
    css,
    dom::{Node, NodeType},
    html,
    layout::to_layout_box,
    render::to_element_container,
    style::to_styled_node,
};

/*
const HTML: &str = r#"<body>
    <p>hello</p>
    <p class="inline">world</p>
    <p class="inline">:)</p>
    <div class="none"><p>this should not be shown</p></div>
    <style>
        .none { 
            display: none;
        }
        .inline {
            display: inline;
        }
    </style>
</body>"#;
*/

const DEFAULT_STYLESHEET: &str = r#"
script, style {
    display: none;
}
p, div {
    display: block;
}
"#;

pub fn collect_tag_inners(node: &Box<Node>, tag_name: &str) -> Vec<String> {
    if let NodeType::Element(ref element) = node.node_type {
        if element.tag_name.as_str() == tag_name {
            return vec![node.inner_text()];
        }
    }

    node.children
        .iter()
        .map(|child| collect_tag_inners(child, tag_name))
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect()
}

pub fn read_html_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();

    f
        .read_to_string(&mut contents)
        .expect("Failed to read the file");

    return contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage  : {} <html-file-name>", args[0]);
        println!("Example: {} index.html", args[0]);
        return
    }

    let html = read_html_file(&args[1]);

    let mut siv = cursive::default();

    let node = html::parse(&html);
    let stylesheet = css::parse(&format!(
        "{}\n{}",
        DEFAULT_STYLESHEET,
        collect_tag_inners(&node, "style".into()).join("\n")
    ));

    let container = to_styled_node(&node, &stylesheet)
        .and_then(|styled_node| Some(to_layout_box(styled_node)))
        .and_then(|layout_box| Some(to_element_container(layout_box)));
    if let Some(c) = container {
        siv.add_fullscreen_layer(c);
    }

    siv.run();
}
