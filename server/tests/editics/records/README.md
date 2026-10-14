# How to generate new tests

1. Start the OnlyOffice protocol monitor:

   ```shell
   cd ../../../../docs/rfcs/1030-collaborative-editics/oo-protocol-monitor/
   npm install
   npm run with-docker
   ```

   This opens a chromium window with multiple OnlyOffice editors and a panel
   capturing the events exchanged with the server.

2. From there you can run your session (have another user joining, edit the document etc.).

3. When you are done click on the copy" button in the "OO Protocol Monitor" panel.

4. Paste the result as a new file in this folder.
