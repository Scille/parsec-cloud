# 4 - Move cursor

## 4.1 - I move cursor

### client → server cursor [ephemeral] 5:06:49 PM

```json
{
 "type": "cursor",
 "cursor": "16;CAAAADAAXwAyADEAAgAAAA=="
}
```

### client → server cursor [ephemeral] 5:06:50 PM

```json
{
 "type": "cursor",
 "cursor": "16;CAAAADAAXwAyADEAAgAAAA=="
}
```

## 4.2 - Other user move cursor

### server → client cursor [ephemeral] 5:06:49 PM

```json
{
 "type": "cursor",
 "messages": [
  {
   "cursor": "16;CAAAADAAXwAyADEAAgAAAA==",
   "time": 1786979209255,
   "user": "f3e4ef5d-16f4-4526-a477-f0437894de55",
   "useridoriginal": "a11cec00100000000000000000000000"
  }
 ]
}
```

### server → client cursor [ephemeral] 5:06:50 PM

```json
{
 "type": "cursor",
 "messages": [
  {
   "cursor": "16;CAAAADAAXwAyADEAAgAAAA==",
   "time": 1786979210273,
   "user": "f3e4ef5d-16f4-4526-a477-f0437894de55",
   "useridoriginal": "a11cec00100000000000000000000000"
  }
 ]
}
```
