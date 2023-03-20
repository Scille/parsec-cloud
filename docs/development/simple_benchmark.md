# Simple benchmark parsec

1. Download <https://github.com/Scille/parsec-cloud/archive/refs/tags/v2.15.0.zip>.

2. Extract the zip file.
    > You can also build `parsec-2.15` to generate more files (I think of the `target` folder)

3. On the **main** branch and on **your** branch
   1. Compile parsec in release mode.
   2. Run

      ```shell
      source tests/script/run_testenv.sh
      ```

   3. Follow its indication to connect as `alice` with the CLI commands.
   4. On another shell

      ```shell
      time cp -R <path where you extracted parsec-2.15> ~/Parsec/alice/parsec-2.15
      ```

   5. Save the taken time.
   6. On the shell that run `testenv` execute the function `cleanup` to stop it.

4. Compare both time
