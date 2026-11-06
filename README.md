# 맘보랭 · Mambo Lang

**맘보랭**은 독립적으로 설계한 오픈 소스 리듬 기반 스택 언어입니다. 소스는 `|`로 마디를 나누고, 한 마디에는 최대 네 개의 명령 박자가 들어갑니다. 공백과 줄바꿈은 단어와 박자를 구분하며, 명령어와 정확히 일치하지 않는 단어는 주석처럼 무시됩니다. 빈 마디는 쉼표처럼 조용히 지나갑니다.

이 프로젝트는 비공식 팬 창작물입니다. Hachimi Lang 또는 다른 프로젝트의 이름 변경, 포크, 토큰 치환물이 아니며 어떤 권리자·단체와도 제휴하거나 승인을 받았다고 주장하지 않습니다. 저작권 보호 대상인 음원, 가사, 로고, 이미지 등의 자산은 포함하지 않습니다.

## 시작하기

Rust 2021을 지원하는 도구 체인으로 빌드합니다. 실행 시 외부 크레이트에 의존하지 않습니다.

```sh
cargo run -- run examples/mambo.mambo
# MAMBO
cargo run -- check examples/mambo.mambo
cargo run -- eval '둠 짝 plus 솔로'
cargo run -- help
cargo run -- version
```

`mambo run <file>`은 UTF-8 파일을 실행하고, `mambo eval <source>`는 인자로 받은 소스를 실행합니다. `mambo check <file>`은 실행 없이 문법과 루프 구조를 검사합니다. 출력에는 자동 줄바꿈이 없습니다. 오류는 `줄:열: 메시지` 형식으로 표준 오류에 표시되며 종료 상태는 실패입니다.

## 언어 규칙

`|`가 마디를 끝냅니다. 각 마디에는 명령어가 0~4개 들어갑니다. 명령어는 공백이나 `|`로 구분된 **정확한 단어**여야 합니다. 따라서 `앞둠`은 주석이지만 `둠|짝`은 두 마디에 걸친 두 명령어입니다. 줄바꿈도 공백처럼 다음 박자를 시작하지만 마디는 끝내지 않습니다. 주석 단어는 박자를 차지하지 않습니다. 다섯 번째 명령어는 오류입니다.

| 명령어 | 효과 |
| --- | --- |
| `둠` | 1을 푸시 |
| `짝` | 2를 푸시 |
| `쉼` | 0을 푸시 |
| `plus` (`플러스` 별칭) | 위의 두 값을 꺼내 아래 값 + 위 값 푸시 |
| `마이너스` | 위의 두 값을 꺼내 아래 값 − 위 값 푸시 |
| `스텝` | 맨 위 값을 복제 |
| `턴` | 위의 두 값을 교환 |
| `솔로` | 맨 위 값을 꺼내 유니코드 스칼라로 출력 |
| `코러스` | 루프 시작; 단독 마디에만 배치 |
| `피날레` | 루프 끝; 단독 마디에만 배치 |

스택 값은 부호 있는 64비트 정수입니다. 산술 오버플로, 빈 스택 접근, 유효하지 않은 유니코드 스칼라 출력은 위치가 표시된 오류입니다. `코러스`에서 스택 맨 위가 0이면 대응하는 `피날레` 다음으로 건너뜁니다. 양수면 본문으로 들어갑니다. `피날레`에서도 맨 위가 양수면 본문 처음으로 돌아가고, 0이면 루프를 벗어납니다. 카운터는 자동으로 줄지 않으며 음수는 오류입니다. 두 루프 표시는 서로 짝을 이루어야 하고 각각 자기 마디에 혼자 있어야 합니다.

```text
짝 | 코러스 | 둠 마이너스 | 피날레 | 둠 솔로
```

이 예제는 카운터 2를 0까지 줄인 뒤 유니코드 값 1을 출력합니다. [MAMBO 출력 예제](examples/mambo.mambo)는 복제와 덧셈으로 각 문자의 코드 포인트를 만듭니다.

기본 제한은 실행 박자 100,000개, 스택 값 1,024개, 루프 중첩 64단계입니다. 루프 표시는 실행될 때마다 한 박자로 계산합니다. 라이브러리 사용자는 `Limits`로 제한을 조정할 수 있습니다.

## 라이브러리

```rust
use mambo_lang::{parse, execute, Limits};

let program = parse("둠 짝 plus 솔로")?;
let output = execute(&program, Limits::default())?;
assert_eq!(output, "\u{3}");
# Ok::<(), mambo_lang::Error>(())
```

`parse`, `execute`, `run`을 제공하며, `ErrorKind`와 `Position`으로 오류를 구조적으로 확인할 수 있습니다. 위치의 줄과 열은 1부터 시작하며 유니코드 스칼라 단위입니다.

## 개발과 참여

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

[기여 가이드](CONTRIBUTING.md), [보안 정책](SECURITY.md), [행동 강령](CODE_OF_CONDUCT.md), [변경 기록](CHANGELOG.md), [MIT 라이선스](LICENSE)를 참고하세요.

---

## English

**Mambo Lang** is an independently designed, open-source rhythm and bar based stack language. `|` closes a bar; each bar holds at most four instruction beats. Whitespace, including newlines, separates beats without ending a bar. Exact token words execute; all other words are comments and use no beats. Empty bars are silent.

Use `mambo run <file>`, `mambo eval <source>`, or `mambo check <file>` after `cargo build`. The token table and loop example above define the language; `plus` also accepts `플러스`. `솔로` pops and emits one Unicode scalar, with no automatic newline. Arithmetic is checked. Errors include one-based Unicode line and column positions. The default limits are 100,000 executed beats, 1,024 stack values, and 64 nested loops.

This is an unofficial fan project with no affiliation or endorsement. It is an original language, not a rename, fork, or token swap of Hachimi Lang. It includes no copyrighted audio, lyrics, logos, or imagery.

The package is MIT licensed and has no runtime dependencies. See the linked project documents above for contributions, security reports, conduct, and release notes.
