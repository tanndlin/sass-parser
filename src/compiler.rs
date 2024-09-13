use crate::types::Class;

pub struct Compiler {
    classes: Vec<Class>,
}

impl Compiler {
    pub fn new(classes: Vec<Class>) -> Compiler {
        Compiler { classes }
    }

    pub fn compile(&mut self, filename: &str) {
        println!("Compiling...");

        let mut css = String::new();
        for class in &self.classes {
            css.push_str(&self.compile_top_level_class(class));
        }

        std::fs::write(filename, css).expect("Unable to write file");
    }

    fn compile_top_level_class(&self, class: &Class) -> String {
        let mut css = self.compile_class(class);
        for sub_class in &class.sub_classes {
            css.push_str(&self.compile_sub_class(sub_class, &class.selector));
        }

        css
    }

    fn compile_sub_class(&self, class: &Class, parent_selector: &str) -> String {
        let compiled_selector = if class.selector.contains('&') {
            class.selector.clone().replace('&', parent_selector)
        } else {
            format!("{} {}", parent_selector, class.selector)
        };

        let mut css = self.compile_class(&Class {
            selector: compiled_selector.clone(),
            styles: class.styles.clone(),
            sub_classes: class.sub_classes.clone(),
        });

        for sub_class in &class.sub_classes {
            css.push_str(&self.compile_sub_class(sub_class, &compiled_selector));
        }

        css.clone()
    }

    fn compile_class(&self, class: &Class) -> String {
        let mut css = String::new();

        css.push_str(&class.selector.clone().to_string());
        css.push_str(" {\n");

        for style in &class.styles {
            css.push_str(&format!("    {}: {};\n", style.name, style.value));
        }

        // Close the class
        css.push_str("}\n\n");

        css
    }
}
