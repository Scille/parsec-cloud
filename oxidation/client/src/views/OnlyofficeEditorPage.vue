<template>
  <div class="form">
    <div id="iframeEditor" />
  </div>
</template>

<script setup lang='ts'>
// <script type="text/javascript" src="<%= apiUrl %>"></script>

let docEditor;

let innerAlert = function (message, inEditor) {
    if (console && console.log)
        console.log(message);
    if (inEditor && docEditor)
        docEditor.showMessage(message);
};

let onAppReady = function () {  // the application is loaded into the browser
    innerAlert("Document editor ready");
};

let onDocumentStateChange = function (event) {  // the document is modified
    let title = document.title.replace(/\*$/g, "");
    document.title = title + (event.data ? "*" : "");
};

let onMetaChange = function (event) {  // the meta information of the document is changed via the meta command
    let favorite = !!event.data.favorite;
    let title = document.title.replace(/^\☆/g, "");
    document.title = (favorite ? "☆" : "") + title;
    docEditor.setFavorite(favorite);  // change the Favorite icon state
};

let onRequestEditRights = function () {  // the user is trying to switch the document from the viewing into the editing mode
    location.href = location.href.replace(RegExp("mode=view\&?", "i"), "");
};

let onRequestHistory = function (event) {  // the user is trying to show the document version history
    let historyObj = <%- JSON.stringify(history) %> || null;

    docEditor.refreshHistory(  // show the document version history
        {
            currentVersion: "<%- file.version %>",
            history: historyObj
        });
};

let onRequestHistoryData = function (data) {  // the user is trying to click the specific document version in the document version history
    let version = data.data;
    let historyData = <%- JSON.stringify(historyData) %> || null;

    docEditor.setHistoryData(historyData[version-1]);  // send the link to the document for viewing the version history
};

let onRequestHistoryClose = function (event){  // the user is trying to go back to the document from viewing the document version history
    document.location.reload();
};

let onError = function (event) {  // an error or some other specific event occurs
    if (event)
        innerAlert(event.data);
};

let onOutdatedVersion = function (event) {  // the document is opened for editing with the old document.key value
    location.reload(true);
};

// replace the link to the document which contains a bookmark
let replaceActionLink = function(href, linkParam) {
    let link;
    let actionIndex = href.indexOf("&action=");
    if (actionIndex != -1) {
        let endIndex = href.indexOf("&", actionIndex + "&action=".length);
        if (endIndex != -1) {
            link = href.substring(0, actionIndex) + href.substring(endIndex) + "&action=" + encodeURIComponent(linkParam);
        } else {
            link = href.substring(0, actionIndex) + "&action=" + encodeURIComponent(linkParam);
        }
    } else {
        link = href + "&action=" + encodeURIComponent(linkParam);
    }
    return link;
}

let onMakeActionLink = function (event) {  // the user is trying to get link for opening the document which contains a bookmark, scrolling to the bookmark position
    let actionData = event.data;
    let linkParam = JSON.stringify(actionData);
    docEditor.setActionLink(replaceActionLink(location.href, linkParam));  // set the link to the document which contains a bookmark
};

let onRequestInsertImage = function(event) {  // the user is trying to insert an image by clicking the Image from Storage button
    docEditor.insertImage({  // insert an image into the file
        "c": event.data.c,
        <%- JSON.stringify(dataInsertImage).substring(1, JSON.stringify(dataInsertImage).length - 1)%>
    })
};

let onRequestCompareFile = function() {  // the user is trying to select document for comparing by clicking the Document from Storage button
    docEditor.setRevisedFile(<%- JSON.stringify(dataCompareFile) %>);  // select a document for comparing
};

let onRequestMailMergeRecipients = function (event) {  // the user is trying to select recipients data by clicking the Mail merge button
    docEditor.setMailMergeRecipients(<%- JSON.stringify(dataMailMergeRecipients) %>);  // insert recipient data for mail merge into the file
};

let onRequestUsers = function () {  // add mentions for not anonymous users
    docEditor.setUsers({  // set a list of users to mention in the comments
        "users": <%- JSON.stringify(usersForMentions) %>
    });
};

let onRequestSendNotify = function(event) {  // the user is mentioned in a comment
    event.data.actionLink = replaceActionLink(location.href, event.data.actionLink);
    let data = JSON.stringify(event.data);
    innerAlert("onRequestSendNotify: " + data);
};

let onRequestSaveAs = function (event) {  //  the user is trying to save file by clicking Save Copy as... button
    let title = event.data.title;
    let url = event.data.url;
    let data = {
        title: title,
        url: url
    }
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "create");
    xhr.setRequestHeader( 'Content-Type', 'application/json');
    xhr.send(JSON.stringify(data));
    xhr.onload = function () {
        innerAlert(xhr.responseText);
        innerAlert(JSON.parse(xhr.responseText).file, true);
    }
}

let config = {<%- include("config") %>,
        events: {
            "onAppReady": onAppReady,
            "onDocumentStateChange": onDocumentStateChange,
            "onRequestEditRights": onRequestEditRights,
            "onError": onError,
            "onRequestHistory":  onRequestHistory,
            "onRequestHistoryData": onRequestHistoryData,
            "onRequestHistoryClose": onRequestHistoryClose,
            "onOutdatedVersion": onOutdatedVersion,
            "onMakeActionLink": onMakeActionLink,
            "onMetaChange": onMetaChange,
            "onRequestInsertImage": onRequestInsertImage,
            "onRequestCompareFile": onRequestCompareFile,
            "onRequestMailMergeRecipients": onRequestMailMergeRecipients,
        }
    };

if (<%- JSON.stringify(usersForMentions) %> != null) {
    config.events.onRequestUsers = onRequestUsers;
    config.events.onRequestSendNotify = onRequestSendNotify;
}

if (config.editorConfig.createUrl) {
    config.events.onRequestSaveAs = onRequestSaveAs;
}

let connectEditor = function () {

  if ((config.document.fileType === "docxf" || config.document.fileType === "oform")
      && DocsAPI.DocEditor.version().split(".")[0] < 7) {
      innerAlert("Please update ONLYOFFICE Docs to version 7.0 to work on fillable forms online.");
      return;
  }

  docEditor = new DocsAPI.DocEditor("iframeEditor", config);
  fixSize();
};

// get the editor sizes
let fixSize = function () {
    let wrapEl = document.getElementsByClassName("form");
    if (wrapEl.length) {
        wrapEl[0].style.height = screen.availHeight + "px";
        window.scrollTo(0, -1);
        wrapEl[0].style.height = window.innerHeight + "px";
    }
};

if (window.addEventListener) {
    window.addEventListener("load", connectEditor);
    window.addEventListener("resize", fixSize);
} else if (window.attachEvent) {
    window.attachEvent("onload", connectEditor);
    window.attachEvent("onresize", fixSize);
}


docManager.init(storageFolder, req, res);

let fileName = fileUtility.getFileName(req.query.fileName);
let fileExt = req.query.fileExt;
let history = [];
let historyData = [];
let lang = docManager.getLang();
let user = users.getUser(req.query.userid);

let userid = user.id;
let name = user.name;
let actionData = req.query.action ? req.query.action : 'null';

let templatesImageUrl = docManager.getTemplateImageUrl(fileUtility.getFileType(fileName));
let createUrl = docManager.getCreateUrl(fileUtility.getFileType(fileName), userid, type, lang);
let templates = [
    {
        'image': '',
        'title': 'Blank',
        'url': createUrl
    },
    {
        'image': templatesImageUrl,
        'title': 'With sample content',
        'url': createUrl + '&sample=true'
    }
];

let userGroup = user.group;
let reviewGroups = user.reviewGroups;
let commentGroups = user.commentGroups;

if (fileExt != null) {
    let fileName = docManager.createDemo(!!req.query.sample, fileExt, userid, name);  // create demo document of a given extension

    // get the redirect path
    let redirectPath = docManager.getServerUrl() + '/editor?fileName=' + encodeURIComponent(fileName) + docManager.getCustomParams();
    res.redirect(redirectPath);
    return;
}
fileExt = fileUtility.getFileExtension(fileName);

let userAddress = docManager.curUserHostAddress();
if (!docManager.existsSync(docManager.storagePath(fileName, userAddress))) {  // if the file with a given name doesn't exist
    throw {
        'message': 'File not found: ' + fileName  // display error message
    };
}
let key = docManager.getKey(fileName);
let url = docManager.getDownloadUrl(fileName);
let urlUser = docManager.getlocalFileUri(fileName, 0, false)
let mode = req.query.mode || 'edit'; // mode: view/edit/review/comment/fillForms/embedded
let type = req.query.type || ''; // type: embedded/mobile/desktop
if (type == '') {
        type = new RegExp(configServer.get('mobileRegEx'), 'i').test(req.get('User-Agent')) ? 'mobile' : 'desktop';
    }

let canEdit = configServer.get('editedDocs').indexOf(fileExt) != -1;  // check if this file can be edited
if ((!canEdit && mode == 'edit' || mode == 'fillForms') && configServer.get('fillDocs').indexOf(fileExt) != -1) {
    mode = 'fillForms';
    canEdit = true;
}
let submitForm = mode == 'fillForms' && userid == 'uid-1' && !1;

let countVersion = 1;

let historyPath = docManager.historyPath(fileName, userAddress);
let changes = null;
let keyVersion = key;

if (historyPath != '') {

    countVersion = docManager.countVersion(historyPath) + 1;  // get the number of file versions
    for (let i = 1; i <= countVersion; i++) {  // get keys to all the file versions
        if (i < countVersion) {
            let keyPath = docManager.keyPath(fileName, userAddress, i);
            if (!fileSystem.existsSync(keyPath)) continue;
            keyVersion = '' + fileSystem.readFileSync(keyPath);
        } else {
            keyVersion = key;
        }
        history.push(docManager.getHistory(fileName, changes, keyVersion, i));  // write all the file history information

        let historyD = {
            version: i,
            key: keyVersion,
            url: i == countVersion ? url : (docManager.getlocalFileUri(fileName, i, true) + '/prev' + fileExt),
        };

        if (i > 1 && docManager.existsSync(docManager.diffPath(fileName, userAddress, i-1))) {  // check if the path to the file with document versions differences exists
            historyD.previous = {  // write information about previous file version
                key: historyData[i-2].key,
                url: historyData[i-2].url,
            };
            historyD.changesUrl = docManager.getlocalFileUri(fileName, i-1) + '/diff.zip';  // get the path to the diff.zip file and write it to the history object
        }

        historyData.push(historyD);

        if (i < countVersion) {
            let changesFile = docManager.changesPath(fileName, userAddress, i);  // get the path to the file with document changes
            changes = docManager.getChanges(changesFile);  // get changes made in the file
        }
    }
} else {  // if history path is empty
    history.push(docManager.getHistory(fileName, changes, keyVersion, countVersion));  // write the history information about the last file version
    historyData.push({
        version: countVersion,
        key: key,
        url: url
    });
}

if (cfgSignatureEnable) {
    for (let i = 0; i < historyData.length; i++) {
        historyData[i].token = jwt.sign(historyData[i], cfgSignatureSecret, {expiresIn: cfgSignatureSecretExpiresIn});  // sign token with given data using signature secret
    }
}

// file config data
let argss = {
    apiUrl: siteUrl + configServer.get('apiUrl'),
    file: {
        name: fileName,
        ext: fileUtility.getFileExtension(fileName, true),
        uri: url,
        uriUser: urlUser,
        version: countVersion,
        created: new Date().toDateString(),
        favorite: user.favorite != null ? user.favorite : 'null'
    },
    editor: {
        type: type,
        documentType: fileUtility.getFileType(fileName),
        key: key,
        token: '',
        callbackUrl: docManager.getCallback(fileName),
        createUrl: userid != 'uid-0' ? createUrl : null,
        templates: user.templates ? templates : null,
        isEdit: canEdit && (mode == 'edit' || mode == 'view' || mode == 'filter' || mode == 'blockcontent'),
        review: canEdit && (mode == 'edit' || mode == 'review'),
        comment: mode != 'view' && mode != 'fillForms' && mode != 'embedded' && mode != 'blockcontent',
        fillForms: mode != 'view' && mode != 'comment' && mode != 'embedded' && mode != 'blockcontent',
        modifyFilter: mode != 'filter',
        modifyContentControl: mode != 'blockcontent',
        copy: !user.deniedPermissions.includes('copy'),
        download: !user.deniedPermissions.includes('download'),
        print: !user.deniedPermissions.includes('print'),
        mode: canEdit && mode != 'view' ? 'edit' : 'view',
        canBackToFolder: type != 'embedded',
        backUrl: docManager.getServerUrl() + '/',
        curUserHostAddress: docManager.curUserHostAddress(),
        lang: lang,
        userid: userid,
        name: name,
        userGroup: userGroup,
        reviewGroups: JSON.stringify(reviewGroups),
        commentGroups: JSON.stringify(commentGroups),
        fileChoiceUrl: fileChoiceUrl,
        submitForm: submitForm,
        plugins: JSON.stringify(plugins),
        actionData: actionData
    },
    history: history,
    historyData: historyData,
    dataInsertImage: {
        fileType: 'png',
        url: docManager.getServerUrl(true) + '/images/logo.png'
    },
    dataCompareFile: {
        fileType: 'docx',
        url: docManager.getServerUrl(true) + '/assets/sample/sample.docx'
    },
    dataMailMergeRecipients: {
        fileType: 'csv',
        url: docManager.getServerUrl(true) + '/csv'
    },
    usersForMentions: user.id != 'uid-0' ? users.getUsersForMentions(user.id) : null,
};

if (cfgSignatureEnable) {
  app.render('config', argss, function(err, html){  // render a config template with the parameters specified
    if (err) {
      console.log(err);
    } else {
      // sign token with given data using signature secret
      argss.editor.token = jwt.sign(JSON.parse('{'+html+'}'), cfgSignatureSecret, {expiresIn: cfgSignatureSecretExpiresIn});
      argss.dataInsertImage.token = jwt.sign(argss.dataInsertImage, cfgSignatureSecret, {expiresIn: cfgSignatureSecretExpiresIn});
      argss.dataCompareFile.token = jwt.sign(argss.dataCompareFile, cfgSignatureSecret, {expiresIn: cfgSignatureSecretExpiresIn});
      argss.dataMailMergeRecipients.token = jwt.sign(argss.dataMailMergeRecipients, cfgSignatureSecret, {expiresIn: cfgSignatureSecretExpiresIn});
    }
    res.render('editor', argss);  // render the editor template with the parameters specified
  });
} else {
  res.render('editor', argss);
}
</script>
