# FileNamer (Rust) - 파일 이름 관리 CLI 도구

## 소개
FileNamer는 Rust로 개발된 파일 이름 일괄 변경 도구입니다.  
지정한 디렉토리 안의 파일명을 특정 규칙에 따라 변환하여 정리할 수 있는 CLI 기반 유틸리티입니다.  
Rust의 안전성, 성능, 간결한 배포 방식을 활용하여 실제 환경에서도 사용할 수 있는 파일명 관리 도구를 만드는 것을 목표로 합니다.

## 기능
- 지정한 디렉토리의 파일명을 일괄 변환
- 공백을 특정 문자로 변환 (예: "_" 또는 "-")
- 파일명 앞에 prefix 추가
- 파일명 끝(확장자 앞)에 suffix 추가
- 파일명을 소문자 또는 대문자로 일괄 변환
- dry-run 기능을 통해 실제 변경 없이 미리보기 제공
- 파일명 충돌 발생 시 자동으로 번호를 붙여 충돌 방지

## 사용방법

### 기본 실행
```
filenamer --dir ./documents
```

### 공백을 언더바로 변환
```
filenamer --dir ./docs --replace-space "_"
```

### prefix 추가
```
filenamer --dir ./music --prefix track_
```

### 파일명을 소문자로 일괄 변환
```
filenamer --dir ./images --lowercase
```

### dry-run 사용 (변경 미적용)
```
filenamer --dir ./videos --dry-run
```

### 여러 옵션 조합
```
filenamer --dir ./files --replace-space "_" --lowercase --prefix new_
```

