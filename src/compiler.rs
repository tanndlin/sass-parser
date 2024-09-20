use std::vec;

use crate::types::{Block, Style};

pub fn compile(blocks: Vec<Block>) -> String {
    let mut css = String::new();
    for block in blocks {
        css.push_str(compile_top_level_block(&block).as_str());
    }

    css
}

fn compile_top_level_block(block: &Block) -> String {
    let mut css: Vec<String> = vec![];
    for sub_block in &block.sub_blocks {
        css.extend(compile_sub_block(sub_block, block.selector.as_str()));
    }

    css.extend(compile_block(&block.selector, &block.styles));
    css.join("")
}

fn compile_sub_block(block: &Block, parent_selector: &str) -> Vec<String> {
    let compiled_selector = if block.selector.contains('&') {
        block.selector.replace('&', parent_selector)
    } else {
        format!("{} {}", parent_selector, block.selector)
    };

    let mut css = compile_block(&compiled_selector, &block.styles);

    let sub_blocks = &block.sub_blocks;
    for sub_block in sub_blocks {
        css.extend(compile_sub_block(sub_block, &compiled_selector));
    }

    css
}

fn compile_block(selector: &String, styles: &Vec<Style>) -> Vec<String> {
    let mut css = vec![selector.to_string()];
    css.push(" {\n".to_string());

    for style in styles {
        css.push(format!("    {}: {};\n", style.name, style.value));
    }

    // Close the block
    css.push("}\n\n".to_string());

    css
}
