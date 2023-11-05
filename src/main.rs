mod css;
mod dom;
mod html;
mod style;

fn main() {
    let my_dom = html::Parser::parse(
        "<div id=\"id\">aaaa<img src=\"image.png\" alt=\"some image\" /></div><p style=\"color: #222; text-color: #111;\">aa</p>"
            .to_string(),
    );

    let my_style = css::Parser::parse(
        "div { text-align: center; color: #000; } p, .class, #id { color: #fff; }",
    );

    let styled = style::style_tree(&my_dom, &my_style);
    styled.print();
}
