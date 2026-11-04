## Info

- Timestamp: 2026-10-16T20:06:46.650Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice locks cell B3 by typing in it (alone, so the block stays held).
  2. Bob joins mid-edit: his auth carries the lock table, and Alice's switch to
     co-editing mode releases her block (`unLockDocument{releaseLocks:true}` →
     a `releaseLock` to everyone, herself included).
  3. Alice re-takes B3, and her collaborative save releases it again.
  4. Bob selects A3 and inserts a column: he takes the structural lock on the
     whole column (`subType: 2`), and his `saveChanges` carries a non-null
     `excelAdditionalInfo` with the `indexCols` lock-shift info (no cursor part),
     relayed as-is to Alice.
  5. Alice's follow-up save carries the same blob with the save-index marker
     (empty changes: it re-syncs the change index space after applying the
     structural change).
  6. Bob locks and edits a cell in the shifted geometry: his `saveChanges`
     carries no `excelAdditionalInfo` and releases his block.
  Note: every lock is released before the insertion, so nothing pins the
  server-side lock recalculation (the `indexCols` info is pure pass-through).

## Events

### 20:06:29.544       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvbeenkndesq/c/?shardkey=recmvbeenkndesq&EIO=4&transport=websocket"
```

### 20:06:29.547   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "G1aktW38_G7gnzK0AAAS",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 20:06:29.599   <-  Alice           license

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

### 20:06:29.741   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvbeenkndesq",
  "documentCallbackUrl": "http://host.containers.internal:41175/callback",
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
    "id": "recmvbeenkndesq",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:41175/spreadsheet.xlsx",
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
  "time": 528,
  "supportAuthChangesAck": true
}
```

### 20:06:29.757   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "7neM3cPulhnCzeNdAAAT",
  "sessionTimeConnect": 1792181189598,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "7neM3cPulhnCzeNdAAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmJlZW5rbmRlc3EiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjE4MTE4OTU5OH0sImlhdCI6MTc5MjE4MTE4OSwiZXhwIjoxNzk0NzczMTg5fQ.VOitckdPR1aa-WFFWR9IPGmo7PBnqs77Wb4KS8poRSo",
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
  "openedAt": 1792188389746
}
```

### 20:06:29.817   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvbeenkndesq/Editor.bin/Editor.bin?md5=olpZ1gmiWragwFfDL4I1VQ&expires=1794775782&shardkey=recmvbeenkndesq&filename=Editor.bin"
    },
    "openedAt": 1792188389746
  }
}
```

### 20:06:29.851   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 20:06:29.896   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:44"
}
```

### 20:06:29.945   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 20:06:30.106   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 20:06:30.106   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:889 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":68000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 20:06:30.106   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 20:06:30.106   <-  Alice           message

```json
{
  "type": "message"
}
```

### 20:06:30.151   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 20:06:31.096   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "aca7ab7a-183a-9941-1161-8508f59f074e",
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

### 20:06:31.097   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "aca7ab7a-183a-9941-1161-8508f59f074e": {
      "time": 1792181191097,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "aca7ab7a-183a-9941-1161-8508f59f074e",
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

### 20:06:31.107   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 20:06:32.715   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 20:06:32.716   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:32.719   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\",\"14;CgAAAAE1AAAA/wAAAAA=\",\"109;aQAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABJAAAAAAICAQIBAgkDPAAAAAAAAQkBKQAAAAAIHAAAAEEAbABpAGMAZQAgAGwAbwBjAGsAcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"]",
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

### 20:06:32.724   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792181192000,
  "syncChangesIndex": 3
}
```

### 20:06:33.859       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvbeenkndesq/c/?shardkey=recmvbeenkndesq&EIO=4&transport=websocket"
```

### 20:06:33.860   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "jyLfIx_F8AuY9f40AAAU",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 20:06:33.888   <-  Bob             license

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

### 20:06:33.989   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvbeenkndesq",
  "documentCallbackUrl": "http://host.containers.internal:41175/callback",
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
    "id": "recmvbeenkndesq",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:41175/spreadsheet.xlsx",
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
  "time": 316,
  "supportAuthChangesAck": true
}
```

### 20:06:33.995   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "7neM3cPulhnCzeNdAAAT",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 20:06:33.995   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792181193993,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "7neM3cPulhnCzeNdAAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "HOT-9_iIO4rx-lg1AAAV",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 20:06:33.996   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvbeenkndesq/Editor.bin/Editor.bin?md5=olpZ1gmiWragwFfDL4I1VQ&expires=1794775782&shardkey=recmvbeenkndesq&filename=Editor.bin"
    },
    "openedAt": 1792188389746
  }
}
```

### 20:06:34.001   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": 3
}
```

### 20:06:34.009   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmvbeenkndesq",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1792181192000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvbeenkndesq",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792181192000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvbeenkndesq",
      "change": "\"109;aQAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABJAAAAAAICAQIBAgkDPAAAAAAAAQkBKQAAAAAIHAAAAEEAbABpAGMAZQAgAGwAbwBjAGsAcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1792181192000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:34.012   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 20:06:34.012   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 20:06:34.110   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "HOT-9_iIO4rx-lg1AAAV",
  "sessionTimeConnect": 1792181193887,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "7neM3cPulhnCzeNdAAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "HOT-9_iIO4rx-lg1AAAV",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [
    {
      "time": 1792181191097,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "aca7ab7a-183a-9941-1161-8508f59f074e",
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
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmJlZW5rbmRlc3EiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxODExOTM4ODd9LCJpYXQiOjE3OTIxODExOTQsImV4cCI6MTc5NDc3MzE5NH0.9PJgrmQ37UZIhuWd-ew7Qzs3HDAEipINuMKd7cMlbGs",
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
  }
}
```

### 20:06:34.469   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:26"
}
```

### 20:06:34.476   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 20:06:34.476   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:34.479   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 20:06:34.480   <-  Alice           releaseLock

```json
{
  "type": "releaseLock",
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "aca7ab7a-183a-9941-1161-8508f59f074e",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1792181194480,
      "changes": null
    }
  ]
}
```

### 20:06:34.480   <-  Bob             releaseLock

```json
{
  "type": "releaseLock",
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "aca7ab7a-183a-9941-1161-8508f59f074e",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1792181194480,
      "changes": null
    }
  ]
}
```

### 20:06:34.480   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 20:06:34.515   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:34.516   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181194515,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:34.520   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 20:06:34.622   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:0"
}
```

### 20:06:34.623   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:8"
}
```

### 20:06:34.623   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 20:06:34.623   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:945 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":68000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 20:06:34.623   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 20:06:34.623   <-  Bob             message

```json
{
  "type": "message"
}
```

### 20:06:34.663   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 20:06:34.664   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792181194663,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:35.701   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 20:06:35.702   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792181195701,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:36.703   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "4eb23b01-3353-28b9-1ebb-027e98e0836a",
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

### 20:06:36.703   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "4eb23b01-3353-28b9-1ebb-027e98e0836a": {
      "time": 1792181196703,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "4eb23b01-3353-28b9-1ebb-027e98e0836a",
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

### 20:06:36.703   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "4eb23b01-3353-28b9-1ebb-027e98e0836a": {
      "time": 1792181196703,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "4eb23b01-3353-28b9-1ebb-027e98e0836a",
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

### 20:06:36.709   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 20:06:36.710   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1792181196710,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:36.741   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 20:06:36.741   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792181196741,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:37.359   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 20:06:37.359   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:37.361   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"111;awAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABLAAAAAAICAQIBAgkDPgAAAAAAAQkBKwAAAAAIHgAAAEEAbABpAGMAZQAgAHIAZQBsAGUAYQBzAGUAcwAuAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 20:06:37.368   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvbeenkndesq",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792181197000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvbeenkndesq",
      "change": "\"111;awAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABLAAAAAAICAQIBAgkDPgAAAAAAAQkBKwAAAAAIHgAAAEEAbABpAGMAZQAgAHIAZQBsAGUAYQBzAGUAcwAuAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1792181197000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "4eb23b01-3353-28b9-1ebb-027e98e0836a",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1792181197367,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 20:06:37.368   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1792181197000,
  "syncChangesIndex": 5
}
```

### 20:06:37.387   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 20:06:37.388   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:37.398   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 20:06:37.399   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 20:06:37.715   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:37.716   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181197715,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:37.781   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 20:06:37.782   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792181197781,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:38.821   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 20:06:38.821   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792181198821,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:39.217   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 2,
      "guid": "80d4bac7-8dec-3fb1-6a7b-e18a09460021",
      "rangeOrObjectId": {
        "c1": 0,
        "r1": 0,
        "c2": 0,
        "r2": 1048575
      }
    }
  ]
}
```

### 20:06:39.218   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "80d4bac7-8dec-3fb1-6a7b-e18a09460021": {
      "time": 1792181199218,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 2,
        "guid": "80d4bac7-8dec-3fb1-6a7b-e18a09460021",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 20:06:39.218   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "80d4bac7-8dec-3fb1-6a7b-e18a09460021": {
      "time": 1792181199218,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 2,
        "guid": "80d4bac7-8dec-3fb1-6a7b-e18a09460021",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 20:06:39.232   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 20:06:39.232   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:39.237   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"45;KQAAAAE2BQECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 20:06:39.238   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:39.238   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181199238,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:39.244   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvbeenkndesq",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792181199000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvbeenkndesq",
      "change": "\"45;KQAAAAE2BQECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"",
      "time": 1792181199000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 2,
        "guid": "80d4bac7-8dec-3fb1-6a7b-e18a09460021",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792181199244,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}"
}
```

### 20:06:39.244   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 5,
  "time": 1792181199000,
  "syncChangesIndex": 7
}
```

### 20:06:39.275   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 20:06:39.276   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:39.304   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 20:06:39.305   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}"
}
```

### 20:06:39.305   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 7,
  "time": 1792181199000,
  "syncChangesIndex": 7
}
```

### 20:06:39.861   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 20:06:39.861   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792181199861,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:40.275   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:40.276   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181200275,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:41.292   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 1,
        "c2": 3,
        "r2": 1
      }
    }
  ]
}
```

### 20:06:41.293   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3": {
      "time": 1792181201293,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      }
    }
  }
}
```

### 20:06:41.293   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3": {
      "time": 1792181201293,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      }
    }
  }
}
```

### 20:06:41.300   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMQAsADMALAAxACwA"
}
```

### 20:06:41.301   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMQAsADMALAAxACwA",
      "time": 1792181201301,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:41.321   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:41.322   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181201321,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:42.467   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 20:06:42.468   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:42.469   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"135;gwAAAAE1EAECAAAANgABAwAAAAEAAAADAAAAAQAAAABjAAAAAAIBAQIDAgkDVgAAAAAAAQkBQwAAAAAINgAAAEIAbwBiACAAbABvAGMAawBzACAAdABoAGUAIABzAGgAaQBmAHQAZQBkACAAYwBlAGwAbAAuAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 20:06:42.510   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvbeenkndesq",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792181202000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvbeenkndesq",
      "change": "\"135;gwAAAAE1EAECAAAANgABAwAAAAEAAAADAAAAAQAAAABjAAAAAAIBAQIDAgkDVgAAAAAAAQkBQwAAAAAINgAAAEIAbwBiACAAbABvAGMAawBzACAAdABoAGUAIABzAGgAaQBmAHQAZQBkACAAYwBlAGwAbAAuAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1792181202000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d6d499c9-5cdc-b8ec-8d71-25711f1fd6a3",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792181202510,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 20:06:42.511   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 7,
  "time": 1792181202000,
  "syncChangesIndex": 9
}
```

### 20:06:42.519   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:42.519   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 20:06:42.520   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181202520,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 20:06:42.520   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 20:06:42.530   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 20:06:42.531   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 20:06:43.461   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 20:06:43.461   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1792181203461,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 20:06:43.555   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 20:06:43.556   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792181203555,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```
