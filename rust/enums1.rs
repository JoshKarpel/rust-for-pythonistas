enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn to_arrow(dir: &Direction) -> String {
    let s = match dir {
        Direction::Up => "↑",
        Direction::Down => "↓",
        Direction::Left => "←",
        Direction::Right => "→",
    };

    s.into()
}


fn main() {
    println!("{}", to_arrow(&Direction::Up));
    println!("{}", to_arrow(&Direction::Down));
    println!("{}", to_arrow(&Direction::Left));
    println!("{}", to_arrow(&Direction::Right));
}
