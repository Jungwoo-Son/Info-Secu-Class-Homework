# numpy axis 속성

python의 수학 라이브러리인 numpy에서 몇몇 함수들은 axis 속성을 가지고 있다.

axis는 축을 의미하는데 다차원 배열을 연산할 기준 축을 말한다. numpy에서 이 속성은 0 이상의 정수 값을 가진다. 따라서 최댓값은 n차원 배열에서 n-1임을 알 수 있다.

1차원 배열에서 (가로로 길이를 가진 배열)에서는 axis = 0은 당연히 가로를 기준으로 한다.
2차원 배열에서는 0은 세로, 1은 가로다.
3차원 배열에서는 (오른손 좌표계를 따를 때) 0은 z 축, 1은 y 축, 2는 x축이다.

즉 N차원 배열 A를 A\[a1\]\[a2\]\[a3\]\[a4\] ... [an]에서 a1에 해당하는 차원(축)이 0부터 해서 하나씩 늘어난다.

내가 이러한 점을 모르고 사용하다 보니 몇번 헷갈려 잘못된 결과를 낸적이 있어 정리를 해봤다.

위의 설명을 토대로 아래의 예시를 봐보자.

```python
>>> x = [1,2,3,4,5,] # 1차원 배열
>>> # sum함수는 기본적으로 모든 요소를 더하고 축이 주어지면 그 축을 기준으로 계산한다
>>> np.sum(x, axis=0) 
15
>>> x = [x, x] # 2차원 배열
>>> np.sum(x, axis=0) # 세로 기준
array([2, 4, 6, 8, 10,])
>>> np.sum(x, axis=1) # 가로 기준
array([15, 15])

>>> x = [x, x] # 3차원 배열
>>> np.sum(x, axis=0)
array([[2, 4, 6, 8, 10,],
	   [2, 4, 6, 8, 10,]])
>>> np.sum(x, axis=1)
array([[2, 4, 6, 8, 10,],
	   [2, 4, 6, 8, 10,]])
>>> np.sum(x, axis=2)
array([[15, 15],
	   [15, 15]])
```

