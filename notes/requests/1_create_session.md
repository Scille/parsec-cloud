# 1 - Create session

## server → client connectState [ephemeral] 4:58:14 PM

```json
{
 "type": "connectState",
 "participantsTimestamp": 1786978694106,
 "participants": [
  {
   "id": "f3e4ef5d-16f4-4526-a477-f0437894de55",
   "idOriginal": "a11cec00100000000000000000000000",
   "username": "Alicey McAliceFace",
   "indexUser": 0,
   "view": false
  }
 ],
 "waitAuth": false
}
```

## client → server auth [ephemeral] 4:58:14 PM

```json
{
 "type": "auth",
 "docid": "609c84f5-3106-4070-ad27-a875d0c4fdbc",
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
  "id": "609c84f5-3106-4070-ad27-a875d0c4fdbc",
  "userid": "a11cec00100000000000000000000000",
  "format": "docx",
  "url": "blob:http://localhost:8083/1f801cf2-265c-4ebe-8348-8b1523510bc0",
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
 "time": 518,
 "supportAuthChangesAck": true
}
```

## client → server authChangesAck [ephemeral] 4:58:14 PM

```json
{
 "type": "authChangesAck"
}
```

## client → server clientLog [ephemeral] 4:58:14 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onDownloadFile time:11"
}
```

## client → server clientLog [ephemeral] 4:58:15 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onOpenDocument time:110"
}
```

## client → server clientLog [ephemeral] 4:58:15 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onLoadFonts time:104"
}
```

## client → server clientLog [ephemeral] 4:58:15 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onApplyChanges time:15"
}
```

## client → server clientLog [ephemeral] 4:58:15 PM

```json
{
 "type": "clientLog",
 "level": "debug",
 "msg": "onDocumentContentReady time:1773"
}
```
