use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::QueueableCommand;
use std::io::{stdout, Write};

pub const SENIOR_TUX: &[&str] = &[
    "           BBBBBBBBBBBB             ",
    "        BBBWWWWWWWWWWWWBBB          ",
    "      BBWWWWWWWWWWWWWWWWWWBB        ",
    "     BWWW  BBB  WW  BBB  ZZBB       ", // Disappointed eyes
    "    BWWW   BBB  WW  BBB   ZZB       ",
    "    BWWW        YY        ZZB       ",
    "    BWWW      YYYYYY      ZZB       ",
    "    BWWW      YYYYYY      ZZB       ",
    "    BWWW                  ZZB       ",
    "     BWWWWWWWWWWWWWWWWWWWWBB        ",
    "      BBBBBBBBBBBBBBBBBBBB          ",
    "    BBBWWWWWWWWWWWWWWWWWWBBB        ",
    "   BWWWWWWWWWWWWWWWWWWWWWWWWB       ",
    "  BWWWWWWWWWWWWWWWWWWWWWWWWWWB      ",
    "  BWWWWWWW        WWWWWWWWWWWB      ", 
    " BWWWWWWWW GGGGGG WWWWWWWWWWWWB     ", // Holding Coffee Mug
    " BWWWWWWWW GCC CCG WWWWWWWWWWWWB    ", 
    " BWWWWWWWW GCC CCG WWWWWWWWWWWWB    ",
    " BWWWWWWWW GGGGGGG WWWWWWWWWWWWB    ",
    " BWWWWWWWW        WWWWWWWWWWWWWB    ",
    " BWWWWWWWWWWWWWWWWWWWWWWWWWWWWWB    ",
    "  BWWWWWWWWWWWWWWWWWWWWWWWWWWWB     ",
    "   BBBBBBBBBBBBBBBBBBBBBBBBBBB      ",
    "     YYY                 YYY        ", 
    "    YYYYY               YYYYY       ",
    "                                    " 
];

fn get_color(c: char) -> Option<Color> {
    match c {
        'B' => Some(Color::Rgb { r: 34, g: 34, b: 34 }),
        'W' => Some(Color::Rgb { r: 240, g: 240, b: 240 }),
        'Y' => Some(Color::Rgb { r: 255, g: 174, b: 0 }),
        'G' => Some(Color::Rgb { r: 93, g: 93, b: 93 }),
        'C' => Some(Color::Rgb { r: 111, g: 78, b: 55 }),
        'Z' => Some(Color::Rgb { r: 170, g: 170, b: 170 }),
        ' ' => None, // Transparent / Reset
        _ => None,
    }
}

pub fn render() -> std::io::Result<()> {
    let mut stdout = stdout();

    // Iterate rows in steps of 2
    for i in (0..SENIOR_TUX.len()).step_by(2) {
        let top_row = SENIOR_TUX[i];
        // Ensure we don't go out of bounds if there's an odd number of rows
        let bottom_row = if i + 1 < SENIOR_TUX.len() {
            SENIOR_TUX[i+1]
        } else {
            "" // Should ideally be padded with spaces if needed
        };

        // We assume rows are equal length for simplicity, or handle mismatches
        let len = top_row.len().max(bottom_row.len());
        
        for col in 0..len {
            let top_char = top_row.chars().nth(col).unwrap_or(' ');
            let bottom_char = bottom_row.chars().nth(col).unwrap_or(' ');

            let top_color = get_color(top_char);
            let bottom_color = get_color(bottom_char);

            // Logic:
            // If top is transparent and bottom is transparent -> Print space
            // If top has color -> Set FG to top color
            // If bottom has color -> Set BG to bottom color
            // Print Upper Half Block '▀'
            
            // Optimization: If both are space, just move cursor or print space? 
            // Better to just print space with reset colors if both empty.
            if top_color.is_none() && bottom_color.is_none() {
                stdout.queue(Print(" "))?;
                continue;
            }

            if let Some(c) = top_color {
                stdout.queue(SetForegroundColor(c))?;
            } else {
                // If top is transparent, we might need to handle it. 
                // However, '▀' draws the top half. If top is transparent, we want text color to be... transparency?
                // Terminal doesn't support "transparent via character".
                // Trick: if top is transparent but bottom is color:
                // We can use Lower Half Block '▄' with FG = BottomColor?
                // OR: Standard approach: FG = Top, BG = Bottom, printing '▀'.
                // If FG is "None" (default), it might be white/grey.
                // We'll set it to Reset if None, but that might look wrong.
                // Best approximation for pixel art:
                // If top is empty, use space? No, bottom might have content.
                // If `top` is empty, we must rely on `bottom`.
                // If we print '▀', we see FG (top). We see BG (bottom).
                // If we want top empty (terminal background) and bottom colored:
                // Set FG=Default, BG=BottomColor. '▀' will draw Default on top. NOT GOOD.
                // We want to see Background on top, Color on bottom.
                // Solution: Set FG = BottomColor. Print '▄' (Lower Half Block).
                // BUT, to keep loop simple, let's stick to standard Algorithm requested:
                // "ForeGround Color = Top Pixel Char. BackGround Color = Bottom Pixel Char. Print ▀."
                
                // Handling transparent "None":
                // If Top is None: we assume it matches terminal background.
                // `crossterm::style::Color::Reset` might work if user theme is dark.
                 stdout.queue(SetForegroundColor(Color::Reset))?; 
            }

            if let Some(c) = bottom_color {
                stdout.queue(SetBackgroundColor(c))?;
            } else {
                stdout.queue(SetBackgroundColor(Color::Reset))?;
            }

            stdout.queue(Print("▀"))?;
            
            // Reset after each char to prevent bleeding
            stdout.queue(SetBackgroundColor(Color::Reset))?;
            stdout.queue(SetForegroundColor(Color::Reset))?;
        }
        stdout.queue(Print("\n"))?;
    }
    
    stdout.flush()?;
    Ok(())
}
