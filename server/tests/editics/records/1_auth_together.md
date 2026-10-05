### 09:10:23.283       Alice      [Transport WebSocket] ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/3944677c-b15f-4402-8b1b-a54d9060057c/c/?shardkey=3944677c-b15f-4402-8b1b-a54d9060057c&EIO=4&transport=websocket"
```

### 09:10:23.616   <-  Alice      [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "jAaWZZCdcO9-8aM6AI6K",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:10:23.780   <-  Alice      license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.4.1",
    "buildNumber": 15,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": true,
    "customization": true,
    "advancedApi": true
  }
}
```

### 09:10:23.879   ->  Alice      auth

```json
{
  "type": "auth",
  "docid": "3944677c-b15f-4402-8b1b-a54d9060057c",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
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
    "id": "3944677c-b15f-4402-8b1b-a54d9060057c",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "https://static.onlyoffice.com/assets/docs/samples/demo.docx",
    "title": "Example Document Title.docx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8193,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "permissions": {
    "edit": true,
    "review": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "headingsColor": null,
  "coEditingMode": "fast",
  "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiIzOTQ0Njc3Yy1iMTVmLTQ0MDItOGIxYi1hNTRkOTA2MDA1N2MiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6Ijc4ZTFlODQxIiwibmFtZSI6IkpvaG4gU21pdGgifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vd3d3Lm9ubHlvZmZpY2UuY29tL3BsdWdpbi1yYWluYm93L2NvbmZpZy5qc29uIl19fSwid2lkdGgiOiIxMDAlIiwiaGVpZ2h0IjoiMTAwJSIsImlhdCI6MTc5MTE5MTQyMn0.n7aQQyvUf-Ntr_1IA7FB00iY6IYFvJKkpjdyEuU_nZ8",
  "time": 872,
  "supportAuthChangesAck": true
}
```

### 09:10:24.059   <-  Alice      auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "dzwB-JlmLYyWl_JyAI6L",
  "sessionTimeConnect": 1791191423699,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "dzwB-JlmLYyWl_JyAI6L",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiIzOTQ0Njc3Yy1iMTVmLTQ0MDItOGIxYi1hNTRkOTA2MDA1N2MiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiI3OGUxZTg0MSIsIm5hbWUiOiJKb2huIFNtaXRoIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTExOTE0MjM2OTl9LCJpYXQiOjE3OTExOTE0MjMsImV4cCI6MTc5Mzc4MzQyM30.WTqMz0lBujkQnyVQaG4W_Rk00zaCkef6PQjFoCNoDHI",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.4.1",
  "buildNumber": 15,
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
  "openedAt": 1791198623960
}
```

### 09:10:24.494   <-  Alice      documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/9.4.1-15/data/site/3944677c-b15f-4402-8b1b-a54d9060057c/Editor.bin/Editor.bin?md5=NmYr5Opp3AwDgJPE23jJ-g&expires=1793786016&shardkey=3944677c-b15f-4402-8b1b-a54d9060057c&filename=Editor.bin"
    },
    "openedAt": 1791198623960
  }
}
```

### 09:10:24.975   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:480"
}
```

### 09:10:25.082   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:106"
}
```

### 09:10:25.185   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 09:10:25.472   ->  Alice      getMessages

```json
{
  "type": "getMessages"
}
```

### 09:10:25.472   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:2213 memory:{\"totalJSHeapSize\":170336940,\"usedJSHeapSize\":120870788,\"jsHeapSizeLimit\":4395630592}"
}
```

### 09:10:25.625   <-  Alice      message

```json
{
  "type": "message"
}
```

### 09:10:29.375       Bob       [Transport WebSocket] ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/3944677c-b15f-4402-8b1b-a54d9060057c/c/?shardkey=3944677c-b15f-4402-8b1b-a54d9060057c&EIO=4&transport=websocket"
```

### 09:10:29.695   <-  Bob       [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "9tXci02zY0sPm2k7AEPj",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:10:30.091   <-  Bob       license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.4.1",
    "buildNumber": 15,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": true,
    "customization": true,
    "advancedApi": true
  }
}
```

### 09:10:30.170   ->  Bob       auth

```json
{
  "type": "auth",
  "docid": "3944677c-b15f-4402-8b1b-a54d9060057c",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
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
    "id": "3944677c-b15f-4402-8b1b-a54d9060057c",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "https://static.onlyoffice.com/assets/docs/samples/demo.docx",
    "title": "Example Document Title.docx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8193,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "permissions": {
    "edit": true,
    "review": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "headingsColor": null,
  "coEditingMode": "fast",
  "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiIzOTQ0Njc3Yy1iMTVmLTQ0MDItOGIxYi1hNTRkOTA2MDA1N2MiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6IkY4OWQ4MDY5YmEyYiIsIm5hbWUiOiJLYXRlIENhZ2UifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vd3d3Lm9ubHlvZmZpY2UuY29tL3BsdWdpbi1yYWluYm93L2NvbmZpZy5qc29uIl19fSwid2lkdGgiOiIxMDAlIiwiaGVpZ2h0IjoiMTAwJSIsImlhdCI6MTc5MTE5MTQyMn0.1oBM-5rN8ZjFD1WEZ-yI0lbVn9gGRB-zuApAfA63jpg",
  "time": 1000,
  "supportAuthChangesAck": true
}
```

### 09:10:30.340   <-  Bob       waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "dzwB-JlmLYyWl_JyAI6L",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 09:10:30.345   <-  Alice      connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791191430256,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "dzwB-JlmLYyWl_JyAI6L",
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
      "connectionId": "Zhw040N0lQ1KeN0aAEPk",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 09:10:30.347   ->  Alice      unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 09:10:30.353   ->  Alice      cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
}
```

### 09:10:30.374   <-  Bob       documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/9.4.1-15/data/site/3944677c-b15f-4402-8b1b-a54d9060057c/Editor.bin/Editor.bin?md5=NmYr5Opp3AwDgJPE23jJ-g&expires=1793786016&shardkey=3944677c-b15f-4402-8b1b-a54d9060057c&filename=Editor.bin"
    },
    "openedAt": 1791198623960
  }
}
```

### 09:10:30.507   <-  Bob       cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
      "time": 1791191430427,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:10:30.515   <-  Bob       auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Zhw040N0lQ1KeN0aAEPk",
  "sessionTimeConnect": 1791191430014,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "dzwB-JlmLYyWl_JyAI6L",
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
      "connectionId": "Zhw040N0lQ1KeN0aAEPk",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiIzOTQ0Njc3Yy1iMTVmLTQ0MDItOGIxYi1hNTRkOTA2MDA1N2MiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiJGODlkODA2OWJhMmIiLCJuYW1lIjoiS2F0ZSBDYWdlIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTExOTE0MzAwMTR9LCJpYXQiOjE3OTExOTE0MzAsImV4cCI6MTc5Mzc4MzQzMH0.dvm_iOqJdFrz6-YM1RGHxx1J1lJyx4fN_qqp1PYfuVg",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.4.1",
  "buildNumber": 15,
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

### 09:10:30.541   ->  Bob       clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:166"
}
```

### 09:10:30.597   ->  Bob       clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:55"
}
```

### 09:10:30.647   ->  Bob       clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 09:10:30.841   ->  Bob       getMessages

```json
{
  "type": "getMessages"
}
```

### 09:10:30.842   ->  Bob       clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1505 memory:{\"totalJSHeapSize\":260798124,\"usedJSHeapSize\":212623176,\"jsHeapSizeLimit\":4395630592}"
}
```

### 09:10:30.892   ->  Bob       cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
}
```

### 09:10:30.993   <-  Bob       message

```json
{
  "type": "message"
}
```

### 09:10:31.047   <-  Alice      cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
      "time": 1791191430966,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```
