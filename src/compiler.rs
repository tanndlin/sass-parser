use crate::types::*;

pub struct Compiler {
    classes: Vec<Class>,
}

impl Compiler {
    pub fn new(classes: Vec<Class>) -> Compiler {
        Compiler { classes }
    }

    pub fn compile(self, file_name: &str) {
        compile(self.classes, file_name);
    }
}

pub fn compile(classes: Vec<Class>, filename: &str) {
    println!("Compiling...");

    let mut css = String::new();
    for class in classes {
        css.push_str(compile_top_level_class(&class).as_str());
    }

    std::fs::write(filename, css).expect("Unable to write file");
}

fn compile_top_level_class(class: &Class) -> String {
    let (shallow, sub_classes) = class.shallow();
    let mut css = "".to_string();
    for sub_class in sub_classes {
        css.push_str(compile_sub_class(sub_class, shallow.selector).as_str());
    }

    compile_class(&shallow) + &css
}

fn compile_sub_class(class: &Class, parent_selector: &str) -> String {
    let compiled_selector = if class.selector.contains('&') {
        class.selector.replace('&', parent_selector)
    } else {
        format!("{} {}", parent_selector, class.selector)
    };

    let (shallow, sub_classes) = (
        &ShallowClass {
            selector: &compiled_selector,
            styles: &class.styles,
        },
        &class.sub_classes,
    );

    let mut css = compile_class(shallow);
    for sub_class in sub_classes {
        css.push_str(&compile_sub_class(sub_class, shallow.selector));
    }

    css
}

fn compile_class(class: &ShallowClass) -> String {
    let mut css = class.selector.to_string();
    css.push_str(" {\n");

    for style in class.styles {
        css.push_str(&format!("    {}: {};\n", style.name, style.value));
    }

    // Close the class
    css.push_str("}\n\n");

    css
}
