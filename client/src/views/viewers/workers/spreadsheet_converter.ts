// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

onmessage = function (event): void {
  import('xlsx').then((XLSX) => {
    const content = XLSX.utils.sheet_to_html(event.data, { header: '', footer: '' });
    postMessage(content);
  });
};
