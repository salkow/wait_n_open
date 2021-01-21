# wait_n_open

## About
This program runs the provided PBS script and waits until the output file has been created.
With this you won't need to check, if the program has finished, every couple of seconds.

## Usage
```sh
wait_n_open [FLAGS] --editor <editor> --job-name <job-name> --pbs-name <pbs-name>
```
Example - `wait_n_open -c -e vim -j 1680_mpi -p myPBSScript.sh`

## FLAGS:
```
    -c, --check-error : Opens the error file, if it isn't empty
    -h, --help : Prints help information
    -V, --version : Prints version information
```

## OPTIONS:
```
    -e, --editor <editor> : Name of the editor [env: EDITOR]
    -j, --job-name <job-name> : Name of the job, from the PBS script
    -p, --pbs-name <pbs-name> : Name of the PBS script
```
