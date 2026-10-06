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
   sed -i
    -e 's/John Smith/Alice/g'
    -e 's/Kate Cage/Bob/g'
    -e 's/F89d8069ba2b/de10a11cec0010000000000000000000/g'
    -e 's/78e1e841/de10808c001000000000000000000000/g'
    <x_my_new_record.md>
   ```
