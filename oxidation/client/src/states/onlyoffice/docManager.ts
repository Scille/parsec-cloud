// const path = require("path");
// const fileSystem = require("fs");
// const fileUtility = require("./fileUtility");
// const documentService = require("./documentService");
// const cacheManager = require("./cacheManager");
// const guidManager = require("./guidManager");
// const configServer = require('config').get('server');
// const os = require("os");

export const docManager: any = {};

docManager.dir = null;
docManager.req = null;
docManager.res = null;

// check if the path exists or not
// docManager.existsSync = function(path: string): boolean {
//   let res = true;
//   try {
// synchronously test the user's permissions for the directory specified by path; the directory is visible to the calling process
//       fileSystem.accessSync(path, fileSystem.F_OK);
//   } catch (e) {  // the response is set to false, if an error occurs
//       res = false;
//   }
//   return res;
// };

// create a new directory if it doesn't exist
// docManager.createDirectory = function(path) {
//   if (!this.existsSync(path)) {
//     fileSystem.mkdirSync(path);
//   }
// };

docManager.init = function (dir: string, req: string, res: string): void {
  docManager.dir = dir;
  docManager.req = req;
  docManager.res = res;
};

// get the language from the request
// docManager.getLang = function () {
//   if (docManager.req.query.lang) {
//     return docManager.req.query.lang;
//   } else {  // the default language value is English
//     return "en"
//   }
// };

// get customization parameters
// docManager.getCustomParams = function () {
//   let params = "";

//   const userid = docManager.req.query.userid;  // user id
//   params += (userid ? "&userid=" + userid : "");

//   const lang = docManager.req.query.lang;  // language
//   params += (lang ? "&lang=" + docManager.getLang() : "");

//   const fileName = docManager.req.query.fileName;  // file name
//   params += (fileName ? "&fileName=" + fileName : "");

//   const mode = docManager.req.query.mode;  // mode: view/edit/review/comment/fillForms/embedded
//   params += (mode ? "&mode=" + mode : "");

//   const type = docManager.req.query.type;  // type: embedded/mobile/desktop
//   params += (type ? "&type=" + type : "");

//   return params;
// };

// get the correct file name if such a name already exists
// docManager.getCorrectName = function (fileName, userAddress) {
//   const baseName = fileUtility.getFileName(fileName, true);  // get file name from the url without extension
//   const ext = fileUtility.getFileExtension(fileName);  // get file extension from the url
//   let name = baseName + ext;  // get full file name
//   let index = 1;

//   while (this.existsSync(docManager.storagePath(name, userAddress))) {  // if the file with such a name already exists in this directory
//       name = baseName + " (" + index + ")" + ext;  // add an index after its base name
//       index++;
//   }

//   return name;
// };

// create demo document
// docManager.createDemo = function (isSample, fileExt, userid, username) {
//   const demoName = (isSample ? "sample" : "new") + "." + fileExt;
//   const fileName = docManager.getCorrectName(demoName);  // get the correct file name if such a name already exists

// copy sample document of a necessary extension to the storage path
//   docManager.copyFile(path.join(__dirname, "..","public", "assets", isSample
// ? "sample" : "new", demoName), docManager.storagePath(fileName));

//   docManager.saveFileData(fileName, userid, username);  // save file data to the file

//   return fileName;
// };

// save file data to the file
// docManager.saveFileData = function (fileName, userid, username, userAddress) {
//   if (!userAddress) {
//       userAddress = docManager.curUserHostAddress();  // get current user host address
//   }
//   // get full creation date of the document
//   const date_create = fileSystem.statSync(docManager.storagePath(fileName, userAddress)).mtime;
//   const minutes = (date_create.getMinutes() < 10 ? '0' : '') + date_create.getMinutes().toString();
//   const month = (date_create.getMonth() < 10 ? '0' : '') + (parseInt(date_create.getMonth().toString()) + 1);
//   const sec = (date_create.getSeconds() < 10 ? '0' : '') + date_create.getSeconds().toString();
//   const date_format = date_create.getFullYear() + "-" + month + "-" +
//  date_create.getDate() + " " + date_create.getHours() + ":" + minutes + ":" + sec;

//   const file_info = docManager.historyPath(fileName, userAddress, true);  // get file history information
//   this.createDirectory(file_info);  // create a new history directory if it doesn't exist

//   fileSystem.writeFileSync(path.join(file_info, fileName + ".txt"),
//  date_format + "," + userid + "," + username);  // write all the file information to a new txt file
// };

// get file data
// docManager.getFileData = function (fileName, userAddress) {
//   const history =
// path.join(docManager.historyPath(fileName, userAddress, true), fileName + ".txt");  // get the path to the file with file information
//   if (!this.existsSync(history)) {  // if such a file doesn't exist
//       return ["2017-01-01", "uid-1", "John Smith"];  // return default information
//   }

//   return ((fileSystem.readFileSync(history)).toString()).split(",");
// };

// get url to the original file
// docManager.getFileUri = function (fileName) {
//   return docManager.getlocalFileUri(fileName, 0, true);
// };

// get local file url
// docManager.getlocalFileUri = function (fileName, version, forDocumentServer) {
//   const serverPath = docManager.getServerUrl(forDocumentServer);
//   const hostAddress = docManager.curUserHostAddress();
//   const url = serverPath + configServer.get("storagePath") + "/" +
// hostAddress + "/" + encodeURIComponent(fileName);  // get full url address to the file
//   if (!version) {
//       return url;
//   }
//   return url + "-history/" + version;  // return history path to the specified file version
// };

// get server url
// docManager.getServerUrl = function (forDocumentServer) {
//   return (forDocumentServer && !!configServer.get("exampleUrl")) ? configServer.get("exampleUrl") : docManager.getServerPath();
// };

// get server address from the request
// docManager.getServerPath = function () {
//   return docManager.getServerHost() + (docManager.req.headers["x-forwarded-path"] || docManager.req.baseUrl);
// };

// get host address from the request
// docManager.getServerHost = function () {
//   return docManager.getProtocol() + "://" + (docManager.req.headers["x-forwarded-host"] || docManager.req.headers["host"]);
// };

// get protocol from the request
// docManager.getProtocol = function () {
//   return docManager.req.headers["x-forwarded-proto"] || docManager.req.protocol;
// };

// get callback url
// docManager.getCallback = function (fileName) {
//   const server = docManager.getServerUrl(true);
//   const hostAddress = docManager.curUserHostAddress();
//   const handler = "/track?filename=" + encodeURIComponent(fileName) +
// "&useraddress=" + encodeURIComponent(hostAddress);  // get callback handler

//   return server + handler;
// };

// get url to the created file
// docManager.getCreateUrl = function (docType, userid, type, lang) {
//   const server = docManager.getServerUrl();
//   var ext = docManager.getInternalExtension(docType).replace(".", "");
//   const handler = "/editor?fileExt=" + ext + "&userid=" + userid + "&type=" + type + "&lang=" + lang;

//   return server + handler;
// }

// get url to download a file
// docManager.getDownloadUrl = function (fileName) {
//   const server = docManager.getServerUrl(true);
//   const hostAddress = docManager.curUserHostAddress();
//   const handler = "/download?fileName=" + encodeURIComponent(fileName) + "&useraddress=" + encodeURIComponent(hostAddress);

//   return server + handler;
// };

// get the storage path of the given file
// docManager.storagePath = function (fileName, userAddress) {
//   fileName = fileUtility.getFileName(fileName);  // get the file name with extension
//   const directory = path.join(docManager.dir,
// docManager.curUserHostAddress(userAddress));  // get the path to the directory for the host address
//   this.createDirectory(directory);  // create a new directory if it doesn't exist
//   return path.join(directory, fileName);  // put the given file to this directory
// };

// get the path to the forcesaved file version
// docManager.forcesavePath = function (fileName, userAddress, create) {
//   let directory = path.join(docManager.dir, docManager.curUserHostAddress(userAddress));
//   if (!this.existsSync(directory)) {  // the directory with host address doesn't exist
//       return "";
//   }
//   directory = path.join(directory, fileName + "-history");  // get the path to the history of the given file
//   if (!create && !this.existsSync(directory)) {  // the history directory doesn't exist and we are not supposed to create it
//       return "";
//   }
//   this.createDirectory(directory);  // create history directory if it doesn't exist
//   directory = path.join(directory, fileName);  // and get the path to the given file
//   if (!create && !this.existsSync(directory)) {
//       return "";
//   }
//   return directory;
// };

// create the path to the file history
// docManager.historyPath = function (fileName, userAddress, create) {
//   let directory = path.join(docManager.dir, docManager.curUserHostAddress(userAddress));
//   if (!this.existsSync(directory)) {
//       return "";
//   }
//   directory = path.join(directory, fileName + "-history");
//   if (!create && !this.existsSync(path.join(directory, "1"))) {
//       return "";
//   }
//   return directory;
// };

// get the path to the specified file version
// docManager.versionPath = function (fileName, userAddress, version) {
// get the path to the history of a given file or create it if it doesn't exist
//   const historyPath = docManager.historyPath(fileName, userAddress, true);
//   return path.join(historyPath, "" + version);
// };

// get the path to the previous file version
// docManager.prevFilePath = function (fileName, userAddress, version) {
//   return path.join(docManager.versionPath(fileName, userAddress, version), "prev" + fileUtility.getFileExtension(fileName));
// };

// get the path to the file with document versions differences
// docManager.diffPath = function (fileName, userAddress, version) {
//   return path.join(docManager.versionPath(fileName, userAddress, version), "diff.zip");
// };

// get the path to the file with document changes
// docManager.changesPath = function (fileName, userAddress, version) {
//   return path.join(docManager.versionPath(fileName, userAddress, version), "changes.txt");
// };

// get the path to the file with key value in it
// docManager.keyPath = function (fileName, userAddress, version) {
//   return path.join(docManager.versionPath(fileName, userAddress, version), "key.txt");
// };

// get the path to the file with the user information
// docManager.changesUser = function (fileName, userAddress, version) {
//   return path.join(docManager.versionPath(fileName, userAddress, version), "user.txt");
// };

// get all the stored files
// docManager.getStoredFiles = function () {
//   const userAddress = docManager.curUserHostAddress();
//   const directory = path.join(docManager.dir, userAddress);
//   this.createDirectory(directory);
//   const result = [];
//   const storedFiles = fileSystem.readdirSync(directory);  // read the user host directory contents
//   for (let i = 0; i < storedFiles.length; i++) {  // run through all the elements from the folder
//       const stats = fileSystem.lstatSync(path.join(directory, storedFiles[i]));  // save element parameters

//       if (!stats.isDirectory()) {  // if the element isn't a directory
//           let historyPath = docManager.historyPath(storedFiles[i], userAddress);  // get the path to the file history
//           let version = 0;
//           if (historyPath != "") {  // if the history path exists
//               version = docManager.countVersion(historyPath);  // get the last file version
//           }

//           const time = stats.mtime.getTime();  // get the time of element modification
//           const item = {  // create an object with element data
//               time: time,
//               name: storedFiles[i],
//               documentType: fileUtility.getFileType(storedFiles[i]),
//               canEdit: configServer.get("editedDocs").indexOf(fileUtility.getFileExtension(storedFiles[i])) != -1,
//               version: version+1
//           };

//           if (!result.length) {  // if the result array is empty
//               result.push(item);  // push the item object to it
//           } else {
//               let j = 0;
//               for (; j < result.length; j++) {
//                   if (time > result[j].time) {  // otherwise, run through all the objects from the result array
//                       break;
//                   }
//               }
//               result.splice(j, 0, item);  // and add new object in ascending order of time
//           }
//       }
//   }
//   return result;
// };

// get current user host address
// docManager.curUserHostAddress = function (userAddress) {
//   if (!userAddress)  // if user address isn't passed to the function
//       userAddress = docManager.req.headers["x-forwarded-for"] ||
// docManager.req.connection.remoteAddress;  // take it from the header or use the remote address

//   return userAddress.replace(new RegExp("[^0-9a-zA-Z.=]", "g"), "_");
// };

// copy file
// docManager.copyFile = function (exist, target) {
//   fileSystem.writeFileSync(target, fileSystem.readFileSync(exist));
// };

// get an internal extension
// docManager.getInternalExtension = function (fileType) {
//   if (fileType == fileUtility.fileType.word)  // .docx for word type
//       return ".docx";

//   if (fileType == fileUtility.fileType.cell)  // .xlsx for cell type
//       return ".xlsx";

//   if (fileType == fileUtility.fileType.slide)  // .pptx for slide type
//       return ".pptx";

//   return ".docx";  // the default value is .docx
// };

// get the template image url
// docManager.getTemplateImageUrl = function (fileType) {
//     let path = docManager.getServerUrl(true);
//     if (fileType == fileUtility.fileType.word)  // for word type
//         return path + "/images/file_docx.svg";

//     if (fileType == fileUtility.fileType.cell)  // for cell type
//         return path + "/images/file_xlsx.svg";

//     if (fileType == fileUtility.fileType.slide)  // for slide type
//         return path + "/images/file_pptx.svg";

//     return path + "/images/file_docx.svg";  // the default value
// }

// get document key
// docManager.getKey = function (fileName) {
//     const userAddress = docManager.curUserHostAddress();
// get document key by adding local file url to the current user host address
//     let key = userAddress + docManager.getlocalFileUri(fileName);

//     let historyPath = docManager.historyPath(fileName, userAddress);  // get the path to the file history
//     if (historyPath != ""){  // if the path to the file history exists
//         key += docManager.countVersion(historyPath);  // add file version number to the document key
//     }

//     let storagePath = docManager.storagePath(fileName, userAddress);  // get the storage path to the given file
//     const stat = fileSystem.statSync(storagePath);  // get file information
//     key += stat.mtime.getTime();  // and add creation time to the document key

//     return documentService.generateRevisionId(key);  // generate the document key value
// };

// get current date
// docManager.getDate = function (date) {
//     const minutes = (date.getMinutes() < 10 ? '0' : '') + date.getMinutes().toString();
//     return date.getMonth() + "/" + date.getDate() + "/" + date.getFullYear() + " " + date.getHours() + ":" + minutes;
// };

// get changes made in the file
// docManager.getChanges = function (fileName) {
//     if (this.existsSync(fileName)) {  // if the directory with such a file exists
//         return JSON.parse(fileSystem.readFileSync(fileName));  // read this file and parse it
//     }
//     return null;
// };

// get the last file version
// docManager.countVersion = function(directory) {
//     let i = 0;
//     while (this.existsSync(path.join(directory, '' + (i + 1)))) {  // run through all the file versions
//         i++;  // and count them
//     }
//     return i;
// };

// get file history information
// docManager.getHistory = function (fileName, content, keyVersion, version) {
//     let oldVersion = false;
//     let contentJson = null;
//     if (content) {  // if content is defined
//         if (content.changes && content.changes.length) {  // and there are some modifications in the content
//             contentJson = content.changes[0];  // write these modifications to the json content
//         } else if (content.length){
//             contentJson = content[0];  // otherwise, write original content to the json content
//             oldVersion = true;  // and note that this is an old version
//         } else {
//             content = false;
//         }
//     }

//     const userAddress = docManager.curUserHostAddress();
//     const username = content ? (oldVersion ? contentJson.username :
// contentJson.user.name) : (docManager.getFileData(fileName, userAddress))[2];
//     const userid = content ? (oldVersion ? contentJson.userid :
// contentJson.user.id) : (docManager.getFileData(fileName, userAddress))[1];
//     const created = content ? (oldVersion ? contentJson.date : contentJson.created) : (docManager.getFileData(fileName, userAddress))[0];
//     const res = (content && !oldVersion) ? content : {changes: content};
//     res.key = keyVersion;  // write the information about the user, creation time, key and version to the result object
//     res.version = version;
//     res.created = created;
//     res.user = {
//         id: userid,
//         name: username != "null" ? username : null
//     };

//     return res;
// };

// clean folder
// docManager.cleanFolderRecursive = function (folder, me) {
//     if (fileSystem.existsSync(folder)) {  // if the given folder exists
//         const files = fileSystem.readdirSync(folder);
//         files.forEach((file) => {  // for each file from the folder
//             const curPath = path.join(folder, file);  // get its current path
//             if (fileSystem.lstatSync(curPath).isDirectory()) {
//                 this.cleanFolderRecursive(curPath, true);  // for each folder included in this one repeat the same procedure
//             } else {
//                 fileSystem.unlinkSync(curPath);  // remove the file
//             }
//         });
//         if (me) {
//             fileSystem.rmdirSync(folder);
//         }
//     }
// };

// get files information
// docManager.getFilesInfo = function (fileId) {
//     const userAddress = docManager.curUserHostAddress();
//     const directory = path.join(docManager.dir, userAddress);
//     const filesInDirectory = this.getStoredFiles();  // get all the stored files from the folder
//     let responseArray = [];
//     let responseObject;
//     for (let currentFile = 0; currentFile < filesInDirectory.length; currentFile++) {  // run through all the files from the directory
//         const file = filesInDirectory[currentFile];
//         const stats = fileSystem.lstatSync(path.join(directory, file.name));  // get file information
//         const fileObject = {  // write file parameters to the file object
//             version: file.version,
//             id: this.getKey(file.name),
//             contentLength: `${(stats.size/1024).toFixed(2)} KB`,
//             pureContentLength: stats.size,
//             title: file.name,
//             updated: stats.mtime
//         };
//         if (fileId !== undefined) {  // if file id is defined
//             if (this.getKey(file.name) == fileId) {  // and it is equal to the document key value
//                 responseObject = fileObject;   // response object will be equal to the file object
//                 break;
//             }
//         }
//         else responseArray.push(fileObject);  // otherwise, push file object to the response array
//     };
//     if (fileId !== undefined) {
//         if (responseObject !== undefined) return responseObject;
//         else return "File not found";
//     }
//     else return responseArray;
// };

// save all the functions to the docManager module to export it later in other files
