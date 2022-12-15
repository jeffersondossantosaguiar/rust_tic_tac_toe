fn main() {
    let mut moves = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]];

    let title = "HASH GAME";
    let bar = "|";

    println!("{: >2}{:^13}", "", title);
    println!("{: >5}{: >4}{: >4}", "1", "2", "3");
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "A {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[0][0], bar, moves[0][1], bar, moves[0][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "B {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[1][0], bar, moves[1][1], bar, moves[1][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "C {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[2][0], bar, moves[2][1], bar, moves[2][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
}
