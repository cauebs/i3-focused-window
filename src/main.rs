use exitfailure::ExitFailure;
use i3ipc::{reply::Node, I3Connection};
use structopt::StructOpt;

fn find_focused(root: &Node) -> Option<&Node> {
    if root.focused {
        return Some(root);
    }

    Iterator::chain(root.nodes.iter(), root.floating_nodes.iter())
        .filter_map(|n| find_focused(n))
        .next()
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "%wx%h %x%y")]
    /// Output format.
    /// {n}You can use '%x' and '%y' for offset,
    /// {n}'%w' and '%h' for dimensions (width and height).
    /// {n}Example: "%x,%y %wx%h" gives "-1+2 3x4".
    format: String,
}

fn format_with_sign(num: i32) -> String {
    if num < 0 {
        num.to_string()
    } else {
        format!("+{}", num.to_string())
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let mut ipc = I3Connection::connect()?;
    let root = ipc.get_tree()?;

    let focused = find_focused(&root).expect("No focused windows found.");
    let (x, y, w, h) = focused.rect;

    let output = args
        .format
        .replace("%w", &w.to_string())
        .replace("%h", &h.to_string())
        .replace("%x", &format_with_sign(x))
        .replace("%y", &format_with_sign(y));

    println!("{}", output);
    Ok(())
}
