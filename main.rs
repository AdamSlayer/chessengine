use speedy2d::color::Color;
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper, MouseButton};
use speedy2d::{Graphics2D, Window};
use std::io::{stdin, Read, self};

const ENGINE_COLOR_WHITE: bool = false;
const DEPTH:i32 = 4;

// TODO_EVENTUALLY: draws by insufficient material/ repetition/ 50 moves
 
// TODO: engine: only search moves that have a decent depth-1 evaluation, more accurate 0depth evaluation


fn main() {
    let window = Window::new_centered("Chess", (544, 544)).unwrap();
    window.run_loop(MyWindowHandler {
        castling_possible: ((true,true), (true, true)), // white(left,right), black(left,right)
        en_passant_position: 255, // 255 if not possible, the position where enemy would have to move the pawn otherwise
        white_to_play_next: true,
        all_possible_moves: Vec::new(),
        dragged_piece: 255,
        mouse_pos: (0.,0.),
        loaded: false,
        images: [None,None,None,None,None,None,None,None,None,None,None,None,None],
        pieces: [Piece::Empty;64]
    });
}


#[derive(Clone,Copy,PartialEq)]
enum Piece {
    Empty,
    PawnD,   PawnL,
    RookD,   RookL,
    KnightD, KnightL,
    BishopD, BishopL,
    QueenD,  QueenL,
    KingD,   KingL
}
impl Piece {
    fn is_empty(self: Self) -> bool {
        match self {
            Piece::Empty => true,
            _ => false
        }
    }
    fn is_dark(self: Self) -> bool {
        match self {
            Piece::PawnD => true,
            Piece::RookD => true,
            Piece::KnightD => true,
            Piece::BishopD => true,
            Piece::QueenD => true,
            Piece::KingD => true,
            _ => false
        }
    }
    fn is_pawn(self: Self) -> bool {
        match self {
            Piece::PawnD => true,
            Piece::PawnL => true,
            _ => false
        }
    }
    fn is_rook(self: Self) -> bool {
        match self {
            Piece::RookD => true,
            Piece::RookL => true,
            _ => false
        }
    }
    fn is_knight(self: Self) -> bool {
        match self {
            Piece::KnightD => true,
            Piece::KnightL => true,
            _ => false
        }
    }
    fn is_bishop(self: Self) -> bool {
        match self {
            Piece::BishopD => true,
            Piece::BishopL => true,
            _ => false
        }
    }
    fn is_king(self: Self) -> bool {
        match self {
            Piece::KingD => true,
            Piece::KingL => true,
            _ => false
        }
    }
    fn is_queen(self: Self) -> bool {
        match self {
            Piece::QueenD => true,
            Piece::QueenL => true,
            _ => false
        }
    }
}

#[derive(PartialEq)]
enum GameEnd {
    NoEnd,
    Stealmate,
    BlackWin,
    WhiteWin
}


struct MyWindowHandler {
    castling_possible: ((bool,bool), (bool, bool)),
    en_passant_position: usize,
    white_to_play_next: bool,
    all_possible_moves: Vec<(usize, Vec<usize>)>,
    dragged_piece: usize,
    mouse_pos: (f32,f32),
    loaded: bool,
    images: [Option<ImageHandle>;13],
    pieces: [Piece;64]
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        // load for the first time, loads images and spawn pieces on the board
        if !self.loaded {
            self.loaded =  true;
            let paths = ["assets/board.png", "assets/pawn_light.png", "assets/pawn_dark.png", "assets/rook_light.png", "assets/rook_dark.png", "assets/knight_light.png", "assets/knight_dark.png", "assets/bishop_light.png", "assets/bishop_dark.png", "assets/queen_light.png", "assets/queen_dark.png", "assets/king_light.png", "assets/king_dark.png"];
            for i in 0..13 {
                let image = graphics
                    .create_image_from_file_path(
                        None,
                        ImageSmoothingMode::NearestNeighbor,
                        paths[i]
                    )
                    .unwrap();

                self.images[i] = Some(image);
            }

            for x in 0..8 {
                for y in 0..8 {
                    self.pieces[y*8+x] = {
                        if y == 1 {Piece::PawnD}
                        else if y == 6 {Piece::PawnL}
                        else if y == 0 {
                            match x {
                                0 => Piece::RookD,
                                1 => Piece::KnightD,
                                2 => Piece::BishopD,
                                3 => Piece::QueenD,
                                4 => Piece::KingD,
                                5 => Piece::BishopD,
                                6 => Piece::KnightD,
                                7 => Piece::RookD,
                                _ => Piece::Empty
                            }
                        }
                        else if y == 7 {
                            match x {
                                0 => Piece::RookL,
                                1 => Piece::KnightL,
                                2 => Piece::BishopL,
                                3 => Piece::QueenL,
                                4 => Piece::KingL,
                                5 => Piece::BishopL,
                                6 => Piece::KnightL,
                                7 => Piece::RookL,
                                _ => Piece::Empty
                            }
                        }
                        else {
                            Piece::Empty
                        }
                    }
                }
            }
        }


        // draw board and pieces on the screen
        graphics.clear_screen(Color::BLACK);
        graphics.draw_image((16.0, 16.0), self.images[0].as_ref().unwrap());

        let mut u_images: Vec<ImageHandle> = Vec::new();
        for i in 0..13 {
            u_images.push(self.images[i].clone().unwrap());
        }
        for x in 0..8 {
            for y in 0..8 {
                if self.pieces[y*8+x] != Piece::Empty {
                    graphics.draw_image( {
                        if self.dragged_piece != y*8+x {
                            (16.+x as f32*64.,16.+y as f32*64.)
                        } else {
                            (self.mouse_pos.0-30., self.mouse_pos.1-30.)
                        }
                    }, {
                        match self.pieces[y*8+x] {
                            Piece::Empty   => &u_images[0],
                            Piece::PawnL   => &u_images[1],
                            Piece::PawnD   => &u_images[2],
                            Piece::RookL   => &u_images[3],
                            Piece::RookD   => &u_images[4],
                            Piece::KnightL => &u_images[5],
                            Piece::KnightD => &u_images[6],
                            Piece::BishopL => &u_images[7],
                            Piece::BishopD => &u_images[8],
                            Piece::QueenL  => &u_images[9],
                            Piece::QueenD  => &u_images[10],
                            Piece::KingL   => &u_images[11],
                            Piece::KingD   => &u_images[12],
                        }
                    })
                }
            }
        }

        for p in &self.all_possible_moves {
            for m in &p.1 {
                let x = (m%8) as f32;
                let y = (m/8) as f32;
                if p.0 == self.dragged_piece {
                    graphics.draw_circle((x*64.+48.,y*64.+48.), 10., Color::from_rgba(1.0, 0.0, 0.0, 0.7));
                }
            }
        }
        
        if self.all_possible_moves.len() == 0 {

            self.all_possible_moves = gen_all_moves(self.pieces, self.en_passant_position, self.castling_possible, self.white_to_play_next, false);

            println!("legal moves for {}: {}",{
                if self.white_to_play_next {
                    "white"
                } else {
                    "black"
                }
            } ,{
                let mut x = 0;
                for p in &self.all_possible_moves {
                    x += p.1.len();
                }
                x
            });
            //println!("best move: {:?}", move_to_notation(best_move(self.pieces, self.en_passant_position, self.castling_possible, self.white_to_play_next, 2)));
            //println!("current position eval: {:?}", eval(self.pieces, self.en_passant_position, self.castling_possible, self.white_to_play_next, DEPTH));
            
            if self.all_possible_moves.len() == 0 {
                if is_check(self.white_to_play_next, self.pieces) {
                    println!("checkmate, {} won", {
                        if self.white_to_play_next {
                            "black"
                        } else {
                            "white"
                        }
                    });
                } else {
                    println!("stealmate");
                }
                
                println!("press enter to continue");
                stdin().read(&mut [0]).unwrap();

                self.castling_possible = ((true,true), (true, true));
                self.en_passant_position = 255;
                self.white_to_play_next = true;
                self.all_possible_moves = Vec::new();
                self.dragged_piece = 255;
                self.mouse_pos = (0.,0.);
                self.loaded = false;
                self.pieces = [Piece::Empty;64];
            }
            if  self.white_to_play_next == ENGINE_COLOR_WHITE {
                let m = best_move(self.pieces, self.en_passant_position, self.castling_possible, self.white_to_play_next, DEPTH);
                let move_made = make_a_move(self.en_passant_position, &mut self.pieces, m.0, m.1, self.castling_possible, true);
                (self.en_passant_position, self.castling_possible) = (move_made.0, move_made.2);
                if move_made.1 == GameEnd::BlackWin {
                    println!("black checkmated white");
                }
                if move_made.1 == GameEnd::WhiteWin {
                    println!("white checkmated black");
                }
                if move_made.1 == GameEnd::Stealmate {
                    println!("stealmate");
                }
                self.white_to_play_next = !self.white_to_play_next;
                self.all_possible_moves.clear();
            }
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.mouse_pos.0 = position.x;
        self.mouse_pos.1 = position.y
    }

    fn on_mouse_button_down( &mut self, _: &mut WindowHelper<()>, button: speedy2d::window::MouseButton ) {
        if button == MouseButton::Left {
            self.dragged_piece = ((self.mouse_pos.0 - 16.)/64.) as usize + ((self.mouse_pos.1 - 16.)/64.) as usize * 8;
        }
    }

    fn on_mouse_button_up( &mut self, _: &mut WindowHelper<()>, button: speedy2d::window::MouseButton ) {
        if button == MouseButton::Left {
            let tg_pos = ((self.mouse_pos.0 - 16.)/64.) as usize + ((self.mouse_pos.1 - 16.)/64.) as usize * 8;

            if  {
                let mut is_move_legal = false;
                if self.pieces[self.dragged_piece].is_dark() == !self.white_to_play_next {
                    'a: for p in &self.all_possible_moves {
                        if self.dragged_piece == p.0 {
                            for m in &p.1 {
                                if *m == tg_pos {
                                    is_move_legal = true;
                                    break 'a
                                }
                            }
                        }
                    }
                }
                is_move_legal
            } {
                let move_made = make_a_move(self.en_passant_position, &mut self.pieces, self.dragged_piece, tg_pos, self.castling_possible, false);
                (self.en_passant_position, self.castling_possible) = (move_made.0, move_made.2);
                if move_made.1 == GameEnd::BlackWin {
                    println!("black checkmated white");
                }
                if move_made.1 == GameEnd::WhiteWin {
                    println!("white checkmated black");
                }
                if move_made.1 == GameEnd::Stealmate {
                    println!("stealmate");
                }
                self.white_to_play_next = !self.white_to_play_next;
                self.all_possible_moves.clear();
            }

            self.dragged_piece = 255;
        }
    }
}



fn make_a_move(en_passant_position:usize, pieces: &mut [Piece; 64], move_start_pos: usize, mut move_tg_pos: usize, castling_possible:((bool,bool),(bool,bool)), called_by_engine: bool) -> (usize, GameEnd, ((bool,bool),(bool,bool))) {
    let mut out = (255, GameEnd::NoEnd, castling_possible);
    if en_passant_position == move_tg_pos && pieces[move_start_pos].is_pawn() {
        if pieces[move_start_pos].is_dark() {
            pieces[move_tg_pos%64%64 - 8] = Piece::Empty;
        } else {
            pieces[move_tg_pos%64 + 8] = Piece::Empty;
        }
    }
    if pieces[move_start_pos].is_pawn() && (move_start_pos.abs_diff(move_tg_pos%64) == 16) {
        out.0 = (move_start_pos + move_tg_pos%64)/2;
    }
    if pieces[move_start_pos].is_pawn() && (move_tg_pos%64/8 == 0 || move_tg_pos%64/8 == 7) {

        if !called_by_engine {
            println!("What do you want to promote into? (q/r/b/n)");

            let mut inp = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut inp).unwrap();
            inp = inp.trim().parse().unwrap();

            if inp == "q" {
                move_tg_pos += 0;
            } else if inp == "r" {
                move_tg_pos += 64;
            } else if inp == "b" {
                move_tg_pos += 128;
            } else if inp == "n" {
                move_tg_pos += 192;
            }
        }
        // promoting
        if pieces[move_start_pos].is_dark() {
            pieces[move_start_pos] = match move_tg_pos/64 {
                0 => Piece::QueenD,
                1 => Piece::RookD,
                2 => Piece::BishopD,
                3 => Piece::KnightD,
                _ => {panic!()}
            };
        } else {
            pieces[move_start_pos] = match move_tg_pos/64 {
                0 => Piece::QueenL,
                1 => Piece::RookL,
                2 => Piece::BishopL,
                3 => Piece::KnightL,
                _ => {panic!()}
            };
        }
        
    }
    if pieces[move_start_pos].is_king() {
        if pieces[move_start_pos].is_dark() {
            out.2.1 = (false, false);
        } else {
            out.2.0 = (false, false);
        }
        if move_tg_pos.abs_diff(move_start_pos) == 2 {
            if move_tg_pos == 8*0+2 {
                pieces[8*0+0] = Piece::Empty;
                pieces[8*0+3] = Piece::RookD;
            }
            else if move_tg_pos == 8*0+6 {
                pieces[8*0+7] = Piece::Empty;
                pieces[8*0+5] = Piece::RookD;
            }
            else if move_tg_pos == 8*7+2 {
                pieces[8*7+0] = Piece::Empty;
                pieces[8*7+3] = Piece::RookL;
            }
            else if move_tg_pos == 8*7+6 {
                pieces[8*7+7] = Piece::Empty;
                pieces[8*7+5] = Piece::RookL;
            } else {
                println!("something went wrong with castling");
            }
        }
    }
    if pieces[move_start_pos].is_rook() {
        if pieces[move_start_pos].is_dark() {
            if move_start_pos%8 == 0 {
                out.2.1.0 = false;
            }
            if move_start_pos%8 == 7 {
                out.2.1.1 = false;
            }
        } else {
            if move_start_pos%8 == 0 {
                out.2.0.0 = false;
            }
            if move_start_pos%8 == 7 {
                out.2.0.1 = false;
            }
        }
    }
    pieces[move_tg_pos%64] = pieces[move_start_pos];
    pieces[move_start_pos] = Piece::Empty;
    out
}

 
fn is_check(white_to_play_next: bool, pieces: [Piece; 64]) -> bool {
    // white_to_play == true => check if its a check for white king
    let mut kp: usize = 255; // king pos
    for p in 0..64 {
        if pieces[p].is_king() && (pieces[p].is_dark() != white_to_play_next) {
            kp = p;
            break
        }
    }

    let apm = gen_all_moves(pieces, 255, ((false,false),(false,false)), white_to_play_next, true);

    for piece in apm {
        for m in piece.1 {
            if m == kp {
                return true
            }
        }
    }

    false
}


fn gen_all_moves(pieces:[Piece;64], en_passant_position:usize, castling_possible:((bool,bool),(bool,bool)), white_to_play_next:bool, called_by_is_check:bool) -> Vec<(usize, Vec<usize>)> {
    let mut apm: Vec<(usize, Vec<usize>)> = Vec::new();
    for piece in 0..64 {
        if !pieces[piece].is_empty() {
            if pieces[piece].is_dark() != (white_to_play_next != called_by_is_check) {
                let mut cpm: Vec<usize> = Vec::new(); // current piece possible moves
                let sx:usize = piece%8;
                let sy:usize = piece/8;

                if pieces[piece].is_king() {
                    let mut pm: Vec<(usize,usize)> = vec![(sx+1,sy+1),(sx-1,sy-1),(sx-1,sy+1),(sx+1,sy-1),(sx,sy+1),(sx,sy-1),(sx+1,sy),(sx-1,sy)];
                    if piece == 0 {
                        pm = vec![(sx+1,sy+1),(sx,sy+1),(sx+1,sy)];
                    }
                    else if piece == 63 {
                        pm = vec![(sx-1,sy-1),(sx,sy-1),(sx-1,sy)];
                    }
                    else if piece == 7 {
                        pm = vec![(sx-1,sy+1),(sx,sy+1),(sx-1,sy)];
                    }
                    else if piece == 56 {
                        pm = vec![(sx+1,sy-1),(sx,sy-1),(sx+1,sy)];
                    } else {
                        if sx == 0 {
                            pm.remove(7);
                            pm.remove(2);
                            pm.remove(1);
                        }
                        if sx == 7 {
                            pm.remove(6);
                            pm.remove(3);
                            pm.remove(0);
                        }
                        if sy == 0 {
                            pm.remove(5);
                            pm.remove(3);
                            pm.remove(1);
                        }
                        if sy == 7 {
                            pm.remove(4);
                            pm.remove(2);
                            pm.remove(0);
                        }
                    }
                    for i in 0..pm.len() {
                        if pieces[pm[i].0+pm[i].1*8].is_empty() || (pieces[pm[i].0+pm[i].1*8].is_dark() != pieces[piece].is_dark()) {
                            cpm.push(pm[i].0+pm[i].1*8);
                        }
                    }
                    // castling
                    if !called_by_is_check {
                        if pieces[piece].is_dark() && castling_possible.1.0 && pieces[1+0*8].is_empty() && pieces[2+0*8].is_empty() && pieces[3+0*8].is_empty() {
                            let mut p = pieces.clone();
                            if !is_check(white_to_play_next, p) { // check if king isnt currently in check
                                p[3+0*8] = Piece::KingD;
                                p[4+0*8] = Piece::Empty;
                                if !is_check(white_to_play_next, p) {  // check if king desnt move through check
                                    cpm.push(0*8+2);
                                }
                            }
                        }
                        if pieces[piece].is_dark() && castling_possible.1.1 && pieces[5+0*8].is_empty() && pieces[6+0*8].is_empty() {
                            let mut p = pieces.clone();
                            if !is_check(white_to_play_next, p) { // check if king isnt currently in check
                                p[5+0*8] = Piece::KingD;
                                p[4+0*8] = Piece::Empty;
                                if !is_check(white_to_play_next, p) {  // check if king desnt move through check
                                    cpm.push(0*8+6);
                                }
                            }
                        }
                        if !pieces[piece].is_dark() && castling_possible.0.0 && pieces[1+7*8].is_empty() && pieces[2+7*8].is_empty() && pieces[3+7*8].is_empty() {
                            let mut p = pieces.clone();
                            if !is_check(white_to_play_next, p) { // check if king isnt currently in check
                                p[3+7*8] = Piece::KingL;
                                p[4+7*8] = Piece::Empty;
                                if !is_check(white_to_play_next, p) {  // check if king desnt move through check
                                    cpm.push(7*8+2);
                                }
                            }
                        }
                        if !pieces[piece].is_dark() && castling_possible.0.1 && pieces[5+7*8].is_empty() && pieces[6+7*8].is_empty() {
                            let mut p = pieces.clone();
                            if !is_check(white_to_play_next, p) { // check if king isnt currently in check
                                p[5+7*8] = Piece::KingL;
                                p[4+7*8] = Piece::Empty;
                                if !is_check(white_to_play_next, p) {  // check if king desnt move through check
                                    cpm.push(7*8+6);
                                }
                            }
                        }
                    }
                }

                if pieces[piece].is_rook() {
                    let dpos_list:[(isize,isize);4] = [(1,0), (-1,0), (0,1), (0,-1)];

                    for dpos in dpos_list {
                        let mut x = (sx as isize + dpos.0) as usize;
                        let mut y = (sy as isize + dpos.1) as usize;
                        if x > 7 || y > 7 {
                            continue
                        }
                        while pieces[x+y*8].is_empty() || pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                            cpm.push(x+y*8);
                            if (!pieces[x+y*8].is_empty()) && pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                                break
                            }
                            x = (x as isize + dpos.0) as usize;
                            y = (y as isize + dpos.1) as usize;
                            if x > 7 || y > 7 {
                                break
                            }
                        }
                    }
                }

                if pieces[piece].is_bishop() {
                    let dpos_list:[(isize,isize);4] = [(1,1), (-1,1), (1,-1), (-1,-1)];

                    for dpos in dpos_list {
                        let mut x = (sx as isize + dpos.0) as usize;
                        let mut y = (sy as isize + dpos.1) as usize;
                        if x > 7 || y > 7 {
                            continue
                        }
                        while pieces[x+y*8].is_empty() || pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                            cpm.push(x+y*8);
                            if (!pieces[x+y*8].is_empty()) && pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                                break
                            }
                            x = (x as isize + dpos.0) as usize;
                            y = (y as isize + dpos.1) as usize;
                            if x > 7 || y > 7 {
                                break
                            }
                        }
                    }
                }

                if pieces[piece].is_queen() {
                    let dpos_list:[(isize,isize);8] = [(1,1), (-1,1), (1,-1), (-1,-1), (1,0), (-1,0), (0,1), (0,-1)];

                    for dpos in dpos_list {
                        let mut x = (sx as isize + dpos.0) as usize;
                        let mut y = (sy as isize + dpos.1) as usize;
                        if x > 7 || y > 7 {
                            continue
                        }
                        while pieces[x+y*8].is_empty() || pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                            cpm.push(x+y*8);
                            if (!pieces[x+y*8].is_empty()) && pieces[x+y*8].is_dark() != pieces[piece].is_dark() {
                                break
                            }
                            x = (x as isize + dpos.0) as usize;
                            y = (y as isize + dpos.1) as usize;
                            if x > 7 || y > 7 {
                                break
                            }
                        }
                    }
                }

                if pieces[piece].is_knight() {
                    let pm: Vec<(usize,usize)> = vec![(sx+2,sy+1),(sx+2,sy-1),(sx-2,sy+1),(sx-2,sy-1),(sx+1,sy+2),(sx-1,sy+2),(sx+1,sy-2),(sx-1,sy-2)];
                    for i in 0..pm.len() {
                        if !(pm[i].0 > 7 || pm[i].1 > 7) {
                            if pieces[pm[i].0+pm[i].1*8].is_empty() || (pieces[pm[i].0+pm[i].1*8].is_dark() != pieces[piece].is_dark()) {
                                cpm.push(pm[i].0+pm[i].1*8);
                            }
                        }
                    }
                }

                if pieces[piece].is_pawn() {
                    let s: isize = if pieces[piece].is_dark() {8} else {-8};

                    if sy == 0 || sy == 7 {
                        println!("pawn didnt promote correctly");
                        panic!();
                    }

                    if pieces[(piece as isize+s) as usize].is_empty() {
                        cpm.push((piece as isize+s) as usize);
                    }
                    if sy == {if pieces[piece].is_dark() {1} else {6}} && pieces[(piece as isize+s) as usize].is_empty() && pieces[(piece as isize+s*2) as usize].is_empty() {
                        cpm.push((piece as isize+s*2) as usize);
                    }
                    if sx != 7 {
                        if  (!pieces[(piece as isize+s+1) as usize].is_empty()) && pieces[(piece as isize+s+1) as usize].is_dark() != pieces[piece].is_dark() {
                            cpm.push((piece as isize+s+1) as usize);
                        }
                    }
                    if sx != 0 {
                        if  (!pieces[(piece as isize+s-1) as usize].is_empty()) && pieces[(piece as isize+s-1) as usize].is_dark() != pieces[piece].is_dark() {
                            cpm.push((piece as isize+s-1) as usize);
                        }
                    }
                    if (sy == 3 || sy == 4) && ((piece as isize+s-1) as usize) == en_passant_position {
                        cpm.push((piece as isize+s-1) as usize);
                    }
                    if (sy == 3 || sy == 4) && ((piece as isize+s+1) as usize) == en_passant_position {
                        cpm.push((piece as isize+s+1) as usize);
                    }
                    if (sy == 1 && !pieces[piece].is_dark()) || (sy == 6 && pieces[piece].is_dark()) {
                        for m in cpm.clone() {
                            cpm.push(m+64);
                            cpm.push(m+128);
                            cpm.push(m+192);
                        }
                    }
                }

                let mut r: usize = 0;
                for mi in 0..cpm.len() {
                    let mut p = pieces.clone();
                    p[cpm[mi-r]%64] = p[piece];
                    p[piece] = Piece::Empty;
                    if !called_by_is_check {
                        if is_check(white_to_play_next, p) {
                            cpm.remove(mi-r);
                            r += 1;
                        }
                    }
                }

                if cpm.len() != 0 {
                    apm.push((piece, cpm));
                }
            }
        }
    }
    apm
}


fn gen_all_positions(pieces:[Piece;64], en_passant_position:usize, castling_possible:((bool,bool),(bool,bool)), white_to_play_next:bool) -> Vec<([Piece;64], usize, ((bool,bool),(bool,bool)), bool)> {
    let mut app: Vec<([Piece;64], usize, ((bool,bool),(bool,bool)), bool)> = Vec::new();
    let apm = gen_all_moves(pieces, en_passant_position, castling_possible, white_to_play_next, false);

    for p in apm {
        for m in p.1 {
            let mut possible_pos = (pieces.clone(), 255, castling_possible, !white_to_play_next);
            let move_made = make_a_move(en_passant_position, &mut possible_pos.0, p.0, m, castling_possible, true);
            possible_pos.1 = move_made.0;
            possible_pos.2 = move_made.2;
            app.push(possible_pos);
        }
    }
    
    app
}


fn eval(pieces:[Piece;64], en_passant_position:usize, castling_possible:((bool,bool),(bool,bool)), white_to_play_next:bool, depth: i32) -> i32 {
    let mut o:i32 = {
        if white_to_play_next {
            -1000
        } else {
            1000
        }
    };
    let app = gen_all_positions(pieces, en_passant_position, castling_possible, white_to_play_next);
    if depth <= 0 {
        o = 0;

        // look for checkmate or stealmate
        if app.len() == 0 {
            if is_check(white_to_play_next, pieces) {
                if white_to_play_next {
                    return 1000000
                } else {
                    return -1000000
                }
            } else {
                return 0;
            }
        }
    
        // count material
        for p in pieces {
            let v = {
                if p.is_queen() {
                    9
                }
                else if p.is_rook() {
                    5
                }
                else if p.is_bishop() {
                    3
                }
                else if p.is_knight() {
                    3
                }
                else if p.is_pawn() {
                    1
                }
                else {
                    0
                }
            };
            let s = {
                if p.is_dark() {
                    -1
                } else {
                    1
                }
            };
            o += s*v;
        
        }
    }
    else {
        for ppi in 0..app.len() {
            let e = eval(app[ppi].0, app[ppi].1, app[ppi].2, app[ppi].3, depth-3);
            if (o + 3 > e) != white_to_play_next {
                let e = eval(app[ppi].0, app[ppi].1, app[ppi].2, app[ppi].3, depth-2);
                if (o + 1 > e) != white_to_play_next {
                    let e = eval(app[ppi].0, app[ppi].1, app[ppi].2, app[ppi].3, depth-1);
                    if (o > e) != white_to_play_next {
                        o = e;
                    }
                }
            }
        }
    }
    o
}


fn best_move(pieces:[Piece;64], en_passant_position:usize, castling_possible:((bool,bool),(bool,bool)), white_to_play_next:bool, depth: i32) -> (usize, usize) {
    let mut app: Vec<([Piece;64], usize, ((bool,bool),(bool,bool)), bool, i32, (usize, usize))> = Vec::new();
    let apm = gen_all_moves(pieces, en_passant_position, castling_possible, white_to_play_next, false);

    for p in apm {
        for m in p.1 {
            let mut possible_pos = (pieces.clone(), 255, castling_possible, !white_to_play_next, 0, (p.0, m));
            let move_made = make_a_move(en_passant_position, &mut possible_pos.0, p.0, m, castling_possible, true);
            possible_pos.1 = move_made.0;
            possible_pos.2 = move_made.2;
            possible_pos.4 = eval(possible_pos.0, possible_pos.1, possible_pos.2, possible_pos.3, depth-2);
            app.push(possible_pos);
        }
    }

    if white_to_play_next {
        app.sort_by_key(|k| -k.4);
    } else {
        app.sort_by_key(|k| k.4);
    }

    for ppi in 0..(app.len().min(6)) {
        app[ppi].4 = eval(app[ppi].0, app[ppi].1, app[ppi].2, app[ppi].3, depth-1);
    }

    if white_to_play_next {
        app.sort_by_key(|k| -k.4);
    } else {
        app.sort_by_key(|k| k.4);
    }

    for ppi in 0..(app.len().min(3)) {
        app[ppi].4 = eval(app[ppi].0, app[ppi].1, app[ppi].2, app[ppi].3, depth);
    }

    if white_to_play_next {
        app.sort_by_key(|k| -k.4);
    } else {
        app.sort_by_key(|k| k.4);
    }

    app[0].5
}

 
fn move_to_notation(m: (usize, usize)) -> String { // converts a (from, to) to long algebraic notation
    let mut s1 = String::from(""); // start pos
    let mut s2 = String::from(""); // target pos
    let mut s3 = String::from(""); // for promoting

    s1.push(match m.0%8 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => {panic!()}
    });
    s1.push(match m.0/8 {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => {panic!()}
    });
    s2.push(match m.1%8 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => {panic!()}
    });
    s2.push(match m.1/8 {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => {panic!()}
    });

    s1+&s2+&s3
}
