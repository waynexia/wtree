mod envir;
mod print;
mod utils;
mod wtree;
fn main() -> std::io::Result<()> {
    wtree::print_tree()?;

    Ok(())
}
