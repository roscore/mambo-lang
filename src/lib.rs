//! Mambo Lang (맘보랭): a four-beat, bar-based stack language.
//!
//! A bar ends at `|`. Exact whitespace-delimited Korean tokens each occupy one
//! beat; other words are ignored as commentary. A bar has at most four beats.

use std::fmt;

/// A one-based Unicode scalar position in source text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// The kind of a parser or runtime failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    TooManyBeats,
    LoopNeedsOwnBar,
    UnmatchedFinale,
    UnclosedChorus,
    LoopDepthLimit,
    StackUnderflow,
    StackLimit,
    StepLimit,
    Overflow,
    InvalidScalar,
    InvalidLoopCounter,
}

impl ErrorKind {
    fn description(self) -> &'static str {
        match self {
            Self::TooManyBeats => "bar has more than four beats",
            Self::LoopNeedsOwnBar => "loop marker must be alone in its bar",
            Self::UnmatchedFinale => "피날레 has no matching 코러스",
            Self::UnclosedChorus => "코러스 has no matching 피날레",
            Self::LoopDepthLimit => "loop nesting limit exceeded",
            Self::StackUnderflow => "stack needs more values",
            Self::StackLimit => "stack limit exceeded",
            Self::StepLimit => "step limit exceeded",
            Self::Overflow => "integer arithmetic overflow",
            Self::InvalidScalar => "output value is not a Unicode scalar",
            Self::InvalidLoopCounter => "loop counter must be nonnegative",
        }
    }
}

/// A failure with its source location.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    position: Position,
}

impl Error {
    fn new(kind: ErrorKind, position: Position) -> Self {
        Self { kind, position }
    }

    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    pub const fn position(self) -> Position {
        self.position
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.position.line,
            self.position.column,
            self.kind.description()
        )
    }
}

impl std::error::Error for Error {}

/// A recognized beat.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BeatKind {
    PushOne,
    PushTwo,
    PushZero,
    Add,
    Subtract,
    Duplicate,
    Swap,
    Output,
    Chorus,
    Finale,
}

impl BeatKind {
    fn from_word(word: &str) -> Option<Self> {
        Some(match word {
            "둠" => Self::PushOne,
            "짝" => Self::PushTwo,
            "쉼" => Self::PushZero,
            "plus" | "플러스" => Self::Add,
            "마이너스" => Self::Subtract,
            "스텝" => Self::Duplicate,
            "턴" => Self::Swap,
            "솔로" => Self::Output,
            "코러스" => Self::Chorus,
            "피날레" => Self::Finale,
            _ => return None,
        })
    }
}

/// A beat and the location of its first Unicode scalar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Beat {
    pub kind: BeatKind,
    pub position: Position,
}

/// Parsed bars. Empty bars are silent measures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    bars: Vec<Vec<Beat>>,
}

impl Program {
    pub fn bars(&self) -> &[Vec<Beat>] {
        &self.bars
    }

    /// Validate the maximum number of nested chorus blocks.
    pub fn validate_depth(&self, max_depth: usize) -> Result<(), Error> {
        let mut depth = 0usize;
        for beat in self.bars.iter().flatten() {
            match beat.kind {
                BeatKind::Chorus => {
                    depth += 1;
                    if depth > max_depth {
                        return Err(Error::new(ErrorKind::LoopDepthLimit, beat.position));
                    }
                }
                BeatKind::Finale => depth -= 1,
                _ => {}
            }
        }
        Ok(())
    }
}

/// Resource limits. Each executed beat, including loop markers, costs one step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub max_steps: usize,
    pub max_stack: usize,
    pub max_loop_depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_steps: 100_000,
            max_stack: 1_024,
            max_loop_depth: 64,
        }
    }
}

/// Parse UTF-8 source into four-beat bars.
pub fn parse(source: &str) -> Result<Program, Error> {
    let mut bars = vec![Vec::new()];
    let mut word = String::new();
    let mut start = Position::new(1, 1);
    let mut position = start;
    let mut previous_cr = false;

    fn finish(word: &mut String, start: Position, bars: &mut [Vec<Beat>]) -> Result<(), Error> {
        if let Some(kind) = BeatKind::from_word(word) {
            let bar = bars.last_mut().expect("parser always has a bar");
            if bar.len() == 4 {
                return Err(Error::new(ErrorKind::TooManyBeats, start));
            }
            bar.push(Beat {
                kind,
                position: start,
            });
        }
        word.clear();
        Ok(())
    }

    for ch in source.chars() {
        if ch == '|' || ch.is_whitespace() {
            finish(&mut word, start, &mut bars)?;
            if ch == '|' {
                bars.push(Vec::new());
            }
        } else {
            if word.is_empty() {
                start = position;
            }
            word.push(ch);
        }
        match ch {
            '\r' => {
                position.line += 1;
                position.column = 1;
            }
            '\n' if !previous_cr => {
                position.line += 1;
                position.column = 1;
            }
            '\n' => {}
            _ => position.column += 1,
        }
        previous_cr = ch == '\r';
    }
    finish(&mut word, start, &mut bars)?;
    let program = Program { bars };
    let mut openings = Vec::new();
    for bar in &program.bars {
        for beat in bar {
            if matches!(beat.kind, BeatKind::Chorus | BeatKind::Finale) && bar.len() != 1 {
                return Err(Error::new(ErrorKind::LoopNeedsOwnBar, beat.position));
            }
            match beat.kind {
                BeatKind::Chorus => {
                    openings.push(beat.position);
                    if openings.len() > Limits::default().max_loop_depth {
                        return Err(Error::new(ErrorKind::LoopDepthLimit, beat.position));
                    }
                }
                BeatKind::Finale => {
                    openings
                        .pop()
                        .ok_or_else(|| Error::new(ErrorKind::UnmatchedFinale, beat.position))?;
                }
                _ => {}
            }
        }
    }
    if let Some(position) = openings.pop() {
        return Err(Error::new(ErrorKind::UnclosedChorus, position));
    }
    Ok(program)
}

fn push(stack: &mut Vec<i64>, value: i64, limit: usize, at: Position) -> Result<(), Error> {
    if stack.len() >= limit {
        return Err(Error::new(ErrorKind::StackLimit, at));
    }
    stack.push(value);
    Ok(())
}

fn top(stack: &[i64], at: Position) -> Result<i64, Error> {
    stack
        .last()
        .copied()
        .ok_or_else(|| Error::new(ErrorKind::StackUnderflow, at))
}

/// Execute parsed source, collecting its Unicode output.
pub fn execute(program: &Program, limits: Limits) -> Result<String, Error> {
    program.validate_depth(limits.max_loop_depth)?;
    let beats: Vec<Beat> = program.bars.iter().flatten().copied().collect();
    let mut pairs = vec![0; beats.len()];
    let mut openings = Vec::new();
    for (index, beat) in beats.iter().enumerate() {
        match beat.kind {
            BeatKind::Chorus => openings.push(index),
            BeatKind::Finale => {
                let start = openings
                    .pop()
                    .expect("validated program has balanced loops");
                pairs[index] = start;
                pairs[start] = index;
            }
            _ => {}
        }
    }
    let mut stack = Vec::new();
    let mut output = String::new();
    let mut pc = 0;
    let mut steps = 0;
    while let Some(beat) = beats.get(pc) {
        let at = beat.position;
        if steps >= limits.max_steps {
            return Err(Error::new(ErrorKind::StepLimit, at));
        }
        steps += 1;
        match beat.kind {
            BeatKind::PushOne => push(&mut stack, 1, limits.max_stack, at)?,
            BeatKind::PushTwo => push(&mut stack, 2, limits.max_stack, at)?,
            BeatKind::PushZero => push(&mut stack, 0, limits.max_stack, at)?,
            BeatKind::Add | BeatKind::Subtract => {
                if stack.len() < 2 {
                    return Err(Error::new(ErrorKind::StackUnderflow, at));
                }
                let right = stack[stack.len() - 1];
                let left = stack[stack.len() - 2];
                let result = if beat.kind == BeatKind::Add {
                    left.checked_add(right)
                } else {
                    left.checked_sub(right)
                }
                .ok_or_else(|| Error::new(ErrorKind::Overflow, at))?;
                stack.pop();
                *stack.last_mut().expect("two values checked") = result;
            }
            BeatKind::Duplicate => {
                let value = top(&stack, at)?;
                push(&mut stack, value, limits.max_stack, at)?;
            }
            BeatKind::Swap => {
                if stack.len() < 2 {
                    return Err(Error::new(ErrorKind::StackUnderflow, at));
                }
                let len = stack.len();
                stack.swap(len - 1, len - 2);
            }
            BeatKind::Output => {
                let value = top(&stack, at)?;
                let scalar = u32::try_from(value)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or_else(|| Error::new(ErrorKind::InvalidScalar, at))?;
                stack.pop();
                output.push(scalar);
            }
            BeatKind::Chorus | BeatKind::Finale => {
                let counter = top(&stack, at)?;
                if counter < 0 {
                    return Err(Error::new(ErrorKind::InvalidLoopCounter, at));
                }
                if beat.kind == BeatKind::Chorus && counter == 0 {
                    pc = pairs[pc] + 1;
                    continue;
                }
                if beat.kind == BeatKind::Finale && counter > 0 {
                    pc = pairs[pc] + 1;
                    continue;
                }
            }
        }
        pc += 1;
    }
    Ok(output)
}

/// Parse and execute with default resource limits.
pub fn run(source: &str) -> Result<String, Error> {
    execute(&parse(source)?, Limits::default())
}
