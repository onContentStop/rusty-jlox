use exit::Exit;
use std::{env, fs::File, io::BufWriter, path::PathBuf};
use subprocess::{Popen, PopenConfig, Redirection};

fn main() -> Exit<i8> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: generate_ast <output directory>");
        return Exit::Err(64);
    }

    let output_dir = &args[0];
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   : Expr, Token, Expr",
            "Grouping : Expr",
            "Literal  : Value",
            "Unary    : Token, Expr",
        ],
        vec!["crate::token::Token", "crate::literal::Value"],
    );

    define_ast(
        output_dir,
        "Stmt",
        vec!["Expression : Expr", "Print      : Expr"],
        vec!["crate::ast::expr::Expr"],
    );

    Exit::Ok
}

#[derive(Debug)]
struct TypeList {
    base: String,
    types: Vec<Type>,
}

#[derive(Debug)]
struct Type {
    name: String,
    fields: Vec<String>,
    a_list: String,
    typed_a_list: String,
}

fn gen_uses(uses: Vec<&'static str>) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for u in uses {
        write!(s, "use {};", u).unwrap();
    }
    s
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    types: Vec<&'static str>,
    uses: Vec<&'static str>,
) {
    let path = PathBuf::from(output_dir).join(base_name.to_lowercase() + ".rs");
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut contents = gen_uses(uses);

    let type_list = TypeList {
        base: base_name.to_owned(),
        types: types
            .iter()
            .map(|ty| {
                let mut split = ty.split(':');
                let name = split.next().unwrap().trim();
                let fields: Vec<String> = split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|f| f.trim().to_owned())
                    .collect();
                Type {
                    name: name.to_owned(),
                    a_list: (0..fields.len())
                        .map(|a| format!("a{},", a))
                        .collect::<Vec<_>>()
                        .join(""),
                    typed_a_list: fields
                        .iter()
                        .enumerate()
                        .map(|(i, t)| format!("a{}: &{},", i, t))
                        .collect(),
                    fields,
                }
            })
            .collect(),
    };

    contents.push_str(&format!("#[derive(Debug)]pub enum {} {{", type_list.base));
    for ty in &type_list.types {
        contents.push_str(&format!("{}(", ty.name));
        for field in &ty.fields {
            if field == &type_list.base {
                use std::fmt::Write;
                write!(contents, "Box<{}>,", field).unwrap();
            } else {
                contents.push_str(&format!("{},", field));
            }
        }
        contents.push_str("),");
    }
    contents.push('}');

    contents.push_str(&format!("impl {} {{", type_list.base));
    contents.push_str("pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {");
    contents.push_str("match self {");
    for ty in &type_list.types {
        contents.push_str(&format!("{}::{}(", type_list.base, ty.name));
        contents.push_str(&ty.a_list);
        contents.push_str(&format!(
            ") => visitor.visit_{}_{}({}),",
            ty.name.to_lowercase(),
            type_list.base.to_lowercase(),
            ty.a_list,
        ));
    }
    contents.push('}'); // match
    contents.push('}'); // accept
    contents.push('}'); // impl Expr

    contents.push_str("pub trait Visitor<R> {");
    for ty in type_list.types {
        contents.push_str(&format!(
            "fn visit_{}_{}(&mut self, {}) -> R;",
            ty.name.to_lowercase(),
            type_list.base.to_lowercase(),
            ty.typed_a_list,
        ));
    }
    contents.push('}'); // trait Visitor<R>
    contents = run_rustfmt_on(contents);
    use std::io::Write;
    writer.write_all(contents.as_bytes()).unwrap();
}

fn run_rustfmt_on(s: String) -> String {
    let (out, _err) = Popen::create(
        &["rustfmt"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            ..Default::default()
        },
    )
    .unwrap()
    .communicate(Some(&s))
    .unwrap();
    out.unwrap()
}
