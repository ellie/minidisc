pub fn run() {
    let zsh = "
function md {
    cd $(minidisc cd $1)
}";

    println!("{}", zsh);
}
