use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    style::{Color, Colors},
};
use std::{env, io::Write};

struct Editor {
    lines: Vec<String>,
    current_line: usize,
    current_char: usize,
    previous_current_char: usize,
    syntax_color_buffer: Vec<Color>,
}

#[derive(PartialEq, Eq)]
enum ProcessResult {
    EditorQuit = 0,
    EditorOk = 1,
}

fn editor_process(editor: &mut Editor, event: Event) -> ProcessResult {
    match event {
        //Esc
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => {
            return ProcessResult::EditorQuit;
        }
        //New line
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => {
            if editor.lines[editor.current_line] == "" {
                editor.current_line += 1;
                editor.lines.insert(editor.current_line, String::new());
                editor.current_char = 0;
                editor.previous_current_char = editor.current_char;
            } else if editor.current_char == editor.lines[editor.current_line].len() - 1 {
                editor.current_line += 1;
                editor.lines.insert(editor.current_line, String::new());
                editor.current_char = 0;
                editor.previous_current_char = editor.current_char;
            } else {
                //Get everything before the current char
                let old_line: String =
                    editor.lines[editor.current_line][0..editor.current_char].into();
                //Get everything after the current char
                let new_line: String = editor.lines[editor.current_line]
                    [editor.current_char..editor.lines[editor.current_line].len()]
                    .into();
                editor.lines[editor.current_line] = old_line;
                editor.current_line += 1;
                editor.lines.insert(editor.current_line, new_line);
                editor.current_char = 0;
            }
        }
        //Backspace
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => {
            if editor.current_char > 0 {
                editor.current_char -= 1;
                editor.lines[editor.current_line].remove(editor.current_char);
                editor.previous_current_char = editor.current_char;
            } else if editor.current_line > 0 {
                editor.lines.remove(editor.current_line);
                editor.current_line -= 1;
                editor.current_char = editor.lines[editor.current_line].len();
                editor.previous_current_char = editor.current_char;
            }
        }
        //Left arrow
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        }) => {
            if editor.current_char > 0 {
                editor.current_char -= 1;
                editor.previous_current_char = editor.current_char;
            } else if editor.current_line > 0 {
                editor.current_line -= 1;
                editor.current_char = editor.lines[editor.current_line].len();
                editor.previous_current_char = editor.current_char;
            }
        }
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        }) => {
            if editor.lines[editor.current_line].len() > 0
                && editor.current_char <= editor.lines[editor.current_line].len() - 1
            {
                editor.current_char += 1;
                editor.previous_current_char = editor.current_char;
            } else if editor.current_line < editor.lines.len() - 1 {
                editor.current_line += 1;
                editor.current_char = 0;
                editor.previous_current_char = editor.current_char;
            }
        }
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => {
            if editor.current_line > 0 {
                editor.current_line -= 1;
                if editor.lines[editor.current_line].len() < editor.previous_current_char {
                    editor.current_char = editor.lines[editor.current_line].len();
                } else {
                    editor.current_char = editor.previous_current_char;
                }
            }
        }
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => {
            if editor.current_line < editor.lines.len() - 1 {
                editor.current_line += 1;
                if editor.lines[editor.current_line].len() < editor.previous_current_char {
                    editor.current_char = editor.lines[editor.current_line].len();
                } else {
                    editor.current_char = editor.previous_current_char;
                }
            }
        }

        //Printable characters
        crossterm::event::Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) => {
            if c != '\n' {
                editor.lines[editor.current_line].insert(editor.current_char, c);
                editor.current_char += 1;
                editor.previous_current_char = editor.current_char;
            }
        }
        _ => {}
    }
    return ProcessResult::EditorOk;
}
fn editor_syntax_highlighting(editor: &mut Editor) {
    let mut characters_count = 0;
    for line in editor.lines.iter() {
        characters_count += line.len();
    }
    if editor.syntax_color_buffer.len() != characters_count {
        editor
            .syntax_color_buffer
            .resize(characters_count, Color::Reset);
    }
    //Parser
    let mut c: usize = 0;
    let mut inside_string: bool = false;
    for line in editor.lines.iter() {
        for chr in line.chars() {
            editor.syntax_color_buffer[c] = Color::Reset;
            if chr == '(' || chr == ')' || chr == '[' || chr == ']' || chr == '{' || chr == '}' {
                editor.syntax_color_buffer[c] = Color::Blue;
            }
            if chr == '-'
                || chr == '+'
                || chr == '/'
                || chr == '*'
                || chr == '^'
                || chr == '%'
                || chr == '='
            {
                editor.syntax_color_buffer[c] = Color::Green;
            }
            if chr.is_digit(10) {
                editor.syntax_color_buffer[c] = Color::DarkMagenta;
            }

            if chr == '"' {
                inside_string = !inside_string;
                editor.syntax_color_buffer[c] = Color::Yellow;
            }
            if inside_string {
                editor.syntax_color_buffer[c] = Color::Yellow;
            }
            c += 1;
        }
    }
}
fn editor_draw(editor: &mut Editor) {
    let mut characters_count = 0;
    for line in editor.lines.iter() {
        characters_count += line.len();
    }
    if editor.syntax_color_buffer.len() != characters_count {
        editor
            .syntax_color_buffer
            .resize(characters_count, Color::Reset);
    }
    let column: u16 = editor.current_char as u16;
    let row: u16 = editor.current_line as u16;
    // Clear the terminal
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
    )
    .unwrap();

    // Print the editor.lines
    let mut c: usize = 0;
    for (i, line) in editor.lines.iter().enumerate() {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::MoveTo(0, i as u16),
            crossterm::style::SetColors(Colors::new(Color::Reset, Color::Reset)),
            crossterm::style::Print(format!("{} ", i))
        )
        .unwrap();
        for (y, chr) in line.chars().enumerate() {
            crossterm::execute!(
                std::io::stdout(),
                crossterm::cursor::MoveTo(y as u16 + 2, i as u16),
                crossterm::style::SetColors(Colors::new(
                    editor.syntax_color_buffer[c],
                    Color::Reset
                )),
                crossterm::style::Print(chr)
            )
            .unwrap();
            c += 1;
        }
    }

    let (width, height) = crossterm::terminal::size().expect("Failed to get terminal size");

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(width - 64, height),
        crossterm::style::SetColors(Colors::new(Color::Reset, Color::Reset)),
        crossterm::style::Print(format!(
            "Terminal Size: {} {} lines: {} Chars: {}",
            width,
            height,
            editor.lines.len(),
            characters_count
        ))
    )
    .unwrap();

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(column + 2, row)
    )
    .unwrap();

    // Flush output to ensure it's visible
    std::io::stdout().flush().unwrap();
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut editor: Editor = Editor {
        lines: Vec::new(),
        current_line: 0,
        current_char: 0,
        previous_current_char: 0,
        syntax_color_buffer: Vec::new(),
    };
    editor.lines.push(String::new());
    crossterm::terminal::enable_raw_mode().unwrap();
    loop {
        if let Ok(event) = crossterm::event::read() {
            let result: ProcessResult = editor_process(&mut editor, event);
            if result == ProcessResult::EditorQuit {
                break;
            }
        }

        //Syntax highlighting
        editor_syntax_highlighting(&mut editor);
        //Draw
        editor_draw(&mut editor);
    }
}
