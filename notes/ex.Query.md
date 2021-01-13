# iter vs get

```rs
query.iter().map(|(a, b)| {})

if let Ok((a, b)) = query.get(entity) {}
```


- get(entity) をすると、その entity に紐づくコンポーネントのセットが取得できる
  - 戻り値は Result


