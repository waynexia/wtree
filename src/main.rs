mod envir;
mod wtree;
fn main() -> std::io::Result<()> {
    wtree::print_tree()?;

    Ok(())
}
