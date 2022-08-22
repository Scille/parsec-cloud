export const configServer = {
  'port': 3000,
  'siteUrl': 'http://documentserver/',
  'wopi': {
    'discovery': 'hosting/discovery'
  },
  'commandUrl': 'coauthoring/CommandService.ashx',
  'converterUrl': 'ConvertService.ashx',
  'apiUrl': 'web-apps/apps/api/documents/api.js',
  'preloaderUrl': 'web-apps/apps/api/documents/cache-scripts.html',
  'exampleUrl': null,
  'viewedDocs': ['.pdf', '.djvu', '.xps', '.oxps'],
  'editedDocs': ['.docx', '.xlsx', '.csv', '.pptx', '.txt', '.docxf'],
  'fillDocs': ['.docx', '.oform'],
  'convertedDocs': [
    '.docm', '.doc', '.dotx', '.dotm', '.dot', '.odt', '.fodt', '.ott',
    '.xlsm', '.xls', '.xltx', '.xltm', '.xlt', '.ods', '.fods', '.ots',
    '.pptm', '.ppt', '.ppsx', '.ppsm', '.pps', '.potx', '.potm', '.pot',
    '.odp', '.fodp', '.otp', '.rtf', '.mht', '.html', '.htm', '.xml', '.epub',
    '.fb2'
  ],
  'storageFolder': './files',
  'storagePath': '/files',
  'maxFileSize': 1073741824,
  'static':[
    {
      'name': '/files',
      'path': './files'
    }
  ],
  'token': {
    'enable': false,
    'useforrequest': true,
    'algorithmRequest': 'HS256',
    'authorizationHeader': 'Authorization',
    'authorizationHeaderPrefix': 'Bearer ',
    'secret': 'secret',
    'expiresIn': '5m'
  }
};
