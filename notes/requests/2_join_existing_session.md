# 2 - Join existing session

## server → client connectState [ephemeral] 5:00:32 PM

```json
{
 "type": "connectState",
 "participantsTimestamp": 1786978832743,
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

## client → server auth [ephemeral] 5:00:32 PM

```json
{
 "type": "auth",
 "docid": "017feaf0-e7ab-4f6b-a568-1555e1a1432a",
 "token": "fghhfgsjdgfjs",
 "user": {
  "id": "a11cec00100000000000000000000000",
  "username": "Alicey McAliceFace",
  "firstname": null,
  "lastname": null,
  "indexUser": -1
 },
 "editorType": 0,
 "lastOtherSaveTime": -1,
 "block": [],
 "sessionId": null,
 "sessionTimeConnect": null,
 "sessionTimeIdle": 0,
 "documentFormatSave": 65,
 "isCloseCoAuthoring": false,
 "openCmd": {
  "c": "open",
  "id": "017feaf0-e7ab-4f6b-a568-1555e1a1432a",
  "userid": "a11cec00100000000000000000000000",
  "format": "docx",
  "url": "blob:http://localhost:8083/163e7425-0d30-448b-b20b-11efb30e1be8",
  "title": "New document.docx",
  "nobase64": true,
  "outputformat": 8193,
  "convertToOrigin": ".pdf.xps.oxps.djvu"
 },
 "lang": "en",
 "mode": "edit",
 "permissions": {
  "edit": true,
  "download": false,
  "print": true
 },
 "encrypted": false,
 "IsAnonymousUser": false,
 "timezoneOffset": -120,
 "headingsColor": null,
 "coEditingMode": "fast",
 "time": 475,
 "supportAuthChangesAck": true
}
```

## client → server authChangesAck [ephemeral] 5:00:32 PM

```json
{
 "type": "authChangesAck"
}
```

## client → server unLockDocument [ephemeral] 5:00:32 PM

```json
{
 "type": "unLockDocument",
 "isSave": false,
 "unlock": true
}
```

## server → client releaseLock [ephemeral] 5:00:32 PM

```json
{
 "type": "releaseLock",
 "locks": {}
}
```

## client → server clientLog [ephemeral] 5:00:32 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onDownloadFile time:4"
}
```

## client → server clientLog [ephemeral] 5:00:33 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onOpenDocument time:116"
}
```

## client → server clientLog [ephemeral] 5:00:33 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onLoadFonts time:107"
}
```

## client → server clientLog [ephemeral] 5:00:34 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onApplyChanges time:16"
}
```

## client → server clientLog [ephemeral] 5:00:34 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onDocumentContentReady time:1655"
}
```

## client → server cursor [ephemeral] 5:00:34 PM

```json
{
 "type": "cursor",
 "cursor": "16;CAAAADAAXwAyADcAAAAAAA=="
}
```

## server → client releaseLock [ephemeral] 5:00:36 PM

```json
{
 "type": "releaseLock",
 "locks": {}
}
```

## server → client cursor [ephemeral] 5:00:36 PM

```json
{
 "type": "cursor",
 "messages": [
  {
   "cursor": "16;CAAAADAAXwAyADcAAAAAAA==",
   "time": 1786978836513,
   "user": "f3e4ef5d-16f4-4526-a477-f0437894de55",
   "useridoriginal": "a11cec00100000000000000000000000"
  }
 ]
}
```
