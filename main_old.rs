use speedy2d::color::Color;
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper, MouseButton};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Speedy2D: Load image", (544, 544)).unwrap();
    window.run_loop(MyWindowHandler {
        color_to_play: PieceColor::Light,
        dragged_piece: (255,255),
        mouse_pos: (0.,0.),
        loaded: false,
        images: [None,None,None,None,None,None,None,None,None,None,None,None,None],
        pieces: [[Piece {
            piecetype: PieceType::Empty,
            color: PieceColor::Light
        };8];8]
    });
}


#[derive(Clone,Copy,PartialEq)]
enum PieceColor {
    Dark,
    Light
}

#[derive(Clone,Copy,PartialEq)]
enum PieceType {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Clone,Copy,PartialEq)]
struct Piece {
    piecetype: PieceType,
    color: PieceColor
}


struct MyWindowHandler {
    color_to_play: PieceColor,
    dragged_piece: (usize,usize),
    mouse_pos: (f32,f32),
    loaded: bool,
    images: [Option<ImageHandle>;13],
    pieces: [[Piece;8];8]
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
                    self.pieces[x][y] = {
                        if y == 1 {
                            Piece {
                                piecetype: PieceType::Pawn,
                                color: PieceColor::Dark
                            }
                        }
                        else if y == 6 {
                            Piece {
                                piecetype: PieceType::Pawn,
                                color: PieceColor::Light
                            }
                        }
                        else if y == 0 {
                            match x {
                                0 => Piece {piecetype: PieceType::Rook,color: PieceColor::Dark},
                                1 => Piece {piecetype: PieceType::Knight,color: PieceColor::Dark},
                                2 => Piece {piecetype: PieceType::Bishop,color: PieceColor::Dark},
                                3 => Piece {piecetype: PieceType::Queen,color: PieceColor::Dark},
                                4 => Piece {piecetype: PieceType::King,color: PieceColor::Dark},
                                5 => Piece {piecetype: PieceType::Bishop,color: PieceColor::Dark},
                                6 => Piece {piecetype: PieceType::Knight,color: PieceColor::Dark},
                                7 => Piece {piecetype: PieceType::Rook,color: PieceColor::Dark},
                                _ => Piece {piecetype: PieceType::Empty,color: PieceColor::Dark}
                            }
                        }
                        else if y == 7 {
                            match x {
                                0 => Piece {piecetype: PieceType::Rook,color: PieceColor::Light},
                                1 => Piece {piecetype: PieceType::Knight,color: PieceColor::Light},
                                2 => Piece {piecetype: PieceType::Bishop,color: PieceColor::Light},
                                3 => Piece {piecetype: PieceType::Queen,color: PieceColor::Light},
                                4 => Piece {piecetype: PieceType::King,color: PieceColor::Light},
                                5 => Piece {piecetype: PieceType::Bishop,color: PieceColor::Light},
                                6 => Piece {piecetype: PieceType::Knight,color: PieceColor::Light},
                                7 => Piece {piecetype: PieceType::Rook,color: PieceColor::Light},
                                _ => Piece {piecetype: PieceType::Empty,color: PieceColor::Light}
                            }
                        }
                        else {
                            Piece {piecetype: PieceType::Empty,color: PieceColor::Light}
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
                if self.pieces[x][y].piecetype != PieceType::Empty {
                    graphics.draw_image( {
                        if self.dragged_piece != (x,y) {
                            (16.+x as f32*64.,16.+y as f32*64.)
                        } else {
                            (self.mouse_pos.0-30., self.mouse_pos.1-30.)
                        }
                    }, {
                        match self.pieces[x][y].piecetype {
                            PieceType::Empty => &u_images[0],
                            PieceType::Pawn => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[1],
                                PieceColor::Dark => &u_images[2] }},
                            PieceType::Rook => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[3],
                                PieceColor::Dark => &u_images[4] }},
                            PieceType::Knight => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[5],
                                PieceColor::Dark => &u_images[6] }},
                            PieceType::Bishop => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[7],
                                PieceColor::Dark => &u_images[8] }},
                            PieceType::Queen => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[9],
                                PieceColor::Dark => &u_images[10] }},
                            PieceType::King => {match self.pieces[x][y].color {
                                PieceColor::Light => &u_images[11],
                                PieceColor::Dark => &u_images[12] }},
                        }
                    })
                }
            }
        }
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.mouse_pos.0 = position.x;
        self.mouse_pos.1 = position.y
    }

    fn on_mouse_button_down( &mut self, helper: &mut WindowHelper<()>, button: speedy2d::window::MouseButton ) {
        if button == MouseButton::Left {
            self.dragged_piece = (((self.mouse_pos.0 - 16.)/64.) as usize, ((self.mouse_pos.1 - 16.)/64.) as usize);
        }
    }

    fn on_mouse_button_up( &mut self, helper: &mut WindowHelper<()>, button: speedy2d::window::MouseButton ) {
        if button == MouseButton::Left {
            let tg_pos = (((self.mouse_pos.0 - 16.)/64.) as usize, ((self.mouse_pos.1 - 16.)/64.) as usize);

            if tg_pos != self.dragged_piece && (tg_pos.0 < 8 && tg_pos.1 < 8) &&
                self.pieces[self.dragged_piece.0][self.dragged_piece.1].color == self.color_to_play && {
                match self.pieces[self.dragged_piece.0][self.dragged_piece.1].piecetype {
                    PieceType::Empty => false,
                    PieceType::Pawn => {
                        // move forward by 1 tile
                        (tg_pos.0 == self.dragged_piece.0 && tg_pos.1 == (self.dragged_piece.1 + {
                            match self.pieces[self.dragged_piece.0][self.dragged_piece.1].color {
                                PieceColor::Light => -1_isize,
                                PieceColor::Dark => 1_isize
                            }
                        } as usize) && self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty)
                        ||
                        // take diagonally
                        ((tg_pos.0 == self.dragged_piece.0-1 || tg_pos.0 == self.dragged_piece.0+1) && tg_pos.1 == (self.dragged_piece.1 + {
                            match self.pieces[self.dragged_piece.0][self.dragged_piece.1].color {
                                PieceColor::Light => -1_isize,
                                PieceColor::Dark => 1_isize
                            }
                        } as usize) && self.pieces[tg_pos.0][tg_pos.1].piecetype != PieceType::Empty && self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color)
                        ||
                        // move forward by 2 tiles
                        (tg_pos.0 == self.dragged_piece.0 && tg_pos.1 == (self.dragged_piece.1 + {
                            match self.pieces[self.dragged_piece.0][self.dragged_piece.1].color {
                                PieceColor::Light => -2_isize,
                                PieceColor::Dark => 2_isize
                            }
                        } as usize) && self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty && (self.dragged_piece.1 == 1 || self.dragged_piece.1 == 6))
                        // TODO: en passant, promoting
                    },
                    PieceType::Rook => {
                        if (tg_pos.0 == self.dragged_piece.0 || tg_pos.1 == self.dragged_piece.1) &&
                        (self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color ||
                        self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty) {
                            let mut is_path_empty = true;
                            if tg_pos.0 == self.dragged_piece.0 {
                                if tg_pos.1 > self.dragged_piece.1 {
                                    for y in self.dragged_piece.1+1..tg_pos.1 {
                                        if self.pieces[self.dragged_piece.0][y].piecetype != PieceType::Empty {
                                            is_path_empty = false;
                                        }
                                    }
                                } else {
                                    for y in tg_pos.1+1..self.dragged_piece.1 {
                                        if self.pieces[self.dragged_piece.0][y].piecetype != PieceType::Empty {
                                            is_path_empty = false;
                                        }
                                    }
                                }
                            } else {
                                if tg_pos.0 > self.dragged_piece.0 {
                                    for x in self.dragged_piece.0+1..tg_pos.0 {
                                        if self.pieces[x][self.dragged_piece.1].piecetype != PieceType::Empty {
                                            is_path_empty = false;
                                        }
                                    }
                                } else {
                                    for x in tg_pos.0+1..self.dragged_piece.0 {
                                        if self.pieces[x][self.dragged_piece.1].piecetype != PieceType::Empty {
                                            is_path_empty = false;
                                        }
                                    }
                                }
                            }
                            is_path_empty
                        } else {
                            false
                        }
                        // TODO: castling
                    },
                    PieceType::Knight => {
                        ((tg_pos.0 == self.dragged_piece.0+2 && tg_pos.1 == self.dragged_piece.1+1) ||
                        (tg_pos.0 == self.dragged_piece.0+2 && tg_pos.1 == self.dragged_piece.1-1) || 
                        (tg_pos.0 == self.dragged_piece.0-2 && tg_pos.1 == self.dragged_piece.1+1) || 
                        (tg_pos.0 == self.dragged_piece.0-2 && tg_pos.1 == self.dragged_piece.1-1) ||
                        (tg_pos.0 == self.dragged_piece.0+1 && tg_pos.1 == self.dragged_piece.1+2) ||
                        (tg_pos.0 == self.dragged_piece.0+1 && tg_pos.1 == self.dragged_piece.1-2) || 
                        (tg_pos.0 == self.dragged_piece.0-1 && tg_pos.1 == self.dragged_piece.1+2) || 
                        (tg_pos.0 == self.dragged_piece.0-1 && tg_pos.1 == self.dragged_piece.1-2))
                        &&
                        (self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color ||
                        self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty)
                    },
                    PieceType::Bishop => {
                        (self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color ||
                        self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty)
                        &&
                        {
                            if (tg_pos.0 as isize - self.dragged_piece.0 as isize == tg_pos.1 as isize - self.dragged_piece.1 as isize 
                            || -(tg_pos.0 as isize - self.dragged_piece.0 as isize) == tg_pos.1 as isize - self.dragged_piece.1 as isize) {
                                let mut is_path_empty = true;
                                let mut dx:isize = 0;
                                let mut dy:isize = 0;

                                if tg_pos.0 > self.dragged_piece.0 {
                                    dx += 1;
                                } else {
                                    dx += -1;
                                }

                                if tg_pos.1 > self.dragged_piece.1 {
                                    dy += 1;
                                } else {
                                    dy += -1;
                                }

                                while (self.dragged_piece.0 as isize + dx, self.dragged_piece.1 as isize  + dy) != (tg_pos.0 as isize, tg_pos.1 as isize) {
                                    if self.pieces[(self.dragged_piece.0 as isize + dx) as usize][(self.dragged_piece.1 as isize  + dy) as usize].piecetype != PieceType::Empty {
                                        is_path_empty = false;
                                        break;
                                    }
                                    if tg_pos.0 > self.dragged_piece.0 {
                                        dx += 1;
                                    } else {
                                        dx += -1;
                                    }
    
                                    if tg_pos.1 > self.dragged_piece.1 {
                                        dy += 1;
                                    } else {
                                        dy += -1;
                                    }
                                }
                                is_path_empty
                            } else {
                                false
                            }
                        }
                        // TODO: not jumping over pieces
                    },
                    PieceType::Queen => {
                        ((tg_pos.0 as isize - self.dragged_piece.0 as isize == tg_pos.1 as isize - self.dragged_piece.1 as isize 
                        || -(tg_pos.0 as isize - self.dragged_piece.0 as isize) == tg_pos.1 as isize - self.dragged_piece.1 as isize) ||
                        (tg_pos.0 == self.dragged_piece.0 || tg_pos.1 == self.dragged_piece.1)) &&
                        (self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color ||
                        self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty)
                        // TODO: not jumping over pieces
                    },
                    PieceType::King => {
                        ((tg_pos.0 == self.dragged_piece.0+1 && tg_pos.1 == self.dragged_piece.1+1) ||
                        (tg_pos.0 == self.dragged_piece.0+1 && tg_pos.1 == self.dragged_piece.1) || 
                        (tg_pos.0 == self.dragged_piece.0+1 && tg_pos.1 == self.dragged_piece.1-1) || 
                        (tg_pos.0 == self.dragged_piece.0 && tg_pos.1 == self.dragged_piece.1-1) ||
                        (tg_pos.0 == self.dragged_piece.0 && tg_pos.1 == self.dragged_piece.1+1) ||
                        (tg_pos.0 == self.dragged_piece.0-1 && tg_pos.1 == self.dragged_piece.1+1) || 
                        (tg_pos.0 == self.dragged_piece.0-1 && tg_pos.1 == self.dragged_piece.1) || 
                        (tg_pos.0 == self.dragged_piece.0-1 && tg_pos.1 == self.dragged_piece.1-1))
                        &&
                        (self.pieces[tg_pos.0][tg_pos.1].color != self.pieces[self.dragged_piece.0][self.dragged_piece.1].color ||
                        self.pieces[tg_pos.0][tg_pos.1].piecetype == PieceType::Empty)
                        // TODO: check, chackmate, castling
                    },
                }
            } {
                self.pieces[tg_pos.0][tg_pos.1] = self.pieces[self.dragged_piece.0][self.dragged_piece.1];
                self.pieces[self.dragged_piece.0][self.dragged_piece.1].piecetype = PieceType::Empty;
                self.color_to_play = {
                    match self.color_to_play {
                        PieceColor::Dark => PieceColor::Light,
                        PieceColor::Light => PieceColor::Dark
                    }
                }
            }

            self.dragged_piece = (255,255);
        }
    }
}
