mod mini_game;
mod selection_hash;


fn main(){
    let stamp_vec =  mini_game::generate_game();
    let result: (i32, i32) = mini_game::get_score(&stamp_vec, 150000);
    println!("{:?}", result);
    selection_hash::selection_hash(3, 6);
}