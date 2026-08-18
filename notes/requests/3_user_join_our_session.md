# 3 - User join our session

## server → client releaseLock [ephemeral] 5:00:32 PM

```json
{
 "type": "releaseLock",
 "locks": {}
}
```

## server → client cursor [ephemeral] 5:00:34 PM

```json
{
 "type": "cursor",
 "messages": [
  {
   "cursor": "16;CAAAADAAXwAyADcAAAAAAA==",
   "time": 1786978834157,
   "user": "773d9bcc-1b97-4032-9087-39f88407cf62",
   "useridoriginal": "a11cec00100000000000000000000000"
  }
 ]
}
```

## server → client connectState [ephemeral] 5:00:34 PM

```json
{
 "type": "connectState",
 "participantsTimestamp": 1786978834768,
 "participants": [
  {
   "id": "773d9bcc-1b97-4032-9087-39f88407cf62",
   "idOriginal": "a11cec00100000000000000000000000",
   "username": "Alicey McAliceFace",
   "indexUser": 0,
   "view": false
  },
  {
   "id": "f3e4ef5d-16f4-4526-a477-f0437894de55",
   "idOriginal": "a11cec00100000000000000000000000",
   "username": "Alicey McAliceFace",
   "indexUser": 1,
   "view": false
  }
 ],
 "waitAuth": false
}
```

## client → server unLockDocument [ephemeral] 5:00:36 PM

```json
{
 "type": "unLockDocument",
 "isSave": false,
 "unlock": true,
 "deleteIndex": null
}
```

## server → client releaseLock [ephemeral] 5:00:36 PM

```json
{
 "type": "releaseLock",
 "locks": {}
}
```

## client → server cursor [ephemeral] 5:00:36 PM

```json
{
 "type": "cursor",
 "cursor": "16;CAAAADAAXwAyADcAAAAAAA=="
}
```
