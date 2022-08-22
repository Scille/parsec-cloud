<template>
  <header>
    <div class="center">
      <a href="">
        <img
          src="@/assets/images/logo.svg"
          alt="ONLYOFFICE"
        >
      </a>
    </div>
  </header>
  <div class="center main">
    <table class="table-main">
      <tbody>
        <tr>
          <td class="left-panel section">
            <div class="help-block">
              <span>Create new</span>
              <div class="clearFix">
                <div class="create-panel clearFix">
                  <ul class="try-editor-list clearFix">
                    <li>
                      <a
                        class="try-editor word reload-page"
                        target="_blank"
                        href="editor?fileExt=docx{{ params }}"
                        title="Create new document"
                      >Document</a>
                    </li>
                    <li>
                      <a
                        class="try-editor cell reload-page"
                        target="_blank"
                        href="editor?fileExt=xlsx{{ params }}"
                        title="Create new spreadsheet"
                      >Spreadsheet</a>
                    </li>
                    <li>
                      <a
                        class="try-editor slide reload-page"
                        target="_blank"
                        href="editor?fileExt=pptx{{ params }}"
                        title="Create new presentation"
                      >Presentation</a>
                    </li>
                    <li>
                      <a
                        class="try-editor form reload-page"
                        target="_blank"
                        href="editor?fileExt=docxf{{ params }}"
                        title="Create new form template"
                      >Form template</a>
                    </li>
                  </ul>
                  <label class="create-sample">
                    <input
                      id="createSample"
                      type="checkbox"
                      class="checkbox"
                    >With sample content
                  </label>
                </div>

                <div class="upload-panel clearFix">
                  <a class="file-upload">Upload file
                    <input
                      type="file"
                      id="fileupload"
                      name="uploadedFile"
                      data-url="upload?{{ params }}"
                    >
                  </a>
                </div>

                <table
                  class="user-block-table"
                  cellspacing="0"
                  cellpadding="0"
                >
                  <tbody>
                    <tr>
                      <td valign="middle">
                        <span class="select-user">Username</span>
                        <img
                          class="info"
                          src="@/assets/images/info.svg"
                        >
                        <select
                          class="select-user"
                          id="user"
                        >
                          <option
                            v-for="user in users"
                            :key="user.id"
                          >
                            {{ user.name == null ? "Anonymous" : user.name }}
                          </option>
                        </select>
                      </td>
                    </tr>
                    <tr>
                      <td valign="middle">
                        <span class="select-user">Language editors interface</span>
                        <select
                          class="select-user"
                          id="language"
                        >
                          <option value="en">
                            English
                          </option>
                          <option value="be">
                            Belarusian
                          </option>
                          <option value="bg">
                            Bulgarian
                          </option>
                          <option value="ca">
                            Catalan
                          </option>
                          <option value="zh">
                            Chinese
                          </option>
                          <option value="cs">
                            Czech
                          </option>
                          <option value="da">
                            Danish
                          </option>
                          <option value="nl">
                            Dutch
                          </option>
                          <option value="fi">
                            Finnish
                          </option>
                          <option value="fr">
                            French
                          </option>
                          <option value="de">
                            German
                          </option>
                          <option value="el">
                            Greek
                          </option>
                          <option value="hu">
                            Hungarian
                          </option>
                          <option value="id">
                            Indonesian
                          </option>
                          <option value="it">
                            Italian
                          </option>
                          <option value="ja">
                            Japanese
                          </option>
                          <option value="ko">
                            Korean
                          </option>
                          <option value="lv">
                            Latvian
                          </option>
                          <option value="lo">
                            Lao
                          </option>
                          <option value="nb">
                            Norwegian
                          </option>
                          <option value="pl">
                            Polish
                          </option>
                          <option value="pt">
                            Portuguese
                          </option>
                          <option value="ro">
                            Romanian
                          </option>
                          <option value="ru">
                            Russian
                          </option>
                          <option value="sk">
                            Slovak
                          </option>
                          <option value="sl">
                            Slovenian
                          </option>
                          <option value="sv">
                            Swedish
                          </option>
                          <option value="es">
                            Spanish
                          </option>
                          <option value="tr">
                            Turkish
                          </option>
                          <option value="uk">
                            Ukrainian
                          </option>
                          <option value="vi">
                            Vietnamese
                          </option>
                        </select>
                      </td>
                    </tr>
                  </tbody>
                </table>

                <div class="links-panel links-panel-border clearFix">
                  <a
                    href="{{ serverUrl }}/wopi"
                    class=""
                  >Go to WOPI page</a>
                </div>
              </div>
            </div>
          </td>
          <td class="section">
            <div class="main-panel">
              <div
                id="portal-info"
                style="display: {{ storedFiles.length > 0 ? 'none' : 'block' }}"
              >
                <span class="portal-name">ONLYOFFICE Document Editors – Welcome!</span>
                <span class="portal-descr">
                  Get started with a demo-sample of ONLYOFFICE Document Editors, the first html5-based editors.
                  <br>
                  You may upload your own documents for testing using the
                  "<b>Upload file</b>" button and <b>selecting</b> the necessary files on your PC.
                </span>
                <span class="portal-descr">
                  You can open the same document using different users in different Web browser sessions,
                  so you can check out multi-user editing functions.
                </span>
                <div
                  v-for="user in users"
                  :key="user.id"
                  class="user-descr"
                >
                  <b>{{ user.name == null ? 'Anonymous' : user.name }}</b>
                  <ul>
                    <li
                      v-for="description in user.descriptions"
                      :key="description"
                    >
                      {{ description }}
                    </li>
                  </ul>
                </div>
              </div>
              <div
                v-if="storedFiles.length > 0"
                class="stored-list"
              >
                <span class="header-list">Your documents</span>
                <table
                  class="tableHeader"
                  cellspacing="0"
                  cellpadding="0"
                  width="100%"
                >
                  <thead>
                    <tr>
                      <td class="tableHeaderCell tableHeaderCellFileName">
                        Filename
                      </td>
                      <td class="tableHeaderCell tableHeaderCellEditors contentCells-shift">
                        Editors
                      </td>
                      <td class="tableHeaderCell tableHeaderCellViewers">
                        Viewers
                      </td>
                      <td class="tableHeaderCell tableHeaderCellDownload">
                        Download
                      </td>
                      <td class="tableHeaderCell tableHeaderCellRemove">
                        Remove
                      </td>
                    </tr>
                  </thead>
                </table>
                <div class="scroll-table-body">
                  <table
                    cellspacing="0"
                    cellpadding="0"
                    width="100%"
                  >
                    <tbody>
                      <tr
                        v-for="(storedFile, i) in storedFiles"
                        :key="storedFile"
                        class="tableRow"
                        title="{{ storedFiles[i].name }} [{{ storedFiles[i].version }}]"
                      >
                        <td class="contentCells">
                          <a
                            class="stored-edit {{ storedFiles[i].documentType }}"
                            href="editor?fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                            target="_blank"
                          >
                            <span>{{ storedFiles[i].name }}</span>
                          </a>
                        </td>
                        <template v-if="storedFiles[i].canEdit">
                          <td class="contentCells contentCells-icon">
                            <a
                              href="editor?type=desktop&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/desktop.svg"
                                alt="Open in editor for full size screens"
                                title="Open in editor for full size screens"
                              >
                            </a>
                          </td>
                          <td class="contentCells contentCells-icon">
                            <a
                              href="editor?type=mobile&mode=edit&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/mobile.svg"
                                alt="Open in editor for mobile devices"
                                title="Open in editor for mobile devices"
                              >
                            </a>
                          </td>
                          <td class="contentCells contentCells-icon">
                            <a
                              href="editor?type=desktop&mode=comment&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/comment.svg"
                                alt="Open in editor for comment"
                                title="Open in editor for comment"
                              >
                            </a>
                          </td>
                          <td
                            v-if="storedFiles[i].documentType == 'word'"
                            class="contentCells contentCells-icon"
                          >
                            <a
                              href="editor?type=desktop&mode=review&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/review.svg"
                                alt="Open in editor for review"
                                title="Open in editor for review"
                              >
                            </a>
                          </td>
                          <td
                            v-else-if="storedFiles[i].documentType == 'cll'"
                            class="contentCells contentCells-icon"
                          >
                            <a
                              href="editor?type=desktop&mode=filter&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/filter.svg"
                                alt="Open in editor without access to change the filter"
                                title="Open in editor without access to change the filter"
                              >
                            </a>
                          </td>
                          <td
                            v-if="storedFiles[i].documentType !== 'word' && storedFiles[i].documentType !== 'cell'"
                            class="contentCells contentCells-icon contentCellsEmpty"
                          />
                          <td
                            v-if="storedFiles[i].documentType == 'word'"
                            class="contentCells contentCells-icon"
                          >
                            <a
                              href="editor?type=desktop&mode=blockcontent&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/block-content.svg"
                                alt="Open in editor without content control modification"
                                title="Open in editor without content control modification"
                              >
                            </a>
                          </td>
                          <td
                            v-else
                            class="contentCells contentCells-icon"
                          />
                          <td
                            v-if="storedFiles[i].documentType !== 'word' && storedFiles[i].documentType !== 'cell'"
                            class="contentCells contentCells-icon"
                          />
                          <td
                            v-if="fillExts.indexOf(
                              storedFiles[i].name.substring(storedFiles[i].name.lastIndexOf('.')).trim().toLowerCase()) !== -1"
                            class="contentCells contentCells-shift contentCells-icon firstContentCellShift"
                          >
                            <a
                              href="editor?type=desktop&mode=fillForms&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/fill-forms.svg"
                                alt="Open in editor for filling in forms"
                                title="Open in editor for filling in forms"
                              >
                            </a>
                          </td>
                          <td
                            v-else
                            class="contentCells contentCells-shift contentCells-icon firstContentCellShift"
                          />
                        </template>

                        <template
                          v-else-if="fillExts.indexOf(
                            storedFiles[i].name.substring(storedFiles[i].name.lastIndexOf('.')).trim().toLowerCase()) !== -1">
                          <td class="contentCells contentCells-icon " />
                          <td class="contentCells contentCells-icon">
                            <a
                              href="editor?type=mobile&mode=fillForms&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/mobile-fill-forms.svg"
                                alt="Open in editor for filling in forms for mobile devices"
                                title="Open in editor for filling in forms for mobile devices"
                              >
                            </a>
                          </td>
                          <td class="contentCells contentCells-icon " />
                          <td class="contentCells contentCells-icon " />
                          <td class="contentCells contentCells-icon " />
                          <td class="contentCells contentCells-shift contentCells-icon firstContentCellShift">
                            <a
                              href="editor?type=desktop&mode=fillForms&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                              target="_blank"
                            >
                              <img
                                src="@/assets/images/fill-forms.svg"
                                alt="Open in editor for filling in forms"
                                title="Open in editor for filling in forms"
                              >
                            </a>
                          </td>
                        </template>

                        <template v-else>
                          <td
                            class="contentCells contentCells-shift contentCells-icon contentCellsEmpty"
                            colspan="6"
                          />
                        </template>

                        <td class="contentCells contentCells-icon firstContentCellViewers">
                          <a
                            href="editor?type=desktop&mode=view&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                            target="_blank"
                          >
                            <img
                              src="@/assets/images/desktop.svg"
                              alt="Open in viewer for full size screens"
                              title="Open in viewer for full size screens"
                            >
                          </a>
                        </td>
                        <td class="contentCells contentCells-icon">
                          <a
                            href="editor?type=mobile&mode=view&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                            target="_blank"
                          >
                            <img
                              src="@/assets/images/mobile.svg"
                              alt="Open in viewer for mobile devices"
                              title="Open in viewer for mobile devices"
                            >
                          </a>
                        </td>
                        <td class="contentCells contentCells-icon contentCells-shift">
                          <a
                            href="editor?type=embedded&mode=embedded&fileName={{ encodeURIComponent(storedFiles[i].name) + params }}"
                            target="_blank"
                          >
                            <img
                              src="@/assets/images/embeded.svg"
                              alt="Open in embedded mode"
                              title="Open in embedded mode"
                            >
                          </a>
                        </td>
                        <td class="contentCells contentCells-icon contentCells-shift downloadContentCellShift">
                          <a href="download?fileName={{ encodeURIComponent(storedFiles[i].name) }}">
                            <img
                              class="icon-download"
                              src="@/assets/images/download.svg"
                              alt="Download"
                              title="Download"
                            >
                          </a>
                        </td>
                        <td class="contentCells contentCells-icon contentCells-shift">
                          <a
                            class="delete-file"
                            data="{{ encodeURIComponent(storedFiles[i].name) }}"
                          >
                            <img
                              class="icon-delete"
                              src="@/assets/images/delete.svg"
                              alt="Delete"
                              title="Delete"
                            >
                          </a>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>

  <div id="mainProgress">
    <div id="uploadSteps">
      <span
        id="uploadFileName"
        class="uploadFileName"
      />
      <div class="describeUpload">
        After these steps are completed, you can work with your document.
      </div>
      <span
        id="step1"
        class="step"
      >1. Loading the file.</span>
      <span class="step-descr">The loading speed depends on file size and additional elements it contains.</span>
      <br>
      <span
        id="step2"
        class="step"
      >2. Conversion.</span>
      <span class="step-descr">The file is converted to OOXML so that you can edit it.</span>
      <br>
      <div id="blockPassword">
        <span class="descrFilePass">The file is password protected.</span>
        <br>
        <div>
          <input
            id="filePass"
            type="password"
          >
          <div
            id="enterPass"
            class="button orange"
          >
            Enter
          </div>
          <div
            id="skipPass"
            class="button gray"
          >
            Skip
          </div>
        </div>
        <span class="errorPass" />
        <br>
      </div>
      <span
        id="step3"
        class="step"
      >3. Loading editor scripts.</span>
      <span class="step-descr">They are loaded only once, they will be cached on your computer.</span>
      <input
        type="hidden"
        name="hiddenFileName"
        id="hiddenFileName"
      >
      <br>
      <br>
      <span class="progress-descr">Note the speed of all operations depends on your connection quality and server location.</span>
      <br>
      <br>
      <div class="error-message">
        <b>Upload error: </b><span />
        <br>
        Please select another file and try again.
      </div>
    </div>
    <br>
    <div
      id="beginEdit"
      class="button orange disable"
    >
      Edit
    </div>
    <div
      id="beginView"
      class="button gray disable"
    >
      View
    </div>
    <div
      id="beginEmbedded"
      class="button gray disable"
    >
      Embedded view
    </div>
    <div
      id="cancelEdit"
      class="button gray"
    >
      Cancel
    </div>
  </div>

  <span
    id="loadScripts"
    data-docs="{{ preloaderUrl }}"
  />

  <footer>
    <div class="center">
      <table>
        <tbody>
          <tr>
            <td>
              <a
                href="http://api.onlyoffice.com/editors/howitworks"
                target="_blank"
              >API Documentation</a>
            </td>
            <td>
              <a href="mailto:sales@onlyoffice.com">Submit your request</a>
            </td>
            <td class="copy">
              &copy; Ascensio Systems SIA 2021. All rights reserved.
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </footer>

  <!-- <script type="text/javascript" src="javascripts/jquery-1.8.2.js"></script>
  <script type="text/javascript" src="javascripts/jquery-ui.js"></script>
  <script type="text/javascript" src="javascripts/jquery.blockUI.js"></script>
  <script type="text/javascript" src="javascripts/jquery.iframe-transport.js"></script>
  <script type="text/javascript" src="javascripts/jquery.fileupload.js"></script>
  <script type="text/javascript" src="javascripts/jquery.dropdownToggle.js"></script>
  <script type="text/javascript" src="javascripts/jscript.js"></script> -->
</template>

<script setup lang="ts">
import { users as usersRaw} from '../states/onlyoffice/users';
import { docManager } from '../states/onlyoffice/docManager';
import { configServer } from '../states/onlyoffice/configServer';

const users = usersRaw;
const storedFiles: any[] = [];
// const storedFiles = docManager.getStoredFiles();
const fillExts = configServer.fillDocs;

// let ConverExtList = "{{ convertExts }}";
// let EditedExtList = "{{ editedExts }}";
// let FilledExtList = "{{ fillExts }}";
// let UrlConverter = "convert";
// let UrlEditor = "editor";

// docManager.init(storageFolder, req, res);

// preloaderUrl: siteUrl + configServer.get('preloaderUrl'),
// convertExts: configServer.get('convertedDocs'),
// editedExts: configServer.get('editedDocs'),
// fillExts: configServer.get("fillDocs"),
// storedFiles: docManager.getStoredFiles(),
// params: docManager.getCustomParams(),
// serverUrl: docManager.getServerUrl(),
</script>

<style>
html {
    height: 100%;
    width: 100%;
}

body {
    background: #FFFFFF;
    color: #333333;
    font-family: Open Sans;
    font-size: 12px;
    font-style: normal;
    font-weight: normal;
    height: 100%;
    margin: 0;
    overflow-y: overlay;
    padding: 0;
    text-decoration: none;
    overflow-x:hidden ;
}

form {
    height: 100%;
}

div {
    margin: 0;
    padding: 0;
}

a,
a:hover,
a:visited {
    color: #333333;
}

header {
    background: #333333;
    height: 48px;
    margin: 0 auto;
    min-width: 1152px;
    width: auto;
}

header img {
    margin: 10px 0 22px 32px;
}

.center {
    position: relative;
    margin: 0 auto 0;
    width: 1152px;
}

.main{
    display: table;
    height: calc(100% - 112px);
    min-height: 536px;
}

.table-main {
    border-spacing: 0;
    height: 100%;
    min-height: 536px;
}

.section{
    height: 100%;
    padding: 0;
    vertical-align: top;
}

.main-panel {
    box-sizing: border-box;
    -moz-box-sizing: border-box;
    height: 100%;
    list-style: none;
    padding: 48px 32px 24px;
    position: relative;
    width: 896px;
}

.portal-name {
    color: #FF6F3D;
    font-size: 24px;
    font-weight: bold;
    line-height: 133%;
    letter-spacing: -0.02em;
}

.portal-descr {
    display: inline-block;
    font-size: 16px;
    line-height: 160%;
    margin-top: 16px;
}

.header-list {
    font-weight: bold;
    font-size: 16px;
    line-height: 133%;
    letter-spacing: -0.02em;
}

label .checkbox {
    margin: 0 5px 3px 0;
    vertical-align: middle;
    cursor: pointer;
}

.try-editor-list {
    list-style: none;
    margin: 0;
    padding: 0;
}

.try-editor-list li {
    margin-bottom: 12px
}

.try-editor {
    background-color: transparent;
    background-repeat: no-repeat;
    display: block;
    font-size: 14px;
    line-height: 40px;
    padding-left: 42px;
    text-decoration: none;
}

.try-editor.word {
    background-image: url("@/assets/images/file_docx.svg");
}

.try-editor.cell {
    background-image: url("@/assets/images/file_xlsx.svg");
}

.try-editor.slide {
    background-image: url("@/assets/images/file_pptx.svg");
}

.try-editor.form {
    background-image: url("@/assets/images/file_docxf.svg");
}

.create-sample {
    color: #666666;
    line-height: 24px;
}

.button,
.button:visited,
.button:hover,
.button:active {
    align-items: center;
    border-radius: 3px;
    box-sizing: border-box;
    cursor:pointer;
    display: inline-block;
    font-weight: 600;
    letter-spacing: 0.08em;
    line-height: 133%;
    padding: 8px 20px;
    text-align: center;
    text-decoration: none;
    text-transform: uppercase;
    vertical-align: middle;
    user-select: none;
    -o-touch-callout: none;
    -moz-touch-callout: none;
    -webkit-touch-callout: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
}

.button.orange {
    background: #FF6F3D;
    border: 1px solid #FF6F3D;
    color: #FFFFFF;
}

.button.orange.disable {
    background: #EDC2B3;
    border: 1px solid #EDC2B3;
    cursor: default;
}

.button.orange:not(.disable):hover{
    background: #ff7a4b;
}

.button.gray {
    border: 1px solid #AAAAAA;
    margin-left: 8px;
}

.button.gray.disable {
    border: 1px solid #E5E5E5;
    color: #B5B5B5;
    cursor: default;
}

.button.gray:not(.disable):hover {
    border: 1px solid #FF6F3D;
    color: #FF6F3D;
}

.upload-panel {
    float: left;
    padding: 24px 0;
}

.file-upload {
    background: url(@/assets/images/file_upload.svg) no-repeat 0 transparent;
    cursor: pointer;
    display: block;
    font-size: 14px;
    line-height: 40px;
    overflow: hidden;
    padding-left: 42px;
    position: relative;
    width: 150px;
}

.file-upload input {
    cursor: pointer;
    height: 40px;
    margin: 0;
    opacity: 0;
    opacity: 0;
    position: absolute;
    right: 0;
    top: 0;
    transform: translate(0px, -21px) scale(2);
    width: 192px;
}

.create-panel,
.links-panel {
    float: left;
    padding: 16px 0;
}

.upload-panel,
.create-panel {
    width: 100%;
    border-bottom: 1px solid #D0D5DA;
}

.links-panel-border {
    margin-top: 24px;
    width: 100%;
    border-top: 1px solid #D0D5DA;
}

#mainProgress {
    color: #333333;
    display: none;
    font-size: 12px;
    margin: 30px 40px;
}

#mainProgress .uploadFileName{
    background-position: left center;
    background-repeat: no-repeat;
    display: block;
    font-size: 14px;
    line-height: 160%;
    overflow: hidden;
    padding-left: 28px;
    text-overflow: ellipsis;
    white-space: nowrap;
}

#mainProgress .describeUpload {
    line-height: 150%;
    letter-spacing: -0.02em;
    padding: 16px 0;
}

#mainProgress #embeddedView {
    display: none;
}

#mainProgress.embedded #embeddedView {
    display: block;
}

#mainProgress.embedded #uploadSteps {
    display: none;
}

.error-message {
    background: url(@/assets/images/error.svg) no-repeat scroll 4px 10px;
    color: #CB0000;
    display: none;
    line-height: 160%;
    letter-spacing: -0.02em;
    margin: 5px 0;
    padding: 10px 10px 10px 30px;
    vertical-align: middle;
    word-wrap: break-word;
}

.step {
    background-repeat: no-repeat;
    background-position: left center;
    background-color: transparent;
    font-weight: bold;
    line-height: 167%;
    padding-left: 35px;
}

.current {
    background-image: url("@/assets/images/loader16.gif");
}

.done {
    background-image: url("@/assets/images/done.svg");
}

.error {
    background-image: url("@/assets/images/notdone.svg");
}

.step-descr {
    display: block;
    margin-left: 35px;
    font-size: 11px;
    line-height: 188%;
}

.progress-descr {
    letter-spacing: -0.02em;
    line-height: 150%;
}

#loadScripts {
    display: none;
}

#iframeScripts {
    position: absolute;
    visibility: hidden;
}

footer {
    background: #333333;
    color: #AAAAAA;
    height: 64px;
    width: 100%;
    position: relative;
    left: 0;
    bottom: 0;
}
footer > .center{
    width: 100%;
}
footer table {
    width: 100%;
    border-spacing: 0;
}

footer table tr {
    position: relative;
    display: flex;
    flex-direction: row;
    align-items: center;
    align-content: center;
    flex-wrap: wrap;
    width: 100vw;
    height: 64px;
}

footer table td {
    display: block;
    position: relative;
    padding-left: 32px;
}

footer a,
footer a:hover,
footer a:visited {
    color: #FF6F3D;
    font-size: 14px;
    line-height: 120%;
}

footer a:hover {
    text-decoration: none;
}
footer table tr td:first-child {
    margin-left: 14%;
}
.copy {
    color: #aaaaaa;
    width: max-content;
    position: relative;
    margin-left: auto;
    margin-right: 14%;
}

.help-block {
    margin: 48px 32px 24px;
}

.help-block span {
    font-size: 14px;
    font-weight: 600;
    line-height: 19px;
}

.stored-list {
    display: block;
    list-style: none;
    padding: 0;
    position: relative;
    height: 100%;
}

.stored-edit {
    background-color: transparent;
    background-position: left center;
    background-repeat: no-repeat;
    display: inline-block;
    height: 16px;
    max-width: 250px;
    overflow: hidden;
    padding: 8px 0 1px 26px;
    text-decoration: none;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.stored-edit.word,
.uploadFileName.word {
    background-image: url("@/assets/images/icon_docx.svg");
}

.stored-edit.cell,
.uploadFileName.cell {
    background-image: url("@/assets/images/icon_xlsx.svg");
}

.stored-edit.slide,
.uploadFileName.slide {
    background-image: url("@/assets/images/icon_pptx.svg");
}

.stored-edit span {
    font-size: 12px;
    line-height: 12px;
}

.stored-edit:hover span {
    text-decoration: underline;
}

.blockTitle {
    background-color: #333333 !important;
    border: none !important;
    border-radius: 0 !important;
    -moz-border-radius: 0 !important;
    -webkit-border-radius: 0 !important;
    color: #F5F5F5 !important;
    font-size: 16px !important;
    font-weight: 600!important;
    line-height: 133%;
    letter-spacing: -0.02em;
    padding: 14px 16px 14px 46px !important;
}

.dialog-close {
    background: url(@/assets/images/close.svg) no-repeat scroll left top;
    cursor: pointer;
    float: right;
    font-size: 1px;
    height: 14px;
    line-height: 1px;
    margin-top: 4px;
    width: 14px;
}

.blockPage {
    border: none !important;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    -moz-box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    -webkit-box-shadow:0 2px 4px rgba(0, 0, 0, 0.5);
    padding: 0 !important;
}

.clearFix:after {
    content: ".";
    display: block;
    height: 0;
    clear: both;
    visibility: hidden;
}

.tableRow {
    background: transparent;
    -moz-transition: all 0.2s ease-in-out;
    -webkit-transition: all 0.2s ease-in-out;
    -o-transition: all 0.2s ease-in-out;
    -ms-transition: all 0.2s ease-in-out;
    transition: all 0.2s ease-in-out;
}

.tableRow:hover {
    background-color: #ECECEC;
}

.tableHeader {
    padding-top: 10px;
}

.tableHeader tr{
    background: transparent;
    cursor: default;
    height: 40px;
    -khtml-user-select: none;
    user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
}

.tableHeaderCell {
    border-bottom: 1px solid #CCCCCC;
    padding: 2px 4px;
    text-align: center;
}

.tableHeaderCellFileName {
    text-align: left;
    width: 37%;
}

.tableHeaderCellEditors{
    width: 29%;
}

.tableHeaderCellViewers{
    width: 11%;
}

.tableHeaderCellDownload{
    width: 13%;
    text-align: right;
    padding-right: 20px;
}

.tableHeaderCellRemove{
    text-align: left;
}

.contentCells {
    display: block;
    font-family: 'Open Sans', sans-serif;
    font-size: 16px;
    padding: 4px;
    white-space: nowrap;
    -khtml-user-select: none;
    user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
}

.contentCells-shift {
    padding-right: 44px;
}

.contentCells-icon {
    width: 4%;
}

.contentCells-wopi {
    height: 25px;
    width: 18%;
    text-align: left;
}

.contentCells-wopi a {
    text-decoration: none;
}

.select-user {
    color: #444444;
    font-family: Open Sans;
    font-size: 12px!important;
    font-weight: normal!important;
    line-height: 16px!important;
}

.info{
    cursor: pointer;
    margin: -2px 5px;
}

.user-block-table {
    height: 100%;
    padding-top: 14px;
    width: 100%;
}

.user-block-table td {
    background-color: #F4F4F4;
    padding-top: 10px;
}

.user-block-table td select {
    border: 1px solid #D0D5DA;
    box-sizing: border-box;
    border-radius: 3px;
    cursor: pointer;
    margin-top: 5px;
    padding: 2px 5px;
    width: 100%;
}

.icon-delete {
    cursor: pointer;
}

.left-panel {
    width: 256px;
    background: #F5F5F5;
}

.scroll-table-body {
    bottom: 0;
    left: 0;
    margin-top: 0px;
    overflow-x: auto;
    position: absolute;
    right: 0;
    top: 71px;
    scrollbar-color: #D0D5DA transparent;
    scrollbar-width: thin;
}

.scroll-table-body::-webkit-scrollbar {
    width: 4px;
}

.scroll-table-body::-webkit-scrollbar-thumb {
    background: #D0D5DA;
    border-radius: 3px;
}

.descrFilePass {
    display: block;
    font-weight: bold;
    line-height: 167%;
}

#filePass {
    border: 1px solid #D0D5DA;
    border-radius: 3px;
    box-sizing: border-box;
    display: inline-block;
    height: 33px;
    letter-spacing: -0.02em;
    line-height: 150%;
    margin-right: 8px;
    outline: none;
    padding: 7px 8px;
    vertical-align: bottom;
    user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    width: 250px;
}

.errorInput {
    border-color: #CB0000!important;
}

.errorPass {
    color: #CB0000;
    display: block;
    line-height: 160%;
    letter-spacing: -0.02em;
    word-wrap: break-word;
}

html {
    overflow-x: hidden;
}

.tableRow {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    position: relative;
}

.tableRow td:first-child {
    display: flex;
    flex-grow: 1;
    max-width: 25%;
}

.tableHeaderCellFileName {
    width: 30%;
}

.tableHeaderCellEditors {
    width: 28%;
}

.tableHeaderCellViewers {
    text-align: center;
    width: 18%
}

.firstContentCellViewers {
    margin-left: auto;
}
.tableHeaderCellDownload{
    padding-right: 28px;
}

.user-descr {
    display: inline-table;
    width: 30vw;
    min-width: 200px;
    max-width: 400px;
}

.user-descr > b {
    margin-left: 25px;
}

.portal-descr:nth-child(3) {
    margin-bottom: 20px;
}
</style>
