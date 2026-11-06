use mambo_lang::{execute, parse, run, ErrorKind, Limits, Position};

#[test]
fn bars_are_four_beats_and_prose_is_ignored() {
    let program = parse("시작 둠 짝 플러스 쉼 | quiet words\n둠").unwrap();
    assert_eq!(program.bars().len(), 2);
    assert_eq!(program.bars()[0].len(), 4);
    assert_eq!(program.bars()[1].len(), 1);
    assert_eq!(run("둠 짝 플러스 솔로").unwrap(), "\u{3}");
}

#[test]
fn token_boundaries_are_exact_and_pipes_need_no_spaces() {
    let program = parse("앞둠 뒤짝|둠|짝").unwrap();
    assert_eq!(
        program.bars().iter().map(Vec::len).collect::<Vec<_>>(),
        vec![0, 1, 1]
    );
}

#[test]
fn parser_reports_unicode_columns_and_bar_overflow() {
    let err = parse("메모 둠 짝 쉼 둠 짝").unwrap_err();
    assert_eq!(err.kind(), ErrorKind::TooManyBeats);
    assert_eq!(err.position(), Position::new(1, 12));
}

#[test]
fn loops_must_be_alone_in_their_bar_and_balanced() {
    assert_eq!(
        parse("둠 코러스").unwrap_err().kind(),
        ErrorKind::LoopNeedsOwnBar
    );
    assert_eq!(
        parse("피날레").unwrap_err().kind(),
        ErrorKind::UnmatchedFinale
    );
    assert_eq!(
        parse("코러스").unwrap_err().kind(),
        ErrorKind::UnclosedChorus
    );
}

#[test]
fn arithmetic_order_and_stack_moves() {
    assert_eq!(run("짝 둠 마이너스 솔로").unwrap(), "\u{1}");
    assert_eq!(run("둠 스텝 플러스 솔로").unwrap(), "\u{2}");
    assert_eq!(run("둠 짝 턴 마이너스 | 솔로").unwrap(), "\u{1}");
}

#[test]
fn output_is_popped_and_unicode_scalar_is_checked() {
    assert_eq!(
        run("짝 솔로 솔로").unwrap_err().kind(),
        ErrorKind::StackUnderflow
    );
    assert_eq!(
        run("둠 짝 마이너스 솔로").unwrap_err().kind(),
        ErrorKind::InvalidScalar
    );
}

#[test]
fn loop_counter_is_tested_at_each_boundary() {
    assert_eq!(
        run("짝 | 코러스 | 둠 마이너스 | 피날레 | 둠 솔로").unwrap(),
        "\u{1}"
    );
    assert_eq!(
        run("쉼 | 코러스 | 둠 솔로 | 피날레 | 둠 솔로").unwrap(),
        "\u{1}"
    );
}

#[test]
fn limits_are_enforced() {
    let tiny = Limits {
        max_steps: 2,
        max_stack: 2,
        max_loop_depth: 1,
    };
    assert_eq!(
        execute(&parse("둠 둠 둠").unwrap(), tiny)
            .unwrap_err()
            .kind(),
        ErrorKind::StepLimit
    );
    assert_eq!(
        execute(
            &parse("둠 둠 둠").unwrap(),
            Limits {
                max_steps: 10,
                ..tiny
            }
        )
        .unwrap_err()
        .kind(),
        ErrorKind::StackLimit
    );
    assert_eq!(
        parse("코러스 | 코러스 | 피날레 | 피날레")
            .unwrap()
            .validate_depth(1)
            .unwrap_err()
            .kind(),
        ErrorKind::LoopDepthLimit
    );
}

#[test]
fn crlf_locations_count_unicode_scalars() {
    let err = run("한글\r\n솔로").unwrap_err();
    assert_eq!(err.position(), Position::new(2, 1));
}

#[test]
fn loop_execution_is_bounded() {
    let program = parse("둠 | 코러스 | 피날레").unwrap();
    let err = execute(
        &program,
        Limits {
            max_steps: 8,
            ..Limits::default()
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::StepLimit);
}

#[test]
fn arithmetic_overflow_is_reported() {
    let mut beats = vec!["둠"];
    for _ in 0..63 {
        beats.extend(["스텝", "plus"]);
    }
    let source = beats
        .chunks(4)
        .map(|bar| bar.join(" "))
        .collect::<Vec<_>>()
        .join(" | ");
    assert_eq!(run(&source).unwrap_err().kind(), ErrorKind::Overflow);
}

#[test]
fn negative_loop_counter_is_rejected() {
    let source = "둠 짝 마이너스 | 코러스 | 피날레";
    assert_eq!(
        run(source).unwrap_err().kind(),
        ErrorKind::InvalidLoopCounter
    );
}

#[test]
fn example_prints_mambo() {
    let source = include_str!("../examples/mambo.mambo");
    assert_eq!(run(source).unwrap(), "MAMBO");
}
