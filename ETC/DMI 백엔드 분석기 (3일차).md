# DMI 백엔드 분석기 (3일차)

백엔드 환경 : Node.js(Javascript), Express(프레임워크)

진입점 : app.js

의존성
async, express, socket.io, xlsx, sequelize

---

## app.js

웹서버, 채팅 서버 런치, 오류 및 기본 라우터들 관리

### 기본 라우터

| 라우터                   | 파일 위치           |
| ------------------------ | ------------------- |
| [/](#routes/index.js)    | ./routes            |
| [/auth](#routes/auth.js) | ./routes/auth       |
| /attendance              | ./routes/attendance |
| /student                 | ./routes/student    |
| /absence                 | ./routes/absence    |
| /club                    | ./routes/club       |
| /class                   | ./routes/class      |
| /chat                    | ./routes/chat       |

### 디렉토리

| 이름        | 역할                                                         |
| ----------- | ------------------------------------------------------------ |
| documents   | 날짜에 따른 활동, 담당선생님, 동아리 위치, 학생 정보 등 처음 선생님으로 부터 엑셀 파일로 넘겨받고 사용할 파일들이 위치한다 |
| image       | 로고, 개발자 사진, 기타 서비스에서 사용할 사진들이 위치한다  |
| middlewares | jwt 확인 등을 시행하는 미들웨어의 구현이 위치한다            |
| models      | ORM을 위한 구현들이 위치한다                                 |
| routes      | API를 제공하기 위한 비지니스 로직이 위치한다                 |
| utils       | jwt 관리, 날짜 변환 등의 구현이 위치한다                     |



---

## routes/index.js

'/'의 라우터

### GET activity

주어진 날짜의 활동을 반환한다

#### req

query : year, month, day

#### res

0, 1, 2, 3 중 하나 (??????????)

#### 동작 방식

xlsx로 `afterSchoolSchedule.xlsx`을 읽어 json으로 변경후 루프 돌아서 주어진 날짜와 같은 날짜의 활동을 반환



### Patch activity

주어진 날짜의 활동을 주어진 활동으로 바꿉니다

#### req

query : year, month, day

body : { "activity" : 활동 종류 }

#### 동작 방식

xlsx로 `afterSchoolSchedule.xlsx`을 읽어 json으로 변경후 루프 돌아서 주어진 날짜와 같은 날짜의 활동을 변경 후 객체를 xlsx를 활용해서 파일 변경



### Patch activity-each

주어진 두 날짜의 활동을 교환합니다

#### req

body : {"date1" : [연도, 월, 일], "date2" : [연도, 월, 일] }

#### 동작 방식

xlsx로 `afterSchoolSchedule.xlsx`을 읽어 json으로 변경한 후 주어진 두 날짜가 같은지 확인한다. 다르다면 루프를 돌아서 각 날자의 인덱스를 가져온 후 두 활동을 스왑



### Get teachers

모든 선생님들을 반환한다

#### res

{teahcers : [선생님들]}

#### 동작 방식

Sequelize라는 ORM 라이브러리를 활용해 DB의 teacher 테이블로 부터 객체화된 데이터들을 가져온 후 반환



### Get teachers/specific

주어진 날짜의 담당 선생님 세명을 반환합니다

#### req

query : year, month, day

#### res

{date: 날짜, f2: 2층선생님, f3: 3층 선생님, f4: 4층 선생님}

#### 동작 방식

xlsx를 이용해 `managerSchedule.xlsx`을 json으로 읽은 후 생성된 배열을 순회하며 주어진 날짜와 동일한 날짜를 가진 인덱스를 찾는다. 해당 인덱스의 객체 반환



### Patch teachers

주어진 날짜의 선생님들을 주어진 선생님들로 바꾼다.

#### req

query: year, month, day

body: {f2: 2층선생님, f3: 3층 선생님, f4: 4층 선생님}

#### 동작 방식

xlsx를 이용해 `managerSchedule.xlsx`을 json으로 읽은 후 생성된 배열을 순회하며 주어진 날짜와 동일한 날짜를 가진 인덱스를 찾고 선생님을 바꿈



### Patch teacher-each

주어진 두 날짜와 두 층(담당선생님 위치)로 두 선생님을 교환합니다

#### req

body: {date1: [년,월,일], date2: [년,월,일], selected1:?, selected2:?, floor1: 층수, floor2: 층수   }

*selected1, selected1는 본 내용에서 직접적으로 사용하지 않아 잘 모르겠다*

#### 동작 방식

xlsx를 이용해 `managerSchedule.xlsx`을 json으로 읽은 후 생성된 배열을 순회하며 주어진 날짜와 동일한 날짜를 가진 인덱스를 찾고 각 날짜의 선생님 교환



### Get create/class

DB에 클래스를 생성합니다

#### 동작 방식

ORM을 이용해 DB의 class테이블에 1~3학년 각 학년 당 1~4반까지 생성 (필드 : 학년, 반)



### Get create/students

DB에 학생을 생성합니다

#### 동작 방식

`students.xlsx`을 json으로 읽은 후 생성된 배열을 순회하며 나온 학생 데이터에서 학년과 반 정보로 맞는 클래스 ORM 객체를 얻은 후 적절한 데이터로 레코드를 생성(이름, 번호, 반, 동아리(현재는 랜덤인듯))



### Get create/teachers

DB에 선생님들을 생성합니다

#### 동작 방식

`teachers.xlsx`을 json으로 읽은 후 순회하며 레코드 생성



### Get create/clubs

DB에 동아리를 생성합니다

#### 동작 방식

`club.xlsx`을 json으로 읽은 후 순회하며 레코드 생성



### 느낀점

대체적으로 구조가 깔끔해서 읽으면 기능과 어떻게 동작할지 보인다
express프레임워크의 구조와 ORM과 DB 설계에대해 공부하게 되었다

xlsx파일 읽는 코드, 날짜 비교하는 코드가 길고 자주 반복된다



## 