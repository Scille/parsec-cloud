# 5 - Add characater

## 5.1 - I change

### client → server isSaveLock [ephemeral] 5:09:17 PM

```json
{
 "type": "isSaveLock",
 "syncChangesIndex": 799
}
```

### server → client saveLock [ephemeral] 5:09:17 PM

```json
{
 "type": "saveLock",
 "saveLock": false
}
```

### client → server saveChanges [persistent] 5:09:17 PM

```json
{
 "type": "saveChanges",
 "changes": "[\"64;AgAAADEA//8BACxLuimoIAIApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///w4AAAAwAC4AMAAuADAALgAwAA==\",\"37;CAAAADAAXwAyADEAAQAcAAEAAAACAAAAAQAAAGEAAAAAAwAAAA==\"]",
 "startSaveChanges": true,
 "endSaveChanges": true,
 "isCoAuthoring": true,
 "isExcel": false,
 "deleteIndex": null,
 "excelAdditionalInfo": "{\"UserId\":\"a11cec001000000000000000000000000\",\"UserShortId\":\"a11cec00100000000000000000000000\",\"CursorInfo\":\"16;CAAAADAAXwAyADEAAwAAAA==\"}",
 "unlock": false,
 "releaseLocks": true
}
```

### server → client unSaveLock [ephemeral] 5:09:17 PM

```json
{
 "type": "unSaveLock",
 "index": 100,
 "time": 1786979357682
}
```

### server → client (info) checkpoint-threshold [ephemeral] 5:09:17 PM · not a real OnlyOffice message

```json
{
 "type": "(info) checkpoint-threshold",
 "seq": 100,
 "note": "a real server would snapshot the document here and trim the log"
}
```

## 5.2 - I receive a change

???
