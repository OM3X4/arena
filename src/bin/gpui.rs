use gpui::{
    App, Application, Bounds, Context, Focusable, KeyBinding, Rgba, Window, WindowBounds,
    WindowOptions, actions, div, img, prelude::*, px, rgb, size,
};
use queenfish::board::Board as QueenFishBoard;
use queenfish::board::bishop_magic::init_bishop_magics;
use queenfish::board::rook_magic::init_rook_magics;
use std::{collections::HashSet, path::Path};

const WHITE_PAWN: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wP.svg";
const WHITE_KNIGHT: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wN.svg";
const WHITE_BISHOP: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wB.svg";
const WHITE_ROOK: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wR.svg";
const WHITE_QUEEN: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wQ.svg";
const WHITE_KING: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\wK.svg";

const BLACK_PAWN: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bP.svg";
const BLACK_KNIGHT: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bN.svg";
const BLACK_BISHOP: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bB.svg";
const BLACK_ROOK: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bR.svg";
const BLACK_QUEEN: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bQ.svg";
const BLACK_KING: &str = "C:\\Learn\\LearnRust\\Chess Arena\\arena\\pieces\\bK.svg";

fn light_board_color() -> Rgba {
    rgb(0xf0d9b5)
}
fn dark_board_color() -> Rgba {
    rgb(0xb58863)
}

actions!(play, [Play]);

struct Board {
    board: QueenFishBoard,
    focus_handle: gpui::FocusHandle,
    available_moves: Vec<(u8, u8)>,
}

impl Focusable for Board {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Board {
    pub fn play(&mut self, _: &Play, _: &mut Window, cx: &mut Context<Self>) {
        let board = &mut self.board;
        let moves = board.generate_moves();
        board.make_move(moves[0]);
        cx.notify();
    }

    pub fn select_square(&mut self, square: u8) {
        let moves = self.board.generate_moves();
        let available_squares = self
            .available_moves
            .iter()
            .map(|mv| mv.1)
            .collect::<Vec<_>>();
        if available_squares.contains(&square) {
            let selected_mv = self
                .available_moves
                .iter()
                .find(|mv| mv.1 == square)
                .unwrap();
            let mv = moves
                .iter()
                .find(|mv| (mv.from() as u8, mv.to() as u8) == *selected_mv)
                .unwrap();
            self.board.make_move(*mv);
            self.available_moves = Vec::new();
            return;
        } else {
            let avail_squares = moves
                .iter()
                .filter(|&x| x.from() == square as usize)
                .map(|&x| (x.from() as u8, x.to() as u8))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            dbg!(&avail_squares);
            self.available_moves = avail_squares;
        }
    }

    pub fn new(focus_handle: gpui::FocusHandle) -> Self {
        Board {
            board: QueenFishBoard::new(),
            focus_handle,
            available_moves: Vec::new(),
        }
    }
}

impl Render for Board {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let squares = (0..64)
            .map(|i| {
                let file = i % 8;
                let rank = i / 8;

                let color = if (file + rank) % 2 == 0 {
                    light_board_color()
                } else {
                    dark_board_color()
                };
                let mut piece_image = "";
                if let Some(piece) = self.board.piece_at[i] {
                    piece_image = match piece as usize {
                        0 => WHITE_PAWN,
                        1 => WHITE_KNIGHT,
                        2 => WHITE_BISHOP,
                        3 => WHITE_ROOK,
                        4 => WHITE_QUEEN,
                        5 => WHITE_KING,
                        6 => BLACK_PAWN,
                        7 => BLACK_KNIGHT,
                        8 => BLACK_BISHOP,
                        9 => BLACK_ROOK,
                        10 => BLACK_QUEEN,
                        11 => BLACK_KING,
                        _ => "",
                    };
                }

                let mut element = div()
                    .size_full()
                    .bg(color)
                    .p_0p5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(img(Path::new(piece_image)).size_full());

                if self
                    .available_moves
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<u8>>()
                    .contains(&(i as u8))
                {
                    if self.board.piece_at[i].is_some() {
                        element = element.child(
                            div()
                                .absolute()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .border_4()
                                        .border_color(rgb(0xaeb187))
                                        .rounded_full()
                                        .w_full() // Adjust size as needed
                                        .h_full(),
                                ),
                        );
                    } else {
                        element = element.child(
                            div()
                                .absolute()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .bg(rgb(0xaeb187))
                                        .rounded_full()
                                        .w(px(20.0)) // Adjust size as needed
                                        .h(px(20.0)),
                                ),
                        );
                    }
                }

                element = element.on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |board, _event, _window, cx| {
                        board.select_square(i as u8);
                        cx.notify();
                    }),
                );
                return element;
            })
            .rev()
            .collect::<Vec<_>>();

        div()
            .key_context("board")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .grid()
            .grid_cols(8)
            .grid_rows(8)
            .children(squares)
            .on_action(cx.listener(Self::play))
    }
}

fn main() {
    init_bishop_magics();
    init_rook_magics();

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.bind_keys([
            KeyBinding::new("space", Play, Some("board")),
            KeyBinding::new("enter", Play, Some("board")),
        ]);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| Board::new(cx.focus_handle())),
        )
        .unwrap();
        cx.activate(true);
    });
}
