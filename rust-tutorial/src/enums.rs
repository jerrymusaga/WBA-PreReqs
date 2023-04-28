
enum Movement {
    //variants
    UP,
    DOWN,
    RIGHT, 
    LEFT
}

fn move_avatar(m: Movement){
    //perform action depending on movement
    match m {
        Movement::UP => println!("Avatar is moving up"),
        Movement::DOWN => println!("Avatar is moving down"),
        Movement::RIGHT => println!("Avatar is moving right"),
        Movement::LEFT => println!("Avatar is moving left"),
    }
}

pub fn run(){
    let avatar1 = Movement::LEFT;
    let avatar2 = Movement::RIGHT;
    let avatar3 = Movement::UP;
    let avatar4 = Movement::DOWN;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}