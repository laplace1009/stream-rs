# stream-rs

`stream-rs`는 Rust를 사용하여 구현된 실시간 비디오 스트리밍 서버입니다. 이 서버는 YouTube와 Twitch 같은 플랫폼에서 요구하는 다양한 스트리밍 기능을 지원합니다.

## 기능

- **실시간 비디오 스트리밍**: 사용자는 실시간으로 비디오를 스트리밍할 수 있습니다.
- **고성능**: Rust와 비동기 프로그래밍 라이브러리 `tokio`를 사용하여 고성능을 보장합니다.
- **경량 서버**: `axum` 웹 프레임워크를 기반으로 하여 경량이면서도 강력한 API 서버 기능을 제공합니다.

## 시작하기

이 프로젝트를 사용하기 위해서는 Rust 개발 환경이 설정되어 있어야 합니다.

### 필요 조건

- Rust
- Cargo

### 설치 방법

1. 레포지토리 클론하기:
   ```bash
   git clone https://github.com/your-username/stream-rs.git
   ```
2. 의존성 설치
   ```bash
   cd stream-rs
   cargo build
   ```
3. 서버 실행하기
   ```bash
   cargo run
   ```
### 기술스택
* 언어: Rust
* 주요 라이브러리:
  + `tokio`: 비동기 런타임
  + `axum`: 경량 웹 프레임워크
  + `serde`: 직렬화 역직렬화

### 라이선스
* MIT
