## Events

### 08:58:09.979       John Smith      [Transport WebSocket] ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/32965127-3f24-4b0d-9759-be2515530d61/c/?shardkey=32965127-3f24-4b0d-9759-be2515530d61&EIO=4&transport=websocket"
```

### 08:58:10.310   <-  John Smith      [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "lNkc5is4ph-Yd72IAHyb",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 08:58:10.465   <-  John Smith      license

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

### 08:58:10.563   ->  John Smith      auth

```json
{
  "type": "auth",
  "docid": "32965127-3f24-4b0d-9759-be2515530d61",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "78e1e841",
    "username": "John Smith",
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
    "id": "32965127-3f24-4b0d-9759-be2515530d61",
    "userid": "78e1e841",
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
  "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiIzMjk2NTEyNy0zZjI0LTRiMGQtOTc1OS1iZTI1MTU1MzBkNjEiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6Ijc4ZTFlODQxIiwibmFtZSI6IkpvaG4gU21pdGgifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vb25seW9mZmljZS5jb20vcGx1Z2luLXJhaW5ib3cvY29uZmlnLmpzb24iXX19LCJ3aWR0aCI6IjEwMCUiLCJoZWlnaHQiOiIxMDAlIiwiaWF0IjoxNzg4NzcxNDg4fQ.UiBLEPwa-MzKaQ96GnpYvAW2U-QQzq97a7OL-drcJFA",
  "time": 914,
  "supportAuthChangesAck": true
}
```

### 08:58:10.739   <-  John Smith      auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "5-InCfhp8AgLcw47AHyc",
  "sessionTimeConnect": 1788771490389,
  "participants": [
    {
      "id": "78e1e8411",
      "idOriginal": "78e1e841",
      "username": "John Smith",
      "indexUser": 1,
      "view": false,
      "connectionId": "5-InCfhp8AgLcw47AHyc",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiIzMjk2NTEyNy0zZjI0LTRiMGQtOTc1OS1iZTI1MTU1MzBkNjEiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiI3OGUxZTg0MSIsIm5hbWUiOiJKb2huIFNtaXRoIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3ODg3NzE0OTAzODl9LCJpYXQiOjE3ODg3NzE0OTAsImV4cCI6MTc5MTM2MzQ5MH0.Pr_8WM7U6oxTSgBGw_K_MKWnH7vNiRYSFcYEN12QfDE",
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
  "openedAt": 1788778690642
}
```

### 08:58:11.172   <-  John Smith      documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/9.4.1-15/data/site/32965127-3f24-4b0d-9759-be2515530d61/Editor.bin/Editor.bin?md5=suPHNeYIv2vV0JGd4RO4Dw&expires=1791366083&shardkey=32965127-3f24-4b0d-9759-be2515530d61&filename=Editor.bin"
    },
    "openedAt": 1788778690642
  }
}
```

### 08:58:11.798   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:624"
}
```

### 08:58:11.902   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:104"
}
```

### 08:58:12.004   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 08:58:12.293   ->  John Smith      getMessages

```json
{
  "type": "getMessages"
}
```

### 08:58:12.294   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:2391 memory:{\"totalJSHeapSize\":163028548,\"usedJSHeapSize\":118734200,\"jsHeapSizeLimit\":4395630592}"
}
```

### 08:58:12.446   <-  John Smith      message

```json
{
  "type": "message"
}
```
