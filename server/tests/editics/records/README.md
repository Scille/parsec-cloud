# How to generate new tests

1. Start the OnlyOffice protocol monitor:

   ```shell
   cd ../../../../docs/rfcs/1030-collaborative-editics/oo-protocol-monitor/
   npm install
   npm run demo
   ```

   This opens a chromium window on the official OnlyOffice website's demo page.

2. From there you can run your session (have another user joining, edit the document etc.).

3. When you are done click on the copy" button in the "OO Protocol Monitor" panel.

4. Paste the result as a new file in this folder.

5. The session logs uses the wrong participant names and IDs, this must be manually changed by doing:

   ```shell
   python cook_record.py <x_my_new_record.md>
   ```
