mod dom;
mod html;

fn main() {
    let my_dom = html::Parser::parse("<div>aaaa</div><p>aa</p>".to_string());

    my_dom.print(0);
}
