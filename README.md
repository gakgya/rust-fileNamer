# FileNamer – 파일명 일괄 변환 도구

## 소개
Rust로 제작한 파일명 정리/변환 CLI 도구입니다.
여러 옵션을 조합해 원하는 형태로 파일명을 일괄 변환할 수 있으며,
--dry-run 옵션을 통해 실제 변경 전에 결과를 미리 확인할 수 있습니다.

파일명 충돌이 발생하는 경우 자동으로 번호를 붙여 안전하게 처리됩니다.
(example.png → example(1).png → example(2).png …)

## 기능
### 기본 실행
```
<directory> <option>
```

## 주요 옵션

| 옵션                      | 설명                   | 예시                  |
| ----------------------- | -------------------- | ------------------- |
| `--prefix <text>`       | 파일명 앞에 텍스트 추가        | `--prefix renamed_` |
| `--suffix <text>`       | 파일명 뒤에 텍스트 추가        | `--suffix _done`    |
| `--lowercase`           | 파일명을 모두 소문자로 변환      | `--lowercase`       |
| `--uppercase`           | 파일명을 모두 대문자로 변환      | `--uppercase`       |
| `--replace <from> <to>` | 파일명 내 특정 문자열 치환      | `--replace " " "_"` |
| `--remove-ext`          | 확장자 제거               | `--remove-ext`      |
| `--recursive`           | 하위 디렉터리 포함 전체 처리     | `--recursive`       |
| `--dry-run`             | 실제 변경 없이 변경될 결과 미리보기 | `--dry-run`         |

### 파일명 충돌 자동 해결
기존 이름과 충돌하면 자동으로 번호를 붙여 안전하게 변경됩니다.
```
photo.png → photo(1).png → photo(2).png …
```

### dry-run 결과 확인 후 실제 적용 여부 선택
```
--dry-run 실행 → 결과 출력
→ y 또는 yes 입력 시 실제 변경
→ n 또는 no 입력 시 취소
```

## 실행방법
### exe 파일 더블클릭 실행
빌드 후 생성되는 파일:
```
target/release/fileNamer.exe
```
이 파일을 더블클릭하면 인터랙티브 모드로 실행됩니다.
예시(프로그램에 입력):
```
C:\images --lowercase --prefix sample_
```
### CMD 또는 PowerShell에서 직접 실행
```
fileNamer.exe <폴더경로> [옵션들...]
```
예시
```
fileNamer.exe test --prefix new_ --lowercase --dry-run
```

## 테스트 파일 자동 생성 (BAT 스크립트)

init_test_files.bat 실행 시 아래 작업을 수행합니다:
- test 폴더가 없으면 생성
- 기존 test 폴더 내용 삭제 (초기화)
- 테스트용 파일 자동 생성

---
## 테스트 명령어 예시
### Dry-run 미리보기
```
fileNamer.exe test --lowercase --prefix sample_ --dry-run
```
### 실제 적용 (dry-run 실행 후 y 입력)
```
Apply changes? (y/n): y

```
### replace 옵션 테스트
```
fileNamer.exe test --replace " " "_" --prefix new_ --dry-run

```
### 하위 폴더 포함 전체 처리
```
fileNamer.exe test --recursive --lowercase --dry-run

```

### 빌드방법
```
cargo build --release
```
실행 파일 위치:
```
target/release/fileNamer.exe
```
### 주의사항
- Windows CMD, PowerShell 지원
- 파일명 충돌 시 자동 번호 추가
- dry-run 모드에서는 실제 파일이 변경되지 않음
- exe 더블클릭 실행 시 입력 대기 상태(인터랙티브 모드) 진입