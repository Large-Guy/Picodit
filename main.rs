use editor::*;

mod editor;

fn main() {
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
