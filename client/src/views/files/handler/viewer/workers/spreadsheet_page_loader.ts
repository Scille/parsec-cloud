// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

onmessage = function (event): void {
  import('xlsx').then((XLSX) => {
    const start = Date.now();
    const content = XLSX.utils.sheet_to_json(event.data.sheet, { header: 'A' });
    const end = Date.now();
    const delay = end - start < 1000 ? 1000 - (end - start) : 0;
    // Adds a small delay if the loading is very fast to avoid blinking
    setTimeout(() => {
      postMessage({ sheetName: event.data.sheetName, content: content });
    }, delay);
  });
};
