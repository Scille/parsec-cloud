## Info

- Timestamp: 2026-10-14T09:38:21.031Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice (edit) connects and edits the document.
  2. Bob (view) joins mid-edit: he receives the backlog of changes as
     authChanges (+ authChangesAck, live viewer) before his auth response.
  3. Alice edits again and her save changes are relayed to him.

## Events

### 09:38:08.201       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv7x2vr9f4zi/c/?shardkey=recmv7x2vr9f4zi&EIO=4&transport=websocket"
```

### 09:38:08.203   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "zVo9EgT85LB1ztVnAAAu",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:08.211   <-  Alice           license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.3.0",
    "buildNumber": 138,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": false,
    "customization": false,
    "advancedApi": false
  }
}
```

### 09:38:08.385   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x2vr9f4zi",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7x2vr9f4zi",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33303/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "edit",
  "permissions": {
    "edit": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 481,
  "supportAuthChangesAck": true
}
```

### 09:38:08.403   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "-igMZm46nqlTHDBvAAAv",
  "sessionTimeConnect": 1791970688210,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-igMZm46nqlTHDBvAAAv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MnZyOWY0emkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDY4ODIxMH0sImlhdCI6MTc5MTk3MDY4OCwiZXhwIjoxNzk0NTYyNjg4fQ.d1PEOb11eCjkW8chHXiVPHl3-8PqKxv9RJ2k1VX3hKg",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.3.0",
  "buildNumber": 138,
  "licenseType": 3,
  "settings": {
    "spellcheckerUrl": "",
    "reconnection": {
      "attempts": 50,
      "delay": 2000
    },
    "binaryChanges": false,
    "websocketMaxPayloadSize": 1572864,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1791977888393
}
```

### 09:38:08.467   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x2vr9f4zi/Editor.bin/Editor.bin?md5=9Ld3J9rINDobuSMZvEmsFw&expires=1794565281&shardkey=recmv7x2vr9f4zi&filename=Editor.bin"
    },
    "openedAt": 1791977888393
  }
}
```

### 09:38:08.548   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 09:38:08.595   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:45"
}
```

### 09:38:08.644   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 09:38:08.787   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:08.787   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:879 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:08.787   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 09:38:08.788   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:38:08.823   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 09:38:09.846   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 09:38:10.229   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "13e1835c-f2b8-c86b-137b-9e788996778b",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 09:38:10.230   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "13e1835c-f2b8-c86b-137b-9e788996778b": {
      "time": 1791970690230,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "13e1835c-f2b8-c86b-137b-9e788996778b",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 09:38:10.242   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 09:38:11.646   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:38:11.647   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:38:11.655   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\",\"14;CgAAAAE1AAAA/wAAAAA=\",\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": false
}
```

### 09:38:11.662   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970691000,
  "syncChangesIndex": 3
}
```

### 09:38:12.908       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv7x2vr9f4zi/c/?shardkey=recmv7x2vr9f4zi&EIO=4&transport=websocket"
```

### 09:38:12.910   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "doYRG7hFQdMBWTS4AAAw",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:12.941   <-  Bob             license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.3.0",
    "buildNumber": 138,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": false,
    "customization": false,
    "advancedApi": false
  }
}
```

### 09:38:12.995   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x2vr9f4zi",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7x2vr9f4zi",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33303/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "view",
  "permissions": {
    "edit": false
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 698,
  "supportAuthChangesAck": true
}
```

### 09:38:13.007   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmv7x2vr9f4zi",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1791970691000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x2vr9f4zi",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791970691000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x2vr9f4zi",
      "change": "\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791970691000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:38:13.009   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 09:38:13.108   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ESsOA5KaiKsdBxHsAAAx",
  "sessionTimeConnect": 1791970692940,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-igMZm46nqlTHDBvAAAv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "ESsOA5KaiKsdBxHsAAAx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [
    {
      "time": 1791970690230,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "13e1835c-f2b8-c86b-137b-9e788996778b",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  ],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MnZyOWY0emkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDY5Mjk0MH0sImlhdCI6MTc5MTk3MDY5MywiZXhwIjoxNzk0NTYyNjkzfQ.ipnksosHGetVGpCM4z6AgMLWFuFogbUTylZgktrjIy4",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.3.0",
  "buildNumber": 138,
  "licenseType": 3,
  "settings": {
    "spellcheckerUrl": "",
    "reconnection": {
      "attempts": 50,
      "delay": 2000
    },
    "binaryChanges": false,
    "websocketMaxPayloadSize": 1572864,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1791977888393
}
```

### 09:38:13.108   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970693002,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-igMZm46nqlTHDBvAAAv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "ESsOA5KaiKsdBxHsAAAx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:38:13.109   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x2vr9f4zi/Editor.bin/Editor.bin?md5=9Ld3J9rINDobuSMZvEmsFw&expires=1794565281&shardkey=recmv7x2vr9f4zi&filename=Editor.bin"
    },
    "openedAt": 1791977888393
  }
}
```

### 09:38:13.117   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 09:38:13.141   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:24"
}
```

### 09:38:13.192   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 09:38:13.250   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:0"
}
```

### 09:38:13.250   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:6"
}
```

### 09:38:13.250   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:13.250   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:949 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:13.250   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 09:38:13.251   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:38:14.214   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 09:38:14.214   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791970694214,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:38:14.639   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "d1e7ec79-04b9-8ec4-412a-f6ee5e491617",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 3,
        "c2": 3,
        "r2": 3
      }
    }
  ]
}
```

### 09:38:14.640   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "13e1835c-f2b8-c86b-137b-9e788996778b": {
      "time": 1791970690230,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "13e1835c-f2b8-c86b-137b-9e788996778b",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    },
    "d1e7ec79-04b9-8ec4-412a-f6ee5e491617": {
      "time": 1791970694639,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d1e7ec79-04b9-8ec4-412a-f6ee5e491617",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 09:38:14.646   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 09:38:14.646   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791970694646,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:38:16.166   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 09:38:16.166   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:38:16.172   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"105;ZQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABFAAAAAAIDAQIDAgkDOAAAAAAAAQkBJQAAAAAIGAAAAEEAbABpAGMAZQA6ACAASABlAGwAbABvAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": true,
  "deleteIndex": 3,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": false
}
```

### 09:38:16.179   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x2vr9f4zi",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791970696000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x2vr9f4zi",
      "change": "\"105;ZQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABFAAAAAAIDAQIDAgkDOAAAAAAAAQkBJQAAAAAIGAAAAEEAbABpAGMAZQA6ACAASABlAGwAbABvAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791970696000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": null
}
```

### 09:38:16.179   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1791970696000,
  "syncChangesIndex": 5
}
```
